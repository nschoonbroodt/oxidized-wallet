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

