# Technical Architecture - Oxidized Wallet

## Project Overview
A local-first personal finance tracking software written in Rust, implementing double-entry bookkeeping with a hierarchical account system. Designed for French users with EUR as primary currency, but architected for future multi-currency support.

## Core Technology Choices

### Programming Language: Rust
**Choice**: Rust for all backend logic
**Reasoning**:
- Memory safety without garbage collection
- Excellent type system for financial calculations
- Strong ecosystem for database and web development
- Learning objective stated by user
- Performance benefits for financial calculations

### Architecture Pattern: Split Codebase
**Choice**: Separate core business logic from UI
**Structure**:
```
oxidized-wallet/
├── wallet-core/     # Business logic library
├── wallet-tauri/    # Tauri UI application
└── wallet-cli/      # Future CLI interface
```

**Reasoning**:
- Clean separation of concerns
- Easier unit testing of business logic
- Future-proofing for multiple interfaces
- Enforces good API design
- Allows independent evolution of UI and core logic

### Database: SQLite + sqlx
**Choice**: SQLite with sqlx for database operations
**Reasoning**:
- **Local-first requirement**: SQLite is embedded, no server needed
- **ACID compliance**: Critical for double-entry bookkeeping integrity
- **Single file**: Easy backup and portability
- **sqlx benefits**: 
  - Compile-time checked queries
  - Type-safe database operations
  - Migration support
  - Async support for future scalability

**Alternative considered**: diesel
- **Rejected because**: More complex setup, ORM overhead not needed for this use case

### UI Framework: Tauri
**Choice**: Tauri with web frontend
**Reasoning**:
- **Native performance**: Rust backend with web UI
- **Modern UI development**: Can use Vue/React/Svelte
- **Small bundle size**: Smaller than Electron
- **Cross-platform**: Windows, macOS, Linux support
- **Learning benefit**: Combines Rust learning with modern web tech

**Alternatives considered**:
- **egui**: Native Rust GUI
  - *Rejected*: More complex styling, less familiar development
- **iced**: Native Rust GUI
  - *Rejected*: Smaller ecosystem, steeper learning curve
- **Web server + browser**: Pure web app
  - *Rejected*: Less integrated experience, browser dependency

### Frontend Technology: Vue 3 + TypeScript
**Choice**: Vue 3 with TypeScript for the Tauri frontend
**Reasoning**:
- **Type safety**: TypeScript provides compile-time error catching
- **Gentle learning curve**: Easier to learn while focusing on Rust
- **Form-heavy optimization**: Excellent for data entry applications
- **Smaller bundle size**: Important for desktop app performance
- **Built-in reactivity**: No need for external state management initially
- **Composition API**: Modern patterns similar to React hooks

### Financial Data Handling
**Choice**: Custom Money type with integer-based storage
**Reasoning**:
- **Precision**: Avoid floating-point rounding errors
- **Type safety**: Prevent mixing currencies accidentally
- **Future-ready**: Support for multi-currency operations

```rust
pub struct Money {
    amount_minor: i64,  // Store as smallest currency unit
    currency: Currency,
}
```

### Error Handling
**Choice**: thiserror for custom error types
**Reasoning**:
- **Ergonomic**: Reduces boilerplate for error definitions
- **Integration**: Works well with anyhow for error propagation
- **Type safety**: Compile-time error checking

### Testing Strategy
**Choice**: Multi-level testing approach
- **Unit tests**: Core business logic (wallet-core)
- **Integration tests**: Database operations
- **End-to-end tests**: Tauri commands and UI interactions

## Non-Functional Requirements

### Performance
- **Target**: Sub-100ms response for typical operations
- **Scalability**: Handle 10,000+ transactions efficiently
- **Memory**: Reasonable memory usage for desktop application

### Security
- **Local data**: No cloud storage, user controls all data
- **Input validation**: Strict validation of all financial data
- **Audit trail**: Immutable transaction history

### Usability
- **Responsive UI**: Works on different screen sizes
- **Keyboard shortcuts**: Power user efficiency
- **Error messages**: Clear, actionable feedback in French

### Maintainability
- **Documentation**: Comprehensive inline and external docs
- **Testing**: High test coverage for financial logic
- **Modular design**: Clear separation of concerns

## Future Considerations

### Multi-currency Support
- Database schema already designed for multiple currencies
- Money type supports currency field
- Exchange rate tracking can be added later

### Performance Optimizations
- Database indexing strategy
- Lazy loading for large datasets
- Caching for frequently accessed data

### Backup and Sync
- Export/import functionality
- Encrypted backup files
- Future cloud sync with user-controlled encryption

## Decision Log

| Decision | Alternatives | Reason |
|----------|-------------|---------|
| SQLite | PostgreSQL, MySQL | Local-first requirement, simplicity |
| sqlx | diesel, sea-orm | Compile-time safety without ORM overhead |
| Tauri | Electron, native GUI | Performance, bundle size, Rust integration |
| Vue 3 | React, Svelte | Gentle learning curve, form optimization, smaller bundle |
| Integer money | Decimal types | Precision, performance |

This architecture provides a solid foundation for the MVP while remaining flexible for future enhancements.