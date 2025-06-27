# Project Structure - Oxidized Wallet

## Overview
This document defines the organization of the Oxidized Wallet codebase, implementing a clean separation between business logic and user interface through a multi-crate workspace.

## Workspace Structure

```
oxidized-wallet/
├── Cargo.toml                 # Workspace configuration
├── Cargo.lock                 # Dependency lock file
├── README.md                  # Project overview and setup
├── CLAUDE.md                  # Development context and commands
├── LICENSE                    # MIT license
│
├── docs/                      # Documentation
│   ├── TECHNICAL_ARCHITECTURE.md
│   ├── DATABASE_SCHEMA.md
│   ├── FEATURE_ROADMAP.md
│   ├── PROJECT_STRUCTURE.md
│   └── API_REFERENCE.md       # Core API documentation
│
├── wallet-core/               # Business logic library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs            # Public API exports
│   │   ├── models/           # Data structures
│   │   │   ├── mod.rs
│   │   │   ├── account.rs    # Account entity
│   │   │   ├── transaction.rs # Transaction entity
│   │   │   ├── money.rs      # Money type with currency
│   │   │   └── entry.rs      # Transaction entry
│   │   ├── db/               # Database layer
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs # Database connection management
│   │   │   ├── migrations.rs # Schema migrations
│   │   │   ├── accounts.rs   # Account queries
│   │   │   ├── transactions.rs # Transaction queries
│   │   │   └── reporting.rs  # Reporting queries
│   │   ├── services/         # Business logic
│   │   │   ├── mod.rs
│   │   │   ├── account_service.rs    # Account management
│   │   │   ├── transaction_service.rs # Transaction management
│   │   │   ├── reporting_service.rs  # Financial reporting
│   │   │   └── validation.rs # Business rule validation
│   │   ├── errors/           # Error handling
│   │   │   ├── mod.rs
│   │   │   ├── db_error.rs   # Database errors
│   │   │   ├── validation_error.rs # Validation errors
│   │   │   └── service_error.rs # Service layer errors
│   │   └── utils/            # Utility functions
│   │       ├── mod.rs
│   │       ├── currency.rs   # Currency utilities
│   │       └── date.rs       # Date handling
│   ├── tests/                # Integration tests
│   │   ├── account_tests.rs
│   │   ├── transaction_tests.rs
│   │   └── reporting_tests.rs
│   └── migrations/           # SQL migration files
│       ├── 001_initial_schema.sql
│       ├── 002_add_indexes.sql
│       └── 003_add_triggers.sql
│
├── wallet-tauri/             # Tauri desktop application
│   ├── Cargo.toml
│   ├── tauri.conf.json       # Tauri configuration
│   ├── src/
│   │   ├── main.rs           # Application entry point
│   │   ├── commands/         # Tauri commands (API layer)
│   │   │   ├── mod.rs
│   │   │   ├── accounts.rs   # Account-related commands
│   │   │   ├── transactions.rs # Transaction-related commands
│   │   │   └── reports.rs    # Reporting commands
│   │   ├── state/            # Application state management
│   │   │   ├── mod.rs
│   │   │   └── app_state.rs  # Shared application state
│   │   └── utils/            # Tauri-specific utilities
│   │       ├── mod.rs
│   │       └── error_handling.rs # Error conversion for frontend
│   ├── src-ui/               # Frontend Vue 3 application
│   │   ├── package.json      # Node.js dependencies
│   │   ├── package-lock.json
│   │   ├── tsconfig.json     # TypeScript configuration
│   │   ├── vite.config.ts    # Vite build configuration
│   │   ├── index.html        # HTML entry point
│   │   ├── src/
│   │   │   ├── main.ts       # Vue application entry
│   │   │   ├── App.vue       # Main application component
│   │   │   ├── components/   # Vue components
│   │   │   │   ├── accounts/
│   │   │   │   │   ├── AccountTree.vue
│   │   │   │   │   ├── AccountForm.vue
│   │   │   │   │   └── AccountBalance.vue
│   │   │   │   ├── transactions/
│   │   │   │   │   ├── TransactionForm.vue
│   │   │   │   │   ├── TransactionList.vue
│   │   │   │   │   └── TransactionEntry.vue
│   │   │   │   ├── reports/
│   │   │   │   │   ├── Dashboard.vue
│   │   │   │   │   ├── BalanceSheet.vue
│   │   │   │   │   └── IncomeStatement.vue
│   │   │   │   └── common/
│   │   │   │       ├── Layout.vue
│   │   │   │       ├── Navigation.vue
│   │   │   │       └── ErrorBoundary.vue
│   │   │   ├── composables/  # Vue 3 composables
│   │   │   │   ├── useAccounts.ts
│   │   │   │   ├── useTransactions.ts
│   │   │   │   └── useReports.ts
│   │   │   ├── types/        # TypeScript type definitions
│   │   │   │   ├── accounts.ts
│   │   │   │   ├── transactions.ts
│   │   │   │   └── api.ts
│   │   │   ├── utils/        # Frontend utilities
│   │   │   │   ├── api.ts    # Tauri API wrappers
│   │   │   │   ├── formatting.ts # Number/date formatting
│   │   │   │   └── validation.ts # Form validation
│   │   │   ├── styles/       # CSS/styling
│   │   │   │   ├── globals.css
│   │   │   │   └── components.css
│   │   │   └── assets/       # Static assets
│   │   │       └── icons/
│   │   └── public/           # Public assets
│   │       └── favicon.ico
│   └── icons/                # Application icons
│       ├── 32x32.png
│       ├── 128x128.png
│       └── icon.ico
│
├── wallet-cli/               # Future CLI interface
│   ├── Cargo.toml
│   └── src/
│       └── main.rs           # CLI entry point (placeholder)
│
└── scripts/                  # Development and build scripts
    ├── setup-dev.sh          # Development environment setup
    ├── build-release.sh      # Release build script
    └── run-tests.sh          # Test runner script
```

## Module Responsibilities

### wallet-core Crate

#### Purpose
Contains all business logic, data models, and database operations. This crate is UI-agnostic and can be used by any frontend.

#### Key Modules

**models/**: Core data structures
- `Account`: Hierarchical account structure with type validation
- `Transaction`: Immutable transaction records
- `Money`: Currency-aware monetary values with integer storage
- `TransactionEntry`: Individual debit/credit entries

**db/**: Database abstraction layer
- Connection pooling and transaction management
- Migration system for schema evolution
- Type-safe query builders using sqlx
- Repository pattern for data access

**services/**: Business logic layer
- Account hierarchy management
- Double-entry transaction validation
- Financial reporting calculations
- Business rule enforcement

**errors/**: Comprehensive error handling
- Domain-specific error types
- Error conversion between layers
- User-friendly error messages

### wallet-tauri Crate

#### Purpose
Desktop application using Tauri framework. Bridges the core business logic with the Vue 3 frontend through Tauri commands.

#### Key Components

**commands/**: Tauri command handlers
- Expose core functionality to frontend
- Handle serialization/deserialization
- Manage application state
- Error handling and user feedback

**Frontend (src-ui/)**:
- Vue 3 with TypeScript for type safety
- Component-based architecture with Composition API
- Composables for state management and logic reuse
- Responsive design for different screen sizes

### Future wallet-cli Crate

#### Purpose
Command-line interface for power users and automation. Will reuse the same wallet-core business logic.

## API Design

### Core Library API (wallet-core)

```rust
// Public API surface
pub use models::{Account, Transaction, Money, TransactionEntry};
pub use services::{AccountService, TransactionService, ReportingService};
pub use errors::{WalletError, ValidationError};

// Example service interface
impl AccountService {
    pub async fn create_account(&self, name: String, account_type: AccountType, parent_id: Option<i64>) -> Result<Account>;
    pub async fn get_account_balance(&self, account_id: i64) -> Result<Money>;
    pub async fn get_account_hierarchy(&self) -> Result<Vec<Account>>;
}
```

### Tauri Commands API

```rust
// Commands exposed to frontend
#[tauri::command]
async fn create_account(name: String, account_type: String, parent_id: Option<i64>) -> Result<Account, String>;

#[tauri::command]
async fn create_transaction(description: String, entries: Vec<TransactionEntryInput>) -> Result<Transaction, String>;

#[tauri::command]
async fn get_monthly_summary(year: i32, month: u8) -> Result<MonthlySummary, String>;
```

## Dependencies Management

### Core Dependencies (wallet-core)
- `sqlx`: Type-safe SQL database operations
- `serde`: Serialization/deserialization
- `thiserror`: Error handling
- `chrono`: Date and time handling
- `uuid`: Unique identifiers
- `rust_decimal`: Precise decimal arithmetic

### Tauri Dependencies (wallet-tauri)
- `tauri`: Desktop app framework
- `serde_json`: JSON handling
- `tokio`: Async runtime

### Frontend Dependencies (Vue 3)
- `vue`: UI framework (v3 with Composition API)
- `typescript`: Type safety
- `vite`: Build tool
- `tailwindcss`: Styling framework
- `pinia`: State management (official Vue store)

## Development Workflow

### Local Development
```bash
# Setup development environment
./scripts/setup-dev.sh

# Run core tests
cd wallet-core && cargo test

# Run Tauri app in development mode
cd wallet-tauri && cargo tauri dev

# Run frontend tests
cd wallet-tauri/src-ui && npm test
```

### Build Process
```bash
# Build release version
./scripts/build-release.sh

# Run all tests
./scripts/run-tests.sh
```

## Testing Strategy

### Unit Tests (wallet-core)
- Business logic validation
- Data model correctness
- Database operations
- Error handling scenarios

### Integration Tests
- End-to-end workflows
- Database migrations
- API contract testing

### Frontend Tests
- Component testing with Vue Test Utils
- User interaction flows
- API integration testing

## File Naming Conventions

### Rust Files
- `snake_case` for file names
- `mod.rs` for module declarations
- Descriptive names reflecting responsibility

### Frontend Files
- `PascalCase` for Vue components (.vue files)
- `camelCase` for utilities and composables
- `kebab-case` for CSS files

### Database Files
- `NNN_descriptive_name.sql` for migrations
- Sequential numbering for migration order

This structure provides clear separation of concerns while maintaining flexibility for future expansion and alternative interfaces.