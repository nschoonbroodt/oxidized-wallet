# Database Schema Design - Oxidized Wallet

## Overview
This document defines the SQLite database schema for the double-entry bookkeeping system. The schema is designed to enforce accounting integrity while supporting hierarchical accounts and future multi-currency operations.

## Core Principles

### Double-Entry Bookkeeping Rules
1. **Every transaction affects at least two accounts**
2. **Total debits must equal total credits for each transaction**
3. **Account balances are calculated, not stored** (except for performance optimization)
4. **Transactions are immutable once created** (audit trail)

### Account Types and Normal Balances
- **Assets** (Debit normal): Bank accounts, cash, investments
- **Liabilities** (Credit normal): Loans, credit cards
- **Equity** (Credit normal): Initial capital, retained earnings
- **Income** (Credit normal): Salary, investment returns
- **Expenses** (Debit normal): Food, transport, utilities

## Schema Definition

### accounts Table
```sql
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    account_type TEXT NOT NULL CHECK (account_type IN ('asset', 'liability', 'equity', 'income', 'expense')),
    parent_id INTEGER REFERENCES accounts(id),
    currency TEXT NOT NULL DEFAULT 'EUR',
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Ensure unique names within the same parent
    UNIQUE(name, parent_id),
    
    -- Prevent circular references
    CHECK (id != parent_id)
);

-- Index for hierarchy queries
CREATE INDEX idx_accounts_parent_id ON accounts(parent_id);
CREATE INDEX idx_accounts_type ON accounts(account_type);
```

**Design Notes**:
- `parent_id` enables hierarchical accounts (BoursoBank -> Compte courant)
- `currency` field ready for multi-currency support
- `is_active` for soft deletion (preserve historical data)
- Unique constraint on name+parent prevents duplicates at same level

### transactions Table
```sql
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    reference TEXT, -- External reference (bank ref, invoice number)
    transaction_date DATE NOT NULL, -- NaiveDate (date only, no time/timezone)
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Metadata
    tags TEXT, -- JSON array of tags for categorization
    notes TEXT
);

CREATE INDEX idx_transactions_date ON transactions(transaction_date);
CREATE INDEX idx_transactions_created ON transactions(created_at);
```

**Design Notes**:
- Transactions are immutable (no updated_at field)
- `reference` for bank imports or external document references
- `tags` as JSON for flexible categorization

### transaction_entries Table
```sql
CREATE TABLE transaction_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    amount_minor INTEGER NOT NULL, -- Amount in smallest currency unit
    currency TEXT NOT NULL DEFAULT 'EUR',
    entry_type TEXT NOT NULL CHECK (entry_type IN ('debit', 'credit')),
    description TEXT, -- Entry-specific description
    
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_entries_transaction ON transaction_entries(transaction_id);
CREATE INDEX idx_entries_account ON transaction_entries(account_id);
CREATE INDEX idx_entries_amount ON transaction_entries(amount_minor);
```

**Design Notes**:
- `amount_minor` stores money as integers in smallest currency unit (avoid floating-point errors)
- Each entry explicitly marked as debit or credit
- Cascade delete maintains referential integrity
- `transaction_date` uses NaiveDate (date only, no time/timezone)

### Constraints and Triggers

#### Double-Entry Balance Constraint
```sql
-- Trigger to enforce double-entry balance
CREATE TRIGGER enforce_double_entry_balance
AFTER INSERT ON transaction_entries
FOR EACH ROW
BEGIN
    -- Check if transaction is balanced after this insert
    SELECT CASE 
        WHEN (
            SELECT SUM(CASE WHEN entry_type = 'debit' THEN amount_minor ELSE -amount_minor END)
            FROM transaction_entries 
            WHERE transaction_id = NEW.transaction_id
        ) != 0
        THEN RAISE(ABORT, 'Transaction must be balanced: debits must equal credits')
    END;
END;
```

#### Account Hierarchy Depth Limit
```sql
-- Prevent excessive nesting (max 5 levels)
CREATE TRIGGER check_account_depth
BEFORE INSERT ON accounts
FOR EACH ROW
WHEN NEW.parent_id IS NOT NULL
BEGIN
    SELECT CASE 
        WHEN (
            WITH RECURSIVE account_path(id, level) AS (
                SELECT NEW.parent_id, 1
                UNION ALL
                SELECT a.parent_id, ap.level + 1
                FROM accounts a
                JOIN account_path ap ON a.id = ap.id
                WHERE a.parent_id IS NOT NULL AND ap.level < 5
            )
            SELECT MAX(level) FROM account_path
        ) >= 5
        THEN RAISE(ABORT, 'Account hierarchy too deep (max 5 levels)')
    END;
END;
```

## Example Account Hierarchy

```
Assets
├── BoursoBank
│   ├── Compte courant
│   ├── Compte d'épargne
│   └── PEA
└── SG
    ├── Livret A
    └── Compte courant

Expenses
├── Transport
│   ├── Voiture
│   │   ├── Essence
│   │   └── Assurance
│   └── Transport public
└── Alimentation
    ├── Courses
    └── Restaurants

Income
├── Salaire
└── Investissements
    ├── Dividendes
    └── Plus-values
```

## Queries and Business Logic

### Account Balance Calculation
```sql
-- Calculate account balance (including child accounts)
WITH RECURSIVE account_tree AS (
    SELECT id FROM accounts WHERE id = ?
    UNION ALL
    SELECT a.id FROM accounts a
    JOIN account_tree at ON a.parent_id = at.id
)
SELECT 
    SUM(CASE 
        WHEN te.entry_type = 'debit' AND a.account_type IN ('asset', 'expense') THEN te.amount_minor
        WHEN te.entry_type = 'credit' AND a.account_type IN ('liability', 'equity', 'income') THEN te.amount_minor
        WHEN te.entry_type = 'debit' AND a.account_type IN ('liability', 'equity', 'income') THEN -te.amount_minor
        WHEN te.entry_type = 'credit' AND a.account_type IN ('asset', 'expense') THEN -te.amount_minor
        ELSE 0
    END) as balance_minor
FROM transaction_entries te
JOIN accounts a ON te.account_id = a.id
WHERE a.id IN (SELECT id FROM account_tree);
```

### Monthly Income/Expense Summary
```sql
SELECT 
    strftime('%Y-%m', t.transaction_date) as month,
    a.account_type,
    SUM(te.amount_minor) as total_minor
FROM transactions t
JOIN transaction_entries te ON t.id = te.transaction_id
JOIN accounts a ON te.account_id = a.id
WHERE a.account_type IN ('income', 'expense')
    AND t.transaction_date >= date('now', '-12 months')
GROUP BY month, a.account_type
ORDER BY month DESC, a.account_type;
```

## Migration Strategy

### Initial Setup
1. Create tables in dependency order
2. Create indexes and triggers
3. Insert basic account structure
4. Insert opening balances as initial transactions

### Future Migrations
- Add columns with DEFAULT values for backward compatibility
- Use ALTER TABLE for schema changes
- Migration files numbered sequentially: `001_initial.sql`, `002_add_tags.sql`

## Performance Considerations

### Indexes
- Primary keys on all tables
- Foreign key columns indexed
- Date columns for time-based queries
- Account hierarchy queries optimized

### Materialized Views (Future)
For performance with large datasets:
```sql
-- Account balances cache (refresh periodically)
CREATE TABLE account_balances_cache (
    account_id INTEGER PRIMARY KEY,
    balance_minor INTEGER NOT NULL,
    last_updated TIMESTAMP NOT NULL
);
```

## Data Integrity Rules

1. **Immutable Transactions**: Once created, transactions cannot be modified
2. **Balanced Entries**: Every transaction must have equal debits and credits
3. **Valid Accounts**: All entries must reference existing, active accounts
4. **Currency Consistency**: All entries in a transaction must use same currency (MVP)
5. **Positive Amounts**: Entry amounts must be positive (sign determined by debit/credit)

This schema provides a robust foundation for double-entry bookkeeping while maintaining flexibility for future enhancements.