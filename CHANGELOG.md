# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Documentation updates to reflect MVP completion

## [0.1.0] - 2025-07-06

### Added
- Initial MVP release
- Double-entry bookkeeping with transaction validation
- Hierarchical account management (5-level depth)
- Account balance calculation with child account aggregation
- Transaction creation with debit/credit entries
- Transaction listing with date filtering
- Monthly income/expense reporting
- Dashboard with key financial metrics
- Vue 3 frontend with Tauri desktop application
- SQLite database with automatic migrations
- French localization and EUR currency support
- Type-safe Rust-TypeScript integration

### Technical
- Repository pattern for data access
- Service layer for business logic
- Integer-based money storage (amount_minor)
- Comprehensive test suite for business logic
- Mock data system for frontend development