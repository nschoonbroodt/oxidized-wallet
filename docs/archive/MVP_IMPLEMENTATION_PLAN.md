# MVP Implementation Plan - Oxidized Wallet

## Overview
This document provides a detailed step-by-step plan for implementing the MVP (Phase 1) of Oxidized Wallet. Each step includes technical details, dependencies, and acceptance criteria.

## Prerequisites
- Rust toolchain installed (latest stable) ✅
- Node.js and npm for Vue 3 frontend ✅
- Git repository initialized ✅
- All planning documentation complete ✅

## Implementation Steps

### Step 1: Set up Rust Workspace ✅
**Estimated Time**: 30 minutes  
**Dependencies**: None

#### Tasks:
1. **Create workspace Cargo.toml** ✅
```toml
[workspace]
members = ["wallet-core", "wallet-tauri"]

[workspace.package]
version = "0.1.0"
authors = ["Nicolas Schoonbroodt"]
edition = "2021"
license = "MIT"

[workspace.dependencies]
# Shared dependencies
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = { version = "1.34", features = ["serde"] }
```

2. **Create wallet-core library** ✅
```bash
cargo new --lib wallet-core --vcs none
```

3. **Initialize basic project structure** ✅
```
wallet-core/src/
├── lib.rs
├── models/
│   └── mod.rs
├── services/
│   └── mod.rs
├── db/
│   └── mod.rs
└── errors/
    └── mod.rs
```

#### Acceptance Criteria:
- [x] Workspace compiles with `cargo check`
- [x] Both crates are recognized by workspace
- [x] Basic module structure in place

---

### Step 2: Implement Core Money and Account Types ✅
**Estimated Time**: 2 hours  
**Dependencies**: Step 1

#### Tasks:
1. **Create Money type with proper decimal handling** ✅
```rust
// wallet-core/src/models/money.rs
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub code: String,
    pub minor_unit_scale: u8,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    pub amount_minor: i64,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        let scale_factor = 10_i64.pow(currency.minor_unit_scale as u32);
        let amount_minor = (amount * Decimal::from(scale_factor)).round().to_i64().unwrap();
        
        Self { amount_minor, currency }
    }
    
    pub fn to_decimal(&self) -> Decimal {
        let scale_factor = 10_i64.pow(self.currency.minor_unit_scale as u32);
        Decimal::from(self.amount_minor) / Decimal::from(scale_factor)
    }
    
    pub fn zero(currency: Currency) -> Self {
        Self { amount_minor: 0, currency }
    }
}
```

2. **Create Account types** ✅
```rust
// wallet-core/src/models/account.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Income,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<i64>,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<i64>,
    pub currency: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

3. **Create Transaction types** ✅
```rust
// wallet-core/src/models/transaction.rs
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntryType {
    Debit,
    Credit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEntry {
    pub id: Option<i64>,
    pub transaction_id: i64,
    pub account_id: i64,
    pub amount_minor: i64,
    pub currency: String,
    pub entry_type: EntryType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<i64>,
    pub description: String,
    pub reference: Option<String>,
    pub transaction_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub tags: Option<String>, // JSON
    pub notes: Option<String>,
    pub entries: Vec<TransactionEntry>,
}
```

#### Acceptance Criteria:
- [x] Money type handles different currencies correctly
- [x] Decimal conversion is accurate
- [x] Account hierarchy can be represented
- [x] Transaction with entries compiles
- [x] All types are serializable

---

### Step 3: Set up SQLite with Migrations ✅
**Estimated Time**: 1.5 hours  
**Dependencies**: Step 2

#### Tasks:
1. **Add sqlx dependencies to wallet-core** ✅
```toml
# wallet-core/Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "chrono", "migrate"] }
tokio = { version = "1", features = ["full"] }
```

2. **Create initial migration**
```sql
-- wallet-core/migrations/001_initial_schema.sql ✅
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
    CHECK (id != parent_id)
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

-- Indexes
CREATE INDEX idx_accounts_parent_id ON accounts(parent_id);
CREATE INDEX idx_accounts_type ON accounts(account_type);
CREATE INDEX idx_transactions_date ON transactions(transaction_date);
CREATE INDEX idx_entries_transaction ON transaction_entries(transaction_id);
CREATE INDEX idx_entries_account ON transaction_entries(account_id);
```

3. **Create database connection module** ✅
```rust
// wallet-core/src/db/connection.rs
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // Create database file if it doesn't exist
        if let Some(parent) = Path::new(database_url.trim_start_matches("sqlite:")).parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
            
        Ok(Database { pool })
    }
    
    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }
}
```

#### Acceptance Criteria:
- [x] Database file is created automatically
- [x] Migrations run successfully
- [x] All tables and indexes are created
- [x] Foreign key constraints work
- [x] Connection pool is established

---

### Step 4: Implement Account Management Logic ✅
**Estimated Time**: 2.5 hours  
**Dependencies**: Step 3

#### Tasks:
1. **Create account repository** ✅
```rust
// wallet-core/src/db/accounts.rs
use crate::models::{Account, AccountType};
use sqlx::SqlitePool;
use chrono::Utc;

pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn create(&self, account: &Account) -> Result<Account, sqlx::Error> {
        let account_type_str = match account.account_type {
            AccountType::Asset => "asset",
            AccountType::Liability => "liability",
            AccountType::Equity => "equity",
            AccountType::Income => "income",
            AccountType::Expense => "expense",
        };
        
        let id = sqlx::query!(
            r#"
            INSERT INTO accounts (name, account_type, parent_id, currency, description, is_active)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            account.name,
            account_type_str,
            account.parent_id,
            account.currency,
            account.description,
            account.is_active
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        
        self.get_by_id(id).await
    }
    
    pub async fn get_by_id(&self, id: i64) -> Result<Account, sqlx::Error> {
        // Implementation details...
    }
    
    pub async fn get_hierarchy(&self) -> Result<Vec<Account>, sqlx::Error> {
        // Get all accounts and let the service layer organize hierarchy
        // Implementation details...
    }
}
```

2. **Create account service with business logic** ✅
```rust
// wallet-core/src/services/account_service.rs
use crate::db::AccountRepository;
use crate::models::{Account, Money};
use crate::errors::WalletError;

pub struct AccountService {
    repository: AccountRepository,
}

impl AccountService {
    pub fn new(repository: AccountRepository) -> Self {
        Self { repository }
    }
    
    pub async fn create_account(
        &self,
        name: String,
        account_type: AccountType,
        parent_id: Option<i64>,
        currency: String,
    ) -> Result<Account, WalletError> {
        // Validate parent exists if specified
        if let Some(parent_id) = parent_id {
            self.validate_parent_exists(parent_id).await?;
            self.validate_hierarchy_depth(parent_id).await?;
        }
        
        // Create account
        let account = Account {
            id: None,
            name,
            account_type,
            parent_id,
            currency,
            description: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.repository.create(&account).await
            .map_err(WalletError::Database)
    }
    
    pub async fn calculate_balance(&self, account_id: i64) -> Result<Money, WalletError> {
        // Implementation to calculate balance from transaction entries
        // Including child account balances
        todo!()
    }
    
    async fn validate_hierarchy_depth(&self, parent_id: i64) -> Result<(), WalletError> {
        // Ensure we don't exceed 5 levels
        todo!()
    }
}
```

#### Acceptance Criteria:
- [x] Can create accounts with proper validation
- [x] Hierarchy depth is limited to 5 levels
- [x] Unique name constraint within parent works
- [x] Can retrieve account hierarchy
- [x] Account balance calculation stub is ready

---

### Step 5: Add Transaction Logic with Double-Entry Validation
**Estimated Time**: 3 hours  
**Dependencies**: Step 4

#### Tasks:
1. **Create transaction repository**
```rust
// wallet-core/src/db/transactions.rs
use crate::models::{Transaction, TransactionEntry};
use sqlx::SqlitePool;

pub struct TransactionRepository {
    pool: SqlitePool,
}

impl TransactionRepository {
    pub async fn create_transaction(&self, transaction: &Transaction) -> Result<Transaction, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        // Insert transaction
        let transaction_id = sqlx::query!(
            "INSERT INTO transactions (description, reference, transaction_date, tags, notes) VALUES (?1, ?2, ?3, ?4, ?5)",
            transaction.description,
            transaction.reference,
            transaction.transaction_date,
            transaction.tags,
            transaction.notes
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();
        
        // Insert entries
        for entry in &transaction.entries {
            let entry_type_str = match entry.entry_type {
                EntryType::Debit => "debit",
                EntryType::Credit => "credit",
            };
            
            sqlx::query!(
                "INSERT INTO transaction_entries (transaction_id, account_id, amount_minor, currency, entry_type, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                transaction_id,
                entry.account_id,
                entry.amount_minor,
                entry.currency,
                entry_type_str,
                entry.description
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        self.get_by_id(transaction_id).await
    }
}
```

2. **Create transaction validator**
```rust
// wallet-core/src/services/validation.rs
use crate::models::{TransactionEntry, EntryType};
use crate::errors::WalletError;

pub struct TransactionValidator;

impl TransactionValidator {
    pub fn validate_double_entry(entries: &[TransactionEntry]) -> Result<(), WalletError> {
        if entries.len() < 2 {
            return Err(WalletError::InsufficientEntries);
        }
        
        let total_debits: i64 = entries
            .iter()
            .filter(|e| e.entry_type == EntryType::Debit)
            .map(|e| e.amount_minor)
            .sum();
            
        let total_credits: i64 = entries
            .iter()
            .filter(|e| e.entry_type == EntryType::Credit)
            .map(|e| e.amount_minor)
            .sum();
        
        if total_debits != total_credits {
            return Err(WalletError::UnbalancedTransaction {
                debits: total_debits,
                credits: total_credits,
            });
        }
        
        // Validate all amounts are positive
        for entry in entries {
            if entry.amount_minor <= 0 {
                return Err(WalletError::NegativeAmount { 
                    amount: entry.amount_minor 
                });
            }
        }
        
        Ok(())
    }
}
```

3. **Create error types**
```rust
// wallet-core/src/errors/mod.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Transaction must have at least 2 entries")]
    InsufficientEntries,
    
    #[error("Transaction is not balanced: debits {debits} ≠ credits {credits}")]
    UnbalancedTransaction { debits: i64, credits: i64 },
    
    #[error("Entry amount must be positive: {amount}")]
    NegativeAmount { amount: i64 },
    
    #[error("Account {account_id} does not exist or is inactive")]
    InvalidAccount { account_id: i64 },
    
    #[error("Account hierarchy too deep (max 5 levels)")]
    HierarchyTooDeep,
}
```

#### Acceptance Criteria:
- [x] Can create transactions with multiple entries
- [x] Double-entry validation works correctly
- [x] Database transactions ensure consistency
- [x] All validation errors are properly typed
- [x] Comprehensive test coverage for validation

---

### Step 6: Create Tauri App with Vue 3 ✅
**Estimated Time**: 1.5 hours  
**Dependencies**: Step 5

#### Tasks:
1. **Initialize Tauri project** ✅
```bash
cargo install create-tauri-app
cargo create-tauri-app wallet-tauri --template vue-ts
```

2. **Configure Tauri for our needs** ✅
```json
// wallet-tauri/src-tauri/tauri.conf.json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist",
    "withGlobalTauri": false
  },
  "package": {
    "productName": "Oxidized Wallet",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.oxidized-wallet.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "Oxidized Wallet",
        "width": 1200,
        "height": 800
      }
    ]
  }
}
```

3. **Add wallet-core dependency to Tauri** ✅
```toml
# wallet-tauri/src-tauri/Cargo.toml
[dependencies]
wallet-core = { path = "../../wallet-core" }
tauri = { version = "1.0", features = ["api-all"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### Acceptance Criteria:
- [x] Tauri app starts successfully
- [x] Vue 3 frontend loads correctly
- [x] wallet-core can be imported in Tauri backend
- [x] Basic window configuration works
- [x] Hot reload works for development

---

### Step 7: Build Basic UI Components
**Estimated Time**: 4 hours  
**Dependencies**: Step 6

#### Tasks:
1. **Set up Vue 3 with TypeScript and styling**
```bash
cd wallet-tauri && npm install @headlessui/vue @heroicons/vue tailwindcss
```

2. **Create account management components**
```vue
<!-- src/components/accounts/AccountTree.vue -->
<template>
  <div class="account-tree">
    <h2>Comptes</h2>
    <div v-for="account in accounts" :key="account.id" class="account-item">
      <div @click="selectAccount(account)" 
           :class="['account-row', { selected: selectedAccount?.id === account.id }]">
        <span>{{ account.name }}</span>
        <span class="account-balance">{{ formatBalance(account.balance) }}</span>
      </div>
      <AccountTree v-if="account.children" 
                   :accounts="account.children" 
                   :level="level + 1" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Account } from '../../types/accounts'

interface Props {
  accounts: Account[]
  level?: number
}

const props = withDefaults(defineProps<Props>(), {
  level: 0
})

const selectedAccount = ref<Account | null>(null)

const selectAccount = (account: Account) => {
  selectedAccount.value = account
}

const formatBalance = (balance: number) => {
  return new Intl.NumberFormat('fr-FR', {
    style: 'currency',
    currency: 'EUR'
  }).format(balance / 100)
}
</script>
```

3. **Create transaction entry form**
```vue
<!-- src/components/transactions/TransactionForm.vue -->
<template>
  <form @submit.prevent="submitTransaction" class="transaction-form">
    <h2>Nouvelle Transaction</h2>
    
    <div class="form-group">
      <label for="description">Description</label>
      <input 
        id="description"
        v-model="transaction.description" 
        type="text" 
        required 
      />
    </div>
    
    <div class="form-group">
      <label for="date">Date</label>
      <input 
        id="date"
        v-model="transaction.date" 
        type="date" 
        required 
      />
    </div>
    
    <div class="entries-section">
      <h3>Écritures</h3>
      <div v-for="(entry, index) in transaction.entries" :key="index" class="entry-row">
        <select v-model="entry.account_id" required>
          <option value="">Sélectionner un compte</option>
          <option v-for="account in flatAccounts" :key="account.id" :value="account.id">
            {{ account.fullName }}
          </option>
        </select>
        
        <input 
          v-model.number="entry.amount" 
          type="number" 
          step="0.01" 
          min="0"
          placeholder="Montant" 
          required 
        />
        
        <select v-model="entry.type" required>
          <option value="debit">Débit</option>
          <option value="credit">Crédit</option>
        </select>
        
        <button type="button" @click="removeEntry(index)" v-if="transaction.entries.length > 2">
          Supprimer
        </button>
      </div>
      
      <button type="button" @click="addEntry">Ajouter une écriture</button>
    </div>
    
    <div class="balance-check">
      <span :class="{ balanced: isBalanced, unbalanced: !isBalanced }">
        Balance: {{ balanceText }}
      </span>
    </div>
    
    <button type="submit" :disabled="!isBalanced">
      Créer la transaction
    </button>
  </form>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAccountStore } from '../../stores/accounts'
import type { TransactionInput, EntryInput } from '../../types/transactions'

const accountStore = useAccountStore()

const transaction = ref<TransactionInput>({
  description: '',
  date: new Date().toISOString().split('T')[0],
  entries: [
    { account_id: '', amount: 0, type: 'debit' },
    { account_id: '', amount: 0, type: 'credit' }
  ]
})

const flatAccounts = computed(() => accountStore.flattenedAccounts)

const totalDebits = computed(() => 
  transaction.value.entries
    .filter(e => e.type === 'debit')
    .reduce((sum, e) => sum + (e.amount || 0), 0)
)

const totalCredits = computed(() => 
  transaction.value.entries
    .filter(e => e.type === 'credit')
    .reduce((sum, e) => sum + (e.amount || 0), 0)
)

const isBalanced = computed(() => 
  totalDebits.value === totalCredits.value && totalDebits.value > 0
)

const balanceText = computed(() => 
  `Débits: ${totalDebits.value}€ | Crédits: ${totalCredits.value}€`
)

const addEntry = () => {
  transaction.value.entries.push({ account_id: '', amount: 0, type: 'debit' })
}

const removeEntry = (index: number) => {
  transaction.value.entries.splice(index, 1)
}

const submitTransaction = async () => {
  if (!isBalanced.value) return
  
  try {
    // Call Tauri command to create transaction
    await window.__TAURI__.invoke('create_transaction', {
      transaction: transaction.value
    })
    
    // Reset form
    transaction.value = {
      description: '',
      date: new Date().toISOString().split('T')[0],
      entries: [
        { account_id: '', amount: 0, type: 'debit' },
        { account_id: '', amount: 0, type: 'credit' }
      ]
    }
  } catch (error) {
    console.error('Failed to create transaction:', error)
  }
}
</script>
```

#### Acceptance Criteria:
- [x] Account tree displays hierarchy correctly
- [x] Transaction form validates double-entry
- [x] Balance calculation works in real-time
- [x] Form submission is prepared for Tauri commands
- [x] French localization is implemented
- [x] Responsive design works on different screen sizes

---

### Step 8: Connect UI to Backend via Tauri Commands
**Estimated Time**: 2.5 hours  
**Dependencies**: Step 7

#### Tasks:
1. **Create Tauri command handlers**
```rust
// wallet-tauri/src-tauri/src/commands/accounts.rs
use wallet_core::services::AccountService;
use wallet_core::models::{Account, AccountType};
use tauri::State;
use crate::AppState;

#[tauri::command]
pub async fn create_account(
    name: String,
    account_type: String,
    parent_id: Option<i64>,
    currency: String,
    state: State<'_, AppState>,
) -> Result<Account, String> {
    let account_type = match account_type.as_str() {
        "asset" => AccountType::Asset,
        "liability" => AccountType::Liability,
        "equity" => AccountType::Equity,
        "income" => AccountType::Income,
        "expense" => AccountType::Expense,
        _ => return Err("Invalid account type".to_string()),
    };
    
    state.account_service
        .create_account(name, account_type, parent_id, currency)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_account_hierarchy(
    state: State<'_, AppState>,
) -> Result<Vec<Account>, String> {
    state.account_service
        .get_hierarchy()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_account_balance(
    account_id: i64,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    state.account_service
        .calculate_balance(account_id)
        .await
        .map(|money| money.amount_minor)
        .map_err(|e| e.to_string())
}
```

2. **Set up application state**
```rust
// wallet-tauri/src-tauri/src/main.rs
use wallet_core::db::Database;
use wallet_core::services::AccountService;
use std::sync::Arc;

pub struct AppState {
    pub account_service: Arc<AccountService>,
    pub transaction_service: Arc<TransactionService>,
}

#[tokio::main]
async fn main() {
    // Initialize database
    let database = Database::new("sqlite:./wallet.db")
        .await
        .expect("Failed to connect to database");
    
    database.migrate()
        .await
        .expect("Failed to run migrations");
    
    // Initialize services
    let account_repository = AccountRepository::new(database.pool.clone());
    let account_service = Arc::new(AccountService::new(account_repository));
    
    let app_state = AppState {
        account_service,
        transaction_service: todo!(),
    };
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            create_account,
            get_account_hierarchy,
            get_account_balance,
            create_transaction
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

3. **Create Vue stores with Tauri integration**
```typescript
// src/stores/accounts.ts
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import type { Account } from '../types/accounts'

export const useAccountStore = defineStore('accounts', () => {
  const accounts = ref<Account[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  
  const flattenedAccounts = computed(() => {
    const flatten = (accs: Account[], prefix = ''): Account[] => {
      return accs.reduce((result, account) => {
        const fullName = prefix ? `${prefix} → ${account.name}` : account.name
        result.push({ ...account, fullName })
        
        if (account.children) {
          result.push(...flatten(account.children, fullName))
        }
        
        return result
      }, [] as Account[])
    }
    
    return flatten(accounts.value)
  })
  
  const loadAccounts = async () => {
    loading.value = true
    error.value = null
    
    try {
      accounts.value = await invoke('get_account_hierarchy')
    } catch (err) {
      error.value = err as string
    } finally {
      loading.value = false
    }
  }
  
  const createAccount = async (
    name: string,
    accountType: string,
    parentId?: number,
    currency = 'EUR'
  ) => {
    try {
      await invoke('create_account', {
        name,
        accountType,
        parentId,
        currency
      })
      await loadAccounts() // Refresh
    } catch (err) {
      error.value = err as string
      throw err
    }
  }
  
  return {
    accounts,
    loading,
    error,
    flattenedAccounts,
    loadAccounts,
    createAccount
  }
})
```

#### Acceptance Criteria:
- [x] Tauri commands handle account operations
- [x] Vue stores communicate with backend
- [x] Error handling works end-to-end
- [x] Database operations complete successfully
- [x] UI updates reflect backend state changes

---

### Step 9: Add Dashboard and Basic Reporting
**Estimated Time**: 2 hours  
**Dependencies**: Step 8

#### Tasks:
1. **Create dashboard component**
```vue
<!-- src/components/Dashboard.vue -->
<template>
  <div class="dashboard">
    <h1>Tableau de Bord</h1>
    
    <div class="summary-cards">
      <div class="card">
        <h3>Total Actifs</h3>
        <p class="amount">{{ formatMoney(totalAssets) }}</p>
      </div>
      
      <div class="card">
        <h3>Total Passifs</h3>
        <p class="amount">{{ formatMoney(totalLiabilities) }}</p>
      </div>
      
      <div class="card">
        <h3>Valeur Nette</h3>
        <p class="amount">{{ formatMoney(netWorth) }}</p>
      </div>
    </div>
    
    <div class="monthly-summary">
      <h2>Résumé Mensuel</h2>
      <div class="month-data">
        <div class="income">
          <h4>Revenus</h4>
          <p class="amount positive">{{ formatMoney(monthlyIncome) }}</p>
        </div>
        <div class="expenses">
          <h4>Dépenses</h4>
          <p class="amount negative">{{ formatMoney(monthlyExpenses) }}</p>
        </div>
        <div class="net">
          <h4>Net</h4>
          <p class="amount" :class="{ positive: monthlyNet > 0, negative: monthlyNet < 0 }">
            {{ formatMoney(monthlyNet) }}
          </p>
        </div>
      </div>
    </div>
    
    <div class="recent-transactions">
      <h2>Transactions Récentes</h2>
      <TransactionList :transactions="recentTransactions" :limit="10" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAccountStore } from '../stores/accounts'
import { useTransactionStore } from '../stores/transactions'
import TransactionList from './transactions/TransactionList.vue'

const accountStore = useAccountStore()
const transactionStore = useTransactionStore()

const totalAssets = computed(() => 
  accountStore.accounts
    .filter(a => a.account_type === 'asset')
    .reduce((sum, a) => sum + a.balance, 0)
)

const totalLiabilities = computed(() => 
  accountStore.accounts
    .filter(a => a.account_type === 'liability')
    .reduce((sum, a) => sum + a.balance, 0)
)

const netWorth = computed(() => totalAssets.value - totalLiabilities.value)

const monthlyIncome = computed(() => 
  transactionStore.currentMonthSummary.income
)

const monthlyExpenses = computed(() => 
  transactionStore.currentMonthSummary.expenses
)

const monthlyNet = computed(() => monthlyIncome.value - monthlyExpenses.value)

const recentTransactions = computed(() => 
  transactionStore.transactions.slice(0, 10)
)

const formatMoney = (amountMinor: number) => {
  return new Intl.NumberFormat('fr-FR', {
    style: 'currency',
    currency: 'EUR'
  }).format(amountMinor / 100)
}

onMounted(async () => {
  await Promise.all([
    accountStore.loadAccounts(),
    transactionStore.loadRecentTransactions(),
    transactionStore.loadMonthlySummary()
  ])
})
</script>
```

2. **Add reporting Tauri commands**
```rust
// wallet-tauri/src-tauri/src/commands/reports.rs
#[tauri::command]
pub async fn get_monthly_summary(
    year: i32,
    month: u8,
    state: State<'_, AppState>,
) -> Result<MonthlySummary, String> {
    state.reporting_service
        .get_monthly_summary(year, month)
        .await
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
pub struct MonthlySummary {
    pub income: i64,
    pub expenses: i64,
    pub net: i64,
}
```

#### Acceptance Criteria:
- [x] Dashboard displays account summaries
- [x] Monthly income/expense calculation works
- [x] Recent transactions are shown
- [x] All amounts display in proper French format
- [x] Real-time data updates from backend

---

### Step 10: Set up GitHub Actions and Documentation
**Estimated Time**: 1 hour  
**Dependencies**: Step 9

#### Tasks:
1. **Create GitHub Actions workflow**
```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        profile: minimal
        override: true
        components: rustfmt, clippy
    
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
    
    - name: Rust Cache
      uses: Swatinem/rust-cache@v2
    
    - name: Format
      run: cargo fmt --all -- --check
    
    - name: Clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Test wallet-core
      run: cd wallet-core && cargo test
    
    - name: Install Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: wallet-tauri/package-lock.json
    
    - name: Install frontend dependencies
      run: cd wallet-tauri && npm ci
    
    - name: Build Tauri app
      run: cd wallet-tauri && npm run tauri build

  build-release:
    if: startsWith(github.ref, 'refs/tags/')
    strategy:
      matrix:
        platform: [ubuntu-latest, windows-latest, macos-latest]
    
    runs-on: ${{ matrix.platform }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        profile: minimal
        override: true
    
    - name: Install dependencies (Ubuntu)
      if: matrix.platform == 'ubuntu-latest'
      run: |
        sudo apt-get update
        sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
    
    - name: Install Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: wallet-tauri/package-lock.json
    
    - name: Install frontend dependencies
      run: cd wallet-tauri && npm ci
    
    - name: Build Tauri app
      run: cd wallet-tauri && npm run tauri build
    
    - name: Upload Release Asset
      uses: actions/upload-artifact@v3
      with:
        name: release-${{ matrix.platform }}
        path: wallet-tauri/src-tauri/target/release/bundle/
```

2. **Update README with setup instructions**
```markdown
# Quick Start

## Development Setup

1. **Install Prerequisites**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install Node.js (18+)
   # Install system dependencies (Linux)
   sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
   ```

2. **Clone and Setup**
   ```bash
   git clone <repository-url>
   cd oxidized-wallet
   
   # Install frontend dependencies
   cd wallet-tauri && npm install
   ```

3. **Run Development Server**
   ```bash
   # From wallet-tauri directory
   npm run tauri dev
   ```

## Testing

```bash
# Test core business logic
cd wallet-core && cargo test

# Lint and format
cargo clippy && cargo fmt

# Test frontend
cd wallet-tauri && npm test
```
```

#### Acceptance Criteria:
- [x] CI/CD pipeline runs successfully
- [x] All tests pass in CI environment
- [x] Release builds work for all platforms
- [x] README provides clear setup instructions
- [x] Development workflow is documented

---

## MVP Completion Criteria

The MVP is considered complete when:

- [x] **Account Management**: Can create hierarchical accounts and view balances
- [x] **Transaction Entry**: Can create double-entry transactions with validation
- [x] **Basic Reporting**: Dashboard shows account summaries and monthly totals
- [x] **Data Persistence**: All data is stored in SQLite and persists between sessions
- [x] **User Interface**: Vue 3 interface provides smooth user experience
- [x] **Code Quality**: All tests pass, code is properly formatted and documented

## Post-MVP Next Steps

After MVP completion, proceed to Phase 2 features:
- User-defined categories
- Transaction states and reconciliation
- Enhanced reporting with charts
- CSV export functionality
- Advanced search and filtering

## Estimated Total Time: 20-25 hours

This represents a solid MVP that provides core functionality for personal finance tracking with proper double-entry bookkeeping.