# Development Decisions - Oxidized Wallet

This document captures all technical decisions made during the planning phase, providing rationale and implementation guidance.

## Core Technical Decisions

### Money Representation
**Decision**: Use `amount_minor` instead of "cents"
```rust
pub struct Money {
    amount_minor: i64,    // Smallest unit (100 = 1 EUR, 100000000 = 1 BTC)
    currency: Currency,
}

pub struct Currency {
    code: String,         // "EUR", "BTC"
    minor_unit_scale: u8, // 2 for EUR, 8 for BTC
    symbol: String,       // "€", "₿"
}
```
**Rationale**: Supports any currency without hardcoding "cents" terminology

### Date/Time Strategy
**Decision**: `NaiveDate` for user-facing dates, UTC for system timestamps
```rust
pub struct Transaction {
    date: NaiveDate,           // User transaction date (no time/timezone)
    created_at: DateTime<Utc>, // System audit timestamp
    updated_at: DateTime<Utc>, // System modification timestamp
}
```
**Rationale**: 
- Financial transactions typically care about date, not time
- Avoids timezone complexity for users
- System timestamps in UTC for consistency

### Decimal Computation
**Decision**: Use `rust_decimal` for calculations
```rust
use rust_decimal::Decimal;

impl Money {
    pub fn to_decimal(&self) -> Decimal {
        Decimal::from(self.amount_minor) / Decimal::from(10_i64.pow(self.currency.minor_unit_scale as u32))
    }
}
```
**Rationale**: Proper decimal arithmetic instead of binary floating-point

### Database Migrations
**Decision**: Automatic backup before migrations
```rust
async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    backup_database()?;
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
```
**Rationale**: Safety net for schema changes in production

### Configuration Management
**Decision**: TOML configuration in platform-specific directories
```toml
# ~/.config/oxidized-wallet/config.toml (Linux)
# ~/Library/Application Support/oxidized-wallet/config.toml (macOS)
# %APPDATA%\oxidized-wallet\config.toml (Windows)

[display]
date_format = "YYYY-MM-DD"    # ISO 8601 everywhere
decimal_separator = "."
thousand_separator = ","

[defaults]
currency = "EUR"
```

### Logging Strategy
**Decision**: Use `tracing` crate
```rust
use tracing::{info, warn, error, debug};

#[tracing::instrument]
pub async fn create_transaction(transaction: Transaction) -> Result<Transaction> {
    info!("Creating transaction");
    // ... implementation
}
```
**Rationale**: Structured logging with performance tracking

### Error Handling
**Decision**: Clear error types with user-friendly messages
```rust
#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Transaction non équilibrée: débits {debits} ≠ crédits {credits}")]
    UnbalancedTransaction { debits: Money, credits: Money },
    
    #[error("Compte {account} introuvable")]
    AccountNotFound { account: String },
}
```

### Backup Strategy
**Decision**: SQLite file copy for MVP, enhanced formats later
- **MVP**: Simple file copy with timestamp
- **Phase 2**: Add JSON export
- **Phase 3**: Add encrypted backups

### Search Implementation
**Decision**: SQL LIKE for MVP, FTS5 later
```sql
-- MVP: Simple search
SELECT * FROM transactions 
WHERE description LIKE '%' || ?1 || '%' 
ORDER BY date DESC;

-- Phase 2: Full-text search
CREATE VIRTUAL TABLE transaction_search USING fts5(...);
```

### Security Measures
**Decision**: Data sanitization and optional encryption
- All queries use parameterized statements (sqlx handles this)
- Path validation for imports/exports
- Optional SQLCipher support (Phase 3)

### API Versioning
**Decision**: Workspace-level versioning
```toml
[workspace]
members = ["wallet-core", "wallet-tauri", "wallet-cli"]

[workspace.package]
version = "0.1.0"
authors = ["Nicolas Schoonbroodt"]
edition = "2021"
```

## Feature-Specific Decisions

### Transaction States (Phase 2)
**Decision**: Add states after MVP
```rust
pub enum TransactionState {
    Confirmed,  // Default for MVP
    Pending,    // Phase 2
    Scheduled,  // Phase 3
}
```

### Duplicate Detection (Phase 2)
**Decision**: Configurable matching criteria
```rust
pub struct DuplicateCheckCriteria {
    date_tolerance_days: i32,
    amount_must_match: bool,
    description_similarity: f32, // 0.0 to 1.0
}
```

### Reconciliation (Phase 2)
**Decision**: Add reconciliation flag
```sql
ALTER TABLE transactions ADD COLUMN reconciled BOOLEAN DEFAULT FALSE;
ALTER TABLE transactions ADD COLUMN reconciled_at TIMESTAMP;
```

### Categories (Phase 2)
**Decision**: User-defined categories with optional presets
```rust
pub struct Category {
    id: i64,
    name: String,
    parent_id: Option<i64>,
    user_defined: bool,
}
```

### Import Profiles (Phase 3)
**Decision**: Save import mappings
```rust
pub struct ImportProfile {
    name: String,
    bank: String,
    account_mappings: HashMap<String, i64>,
    column_mappings: ColumnMapping,
}
```

### Undo/Redo
**Decision**: No undo, but support reverse transactions
- Transactions are immutable
- Create compensating transactions for corrections
- Clear confirmation dialogs

## Development Guidelines

### Code Quality
- Run `cargo clippy` before commits
- Maintain > 80% test coverage for wallet-core
- Use `#[tracing::instrument]` for key functions

### Performance Targets
- Account balance calculation: < 50ms
- Transaction list (100 items): < 100ms
- Report generation: < 500ms

### Accessibility
- Full keyboard navigation
- Semantic HTML in Vue components
- ARIA labels where needed

### Distribution
- **MVP**: Manual GitHub releases
- **Post-MVP**: GitHub Actions for:
  - Linux: AppImage
  - macOS: .dmg
  - Windows: .msi

## Platform-Specific Considerations

### Linux
- Follow XDG Base Directory specification
- Desktop file for application menu

### macOS
- Code signing (Phase 3)
- Notarization for distribution

### Windows
- Store config in %APPDATA%
- Consider portable mode

## Future Considerations

### Mobile Support (Phase 4+)
- Design wallet-core API to be mobile-ready
- Consider REST API wrapper
- Possible Tauri Mobile when stable

### Extensibility
- Design with plugin architecture in mind
- Consider Lua for custom reports (Phase 4+)

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2024-01-XX | Use `amount_minor` not "cents" | Support multiple currencies |
| 2024-01-XX | NaiveDate for transactions | Simplify timezone handling |
| 2024-01-XX | Vue 3 over React | Easier learning curve, better for forms |
| 2024-01-XX | SQLite over PostgreSQL | Local-first requirement |
| 2024-01-XX | Workspace versioning | Simplify multi-crate management |

This document will be updated as new decisions are made during development.