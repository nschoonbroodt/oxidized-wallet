# Oxidized Wallet - Claude Development Context

## Project Overview
Personal finance tracking software with double-entry bookkeeping, written in Rust with Tauri for desktop UI. Local-first architecture with SQLite storage. Designed for French users managing EUR accounts across multiple banks.

## Key Project Constraints
- **Local-first**: No cloud data storage, SQLite only
- **Double-entry bookkeeping**: Every transaction must balance
- **Hierarchical accounts**: Support 3+ levels (BoursoBank → Compte courant → Sous-compte)
- **French market focus**: EUR currency, French banking patterns
- **Learning project**: User wants to learn Rust through this project

## Architecture Decisions

### Technology Stack
- **Backend**: Rust with sqlx for database operations
- **Frontend**: Tauri + Vue 3 + TypeScript
- **Database**: SQLite with strict ACID compliance
- **Architecture**: Split codebase (wallet-core + wallet-tauri)

### Key Design Patterns
- Repository pattern for data access (internal layer)
- Service layer for business logic (public layer)
- Integer-based money storage (avoid floating-point errors)

## Development Commands

### Setup
```bash
# Install frontend dependencies
cd wallet-tauri/src-ui && npm install
```

### Development Workflow
```bash
# Run tests (core business logic)
cd wallet-core && cargo test

# Run Tauri app in development mode
cd wallet-tauri && cargo tauri dev

# Database migrations (also run automatically in tauri)
cd wallet-core && sqlx migrate run

# Lint and format
cargo clippy && cargo fmt

# Frontend development
cd wallet-tauri/src-ui && npm run dev
```

### Database Operations
Managed on boot by tauri app and [sqlx:test]

## Current Development Phase
**Phase**: MVP Complete - Planning v0.2.0
**Status**: Production-ready MVP with full double-entry bookkeeping
**Next**: v0.2.0. See plan in docs/planning/V0.2.0_PLAN.md

## Code Style Preferences
- Use `thiserror` for error handling
- Prefer `Result<T, E>` over panics
- Use `async/await` for database operations
- Strong typing with custom types (Money, AccountType, etc.)
- Comprehensive unit tests for business logic
- Clear separation between core logic and UI
- Vue 3 Composition API with TypeScript
- Use composables for reusable logic
- Use `amount_minor` for money storage (not "cents")
- Use `NaiveDate` for transaction dates (no timezone)
- Use `rust_decimal` for calculations

## Business Rules (Critical)

### Double-Entry Bookkeeping
1. Every transaction must have at least 2 entries
2. Sum of debits must equal sum of credits
3. Debits increase: Assets, Expenses
4. Credits increase: Liabilities, Equity, Income
5. Account types determine normal balance direction

### Account Hierarchy
- top level account = Assets, Liabilities, Income, Expenses, Equity (created by migration)
- Unique names within same parent
- Soft delete (mark inactive, preserve history)
- Balance calculation includes child accounts

### Data Integrity
- Transactions are immutable once created
- All monetary amounts stored as integers (amount_minor)
- Strict foreign key constraints
- Audit trail for all changes

## French Banking Context
- EUR as primary currency
- Common account types: Compte courant, Livret A, PEA
- Banks: BoursoBank, Société Générale, etc.
- Future: Support for French tax categories

## Testing Strategy
- Unit tests: Core business logic (wallet-core)
- Integration tests: Database operations
- End-to-end tests: Tauri commands
- Property-based testing for financial calculations

## Security Requirements
- Local data only (no network calls)
- Input validation for all financial data
- SQL injection prevention (parameterized queries)
- No secrets in code or logs

## Future Considerations
- Multi-currency support (schema ready)
- CSV import from French banks
- Investment tracking (PEA accounts)
- Budget tracking and reporting

## Documentation Files
- `docs/architecture/TECHNICAL_ARCHITECTURE.md`: Technology choices and reasoning (Vue 3)
- `docs/architecture/DATABASE_SCHEMA.md`: Complete database design with amount_minor
- `docs/architecture/PROJECT_STRUCTURE.md`: Codebase organization (Vue 3 frontend)
- `docs/architecture/DEVELOPMENT_DECISIONS.md`: All technical decisions and rationale
- `docs/planning/FEATURE_ROADMAP.md`: Development phases and milestones
- `docs/planning/DOUBLE_ENTRY_RULES.md`: Accounting principles and implementation

## Development Notes
- Use `cargo workspace` for multi-crate project
- Workspace-level versioning (shared version across crates)
- Tauri configuration in `tauri.conf.json`
- Database migrations in `wallet-core/migrations/`
- Vue components in `wallet-tauri/src/components/`
- Automatic backup before migrations
- Configuration in platform-specific directories (XDG, AppSupport)

## Critical Success Factors
1. **Data integrity**: Double-entry validation must be bulletproof
2. **User experience**: French users should find it intuitive
3. **Performance**: Responsive even with years of data
4. **Reliability**: No data loss, robust error handling

Remember: This is a learning project, so explain Rust concepts and patterns when implementing features. Focus on clean, maintainable code over premature optimization.