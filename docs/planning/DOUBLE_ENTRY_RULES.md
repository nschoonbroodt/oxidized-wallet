# Double-Entry Bookkeeping Rules and Implementation

## Overview
This document defines the double-entry bookkeeping rules and their implementation in Oxidized Wallet. Double-entry bookkeeping is the foundation of accurate financial tracking, ensuring that every transaction maintains the fundamental accounting equation: **Assets = Liabilities + Equity**.

## Fundamental Principles

### The Accounting Equation
```
Assets = Liabilities + Equity
```

This equation must always remain balanced. Every transaction affects at least two accounts and maintains this balance.

### The Golden Rule of Double-Entry
**"For every debit, there must be an equal and opposite credit"**

- **Debits** are recorded on the left side
- **Credits** are recorded on the right side  
- Total debits must equal total credits for each transaction

## Account Types and Normal Balances

### Asset Accounts (Debit Normal Balance)
**Examples**: Bank accounts, cash, investments, property
- **Increase with**: Debits (left side)
- **Decrease with**: Credits (right side)
- **French examples**: Compte courant, Livret A, PEA

```
Asset Account (e.g., Compte Courant BoursoBank)
Debit (↑)  |  Credit (↓)
   +500    |    -200
   +100    |
-----------|-----------
   Balance: 400 EUR (Debit)
```

### Liability Accounts (Credit Normal Balance)
**Examples**: Loans, credit cards, mortgages
- **Increase with**: Credits (right side)
- **Decrease with**: Debits (left side)
- **French examples**: Prêt immobilier, Carte de crédit

```
Liability Account (e.g., Prêt Immobilier)
Debit (↓)  |  Credit (↑)
   -100    |    +1000
           |    +500
-----------|-----------
   Balance: 1400 EUR (Credit)
```

### Equity Accounts (Credit Normal Balance)
**Examples**: Owner's capital, retained earnings
- **Increase with**: Credits (right side)
- **Decrease with**: Debits (left side)

### Income Accounts (Credit Normal Balance)
**Examples**: Salary, investment returns, freelance income
- **Increase with**: Credits (right side)
- **Decrease with**: Debits (left side)
- **French examples**: Salaire, Dividendes, Freelance

```
Income Account (e.g., Salaire)
Debit (↓)  |  Credit (↑)
           |    +2500
           |    +2500
-----------|-----------
   Balance: 5000 EUR (Credit)
```

### Expense Accounts (Debit Normal Balance)
**Examples**: Food, transport, utilities, insurance
- **Increase with**: Debits (left side)
- **Decrease with**: Credits (right side)
- **French examples**: Alimentation, Transport, Assurance

```
Expense Account (e.g., Transport → Essence)
Debit (↑)  |  Credit (↓)
   +60     |    
   +45     |    
-----------|-----------
   Balance: 105 EUR (Debit)
```

## Transaction Examples

### Example 1: Salary Receipt
**Scenario**: Receiving 2500 EUR salary into BoursoBank checking account

```
Date: 2025-01-31
Description: Salaire janvier 2025

Entries:
- Debit:  Assets → BoursoBank → Compte courant    +2500 EUR
- Credit: Income → Salaire                        +2500 EUR

Verification: 2500 EUR debit = 2500 EUR credit ✓
```

### Example 2: Grocery Shopping
**Scenario**: Spending 65 EUR on groceries, paid with checking account

```
Date: 2025-02-01
Description: Courses Carrefour

Entries:
- Debit:  Expenses → Alimentation → Courses       +65 EUR
- Credit: Assets → BoursoBank → Compte courant    -65 EUR

Verification: 65 EUR debit = 65 EUR credit ✓
```

### Example 3: Fuel Purchase
**Scenario**: 58 EUR fuel for car, paid with credit card

```
Date: 2025-02-02
Description: Essence Total

Entries:
- Debit:  Expenses → Transport → Voiture → Essence  +58 EUR
- Credit: Liabilities → Carte de crédit             +58 EUR

Verification: 58 EUR debit = 58 EUR credit ✓
```

### Example 4: Savings Transfer
**Scenario**: Transfer 500 EUR from checking to savings account

```
Date: 2025-02-03
Description: Épargne mensuelle

Entries:
- Debit:  Assets → BoursoBank → Compte d'épargne   +500 EUR
- Credit: Assets → BoursoBank → Compte courant     -500 EUR

Verification: 500 EUR debit = 500 EUR credit ✓
Note: Both accounts are assets, so this is asset reallocation
```

### Example 5: Credit Card Payment
**Scenario**: Paying 200 EUR credit card balance from checking account

```
Date: 2025-02-15
Description: Remboursement carte de crédit

Entries:
- Debit:  Liabilities → Carte de crédit            -200 EUR
- Credit: Assets → BoursoBank → Compte courant     -200 EUR

Verification: 200 EUR debit = 200 EUR credit ✓
```

## Implementation Rules in Code

### Database Constraints
```sql
-- Ensure transaction balance (implemented as trigger)
CREATE TRIGGER enforce_double_entry_balance
AFTER INSERT ON transaction_entries
FOR EACH ROW
BEGIN
    SELECT CASE 
        WHEN (
            SELECT SUM(CASE WHEN entry_type = 'debit' THEN amount_cents ELSE -amount_cents END)
            FROM transaction_entries 
            WHERE transaction_id = NEW.transaction_id
        ) != 0
        THEN RAISE(ABORT, 'Transaction must be balanced: debits must equal credits')
    END;
END;
```

### Rust Validation Logic
```rust
pub struct TransactionValidator;

impl TransactionValidator {
    pub fn validate_balance(entries: &[TransactionEntry]) -> Result<(), ValidationError> {
        let total_debits: i64 = entries
            .iter()
            .filter(|e| e.entry_type == EntryType::Debit)
            .map(|e| e.amount_cents)
            .sum();
            
        let total_credits: i64 = entries
            .iter()
            .filter(|e| e.entry_type == EntryType::Credit)
            .map(|e| e.amount_cents)
            .sum();
            
        if total_debits != total_credits {
            return Err(ValidationError::UnbalancedTransaction {
                debits: total_debits,
                credits: total_credits,
            });
        }
        
        Ok(())
    }
    
    pub fn validate_minimum_entries(entries: &[TransactionEntry]) -> Result<(), ValidationError> {
        if entries.len() < 2 {
            return Err(ValidationError::InsufficientEntries);
        }
        Ok(())
    }
}
```

### Balance Calculation Logic
```rust
impl Account {
    pub fn calculate_balance(&self, entries: &[TransactionEntry]) -> Money {
        let balance_cents = entries
            .iter()
            .map(|entry| {
                match (entry.entry_type, self.account_type) {
                    // Normal balance increases
                    (EntryType::Debit, AccountType::Asset | AccountType::Expense) => entry.amount_cents,
                    (EntryType::Credit, AccountType::Liability | AccountType::Equity | AccountType::Income) => entry.amount_cents,
                    // Normal balance decreases
                    (EntryType::Credit, AccountType::Asset | AccountType::Expense) => -entry.amount_cents,
                    (EntryType::Debit, AccountType::Liability | AccountType::Equity | AccountType::Income) => -entry.amount_cents,
                }
            })
            .sum();
            
        Money::from_cents(balance_cents, self.currency)
    }
}
```

## French-Specific Considerations

### Common Account Structure
```
Assets
├── BoursoBank
│   ├── Compte courant (Checking)
│   ├── Compte d'épargne (Savings)
│   └── PEA (Investment account)
├── Société Générale  
│   ├── Livret A (Tax-free savings)
│   └── Compte courant
└── Liquide (Cash)

Liabilities
├── Prêt immobilier (Mortgage)
├── Carte de crédit (Credit card)
└── Découvert autorisé (Overdraft)

Income
├── Salaire (Salary)
├── Freelance
├── Dividendes (Dividends)
└── Intérêts (Interest)

Expenses
├── Logement (Housing)
│   ├── Loyer (Rent)
│   ├── Charges (Utilities)
│   └── Assurance habitation (Home insurance)
├── Transport
│   ├── Voiture (Car)
│   │   ├── Essence (Fuel)
│   │   ├── Assurance (Insurance)
│   │   └── Entretien (Maintenance)
│   └── Transport public
├── Alimentation (Food)
│   ├── Courses (Groceries)
│   └── Restaurants
└── Santé (Health)
    ├── Médecin (Doctor)
    └── Pharmacie (Pharmacy)
```

## Error Handling

### Common Validation Errors
```rust
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Transaction must have at least 2 entries")]
    InsufficientEntries,
    
    #[error("Transaction is not balanced: debits {debits}¢ ≠ credits {credits}¢")]
    UnbalancedTransaction { debits: i64, credits: i64 },
    
    #[error("Entry amount must be positive: {amount}¢")]
    NegativeAmount { amount: i64 },
    
    #[error("All entries must use the same currency in MVP")]
    MixedCurrencies,
    
    #[error("Account {account_id} does not exist or is inactive")]
    InvalidAccount { account_id: i64 },
}
```

## Testing Strategy

### Property-Based Tests
```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn transaction_always_balances(entries in valid_transaction_entries()) {
            let result = TransactionValidator::validate_balance(&entries);
            prop_assert!(result.is_ok());
        }
        
        #[test]
        fn account_balance_reflects_all_entries(
            account in any::<Account>(),
            entries in prop::collection::vec(transaction_entry(), 1..100)
        ) {
            let balance = account.calculate_balance(&entries);
            // Balance should equal sum of properly signed entries
            prop_assert_eq!(balance.cents(), expected_balance(&account, &entries));
        }
    }
}
```

## Monthly Closing Process

### Balance Verification
At month-end, verify the accounting equation:
```rust
pub fn verify_accounting_equation(db: &Database) -> Result<bool, DatabaseError> {
    let total_assets = calculate_total_assets(db)?;
    let total_liabilities = calculate_total_liabilities(db)?;
    let total_equity = calculate_total_equity(db)?;
    
    Ok(total_assets == total_liabilities + total_equity)
}
```

### Trial Balance Report
Generate trial balance to verify all accounts:
```rust
pub struct TrialBalanceEntry {
    pub account_name: String,
    pub debit_balance: Option<Money>,
    pub credit_balance: Option<Money>,
}

pub fn generate_trial_balance(db: &Database) -> Result<Vec<TrialBalanceEntry>, DatabaseError> {
    // Implementation returns all accounts with their balances
    // Total debits must equal total credits
}
```

This implementation ensures strict adherence to double-entry bookkeeping principles while providing clear validation and error handling for the French personal finance context.