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

    UNIQUE(name, parent_id),
    CHECK(id != parent_id)
);
 -- Insert the five root accounts
  INSERT INTO accounts (name, account_type, parent_id, currency, description, is_active) VALUES
    ('Assets', 'asset', NULL, 'EUR', 'All asset accounts', 1),
    ('Liabilities', 'liability', NULL, 'EUR', 'All liability accounts', 1),
    ('Equity', 'equity', NULL, 'EUR', 'Owner''s equity accounts', 1),
    ('Income', 'income', NULL, 'EUR', 'All income accounts', 1),
    ('Expenses', 'expense', NULL, 'EUR', 'All expense accounts', 1);


CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    reference TEXT,
    transaction_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags TEXT,
    notes TEXT
);

CREATE TABLE transaction_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    amount_minor INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'EUR',
    entry_type TEXT NOT NULL CHECK (entry_type IN ('debit', 'credit')),
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_accounts_parent_id ON accounts(parent_id);
CREATE INDEX idx_accounts_type ON accounts(account_type);
CREATE INDEX idx_transactions_date ON transactions(transaction_date);
CREATE INDEX idx_entries_transaction ON transaction_entries(transaction_id);
CREATE INDEX idx_entries_account ON transaction_entries(account_id);

