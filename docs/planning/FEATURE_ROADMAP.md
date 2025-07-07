# Feature Roadmap - Oxidized Wallet

## Project Vision
Create a local-first personal finance tracking application with double-entry bookkeeping, designed for French users managing EUR accounts across multiple banks with hierarchical organization.

## Development Phases

### 🎯 MVP (Phase 1) - Core Foundation ✅
**Goal**: Functional double-entry bookkeeping with basic account management

#### Account Management
- ✅ Create hierarchical account structure (5 levels supported)
- ✅ Account types: Asset, Liability, Equity, Income, Expense
- ✅ Account balance calculation with hierarchy
- ⏳ Predefined French account categories → v0.2.0
- ⏳ Account editing UI → v0.2.0
- ⏳ Account deactivation UI (backend complete) → v0.2.0

#### Transaction Management
- ✅ Double-entry transaction creation
- ✅ Manual transaction entry with validation
- ✅ Transaction description and reference fields
- ✅ Date-based transaction organization
- ✅ Basic transaction listing and search by date

#### Core Reporting
- ✅ Individual account balances
- ✅ Total portfolio value
- ✅ Monthly income vs expense summary
- ✅ Account hierarchy view with balances

#### Technical Foundation
- ✅ SQLite database with proper schema
- ✅ Tauri application setup
- ✅ Basic Vue 3 frontend
- ✅ Core business logic in wallet-core
- ✅ Double-entry validation and constraints

#### User Interface (MVP)
- ✅ Account tree view
- ✅ Transaction entry form
- ✅ Simple transaction list
- ✅ Basic dashboard with key metrics
- ✅ French language interface

**MVP Success Criteria**: ✅
- ✅ Can create account hierarchy matching user's bank structure
- ✅ Can enter transactions manually with double-entry validation
- ✅ Shows accurate account balances and monthly summaries
- ✅ Data persists locally in SQLite

---

### 🚀 Phase 2 - Enhanced Usability (v0.2.0)
**Timeline**: 3-4 weeks  
**Goal**: Polish MVP and improve daily usability  
**Status**: In Planning - See `docs/planning/V0.2.0_PLAN.md`

#### Priority Features (v0.2.0)
- 🎯 Transaction templates for recurring entries
- 🎯 Account editing UI (backend already supports)
- 🎯 Account deactivation UI (soft delete)
- 🎯 CSV export functionality (transactions, balances, reports)
- 🎯 Enhanced transaction search and filtering
- 🎯 Keyboard shortcuts for power users
- 🎯 Transaction entry improvements (auto-complete, recent accounts)
- 🎯 Better error handling and user feedback
- 🎯 Database backup/restore UI

#### Deferred to Later Phases
- 📅 User-defined categories and tagging system → Phase 3
- 📅 Transaction states (Pending, Confirmed) → Phase 3
- 📅 Transaction reconciliation flags → Phase 3
- 📅 Transaction editing (immutable by design)
- 📅 Category-based expense analysis → Phase 3
- 📅 Advanced reporting (balance sheet, income statement) → Phase 3
- 📅 Dark mode support → Phase 3

**Phase 2 Success Criteria**:
- Transaction templates save time on recurring entries
- Complete account management UI (create, edit, deactivate)
- Data export for backup and external analysis
- Improved search and navigation efficiency
- Professional error handling

---

### 📊 Phase 3 - Advanced Features
**Timeline**: 4-5 weeks  
**Goal**: Professional-grade financial tracking

#### CSV Import System
- 📋 Bank CSV import with mapping
- 📋 Import profiles for major French banks (BoursoBank, SG, etc.)
- 📋 Duplicate transaction detection during import
- 📋 Import validation and error handling
- 📋 Enhanced reconciliation features

#### Budget and Planning
- 📋 Budget creation and tracking
- 📋 Budget vs actual reporting
- 📋 Spending alerts and notifications
- 📋 Financial goal setting and tracking

#### Advanced Reporting
- 📋 Charts and graphs (spending trends, category breakdowns)
- 📋 Net worth tracking over time
- 📋 Cash flow analysis
- 📋 Tax preparation reports (French tax categories)

#### Recurring Transactions
- 📋 Automatic recurring transaction creation
- 📋 Flexible recurrence patterns (monthly, quarterly, annual)
- 📋 Recurring transaction management

#### Enhanced Data Management
- 📋 Data archiving for performance
- 📋 Advanced search with filters
- 📋 Custom report builder

**Phase 3 Success Criteria**:
- Seamless bank data import
- Comprehensive budgeting system
- Professional-grade reporting with visualizations

---

### 🌍 Phase 4 - Multi-Currency & Investments
**Timeline**: 5-6 weeks  
**Goal**: Support for complex financial scenarios

#### Multi-Currency Support
- 📋 Multiple currency accounts
- 📋 Exchange rate tracking and updates
- 📋 Currency conversion in transactions
- 📋 Multi-currency reporting
- 📋 Foreign exchange gain/loss tracking

#### Investment Tracking
- 📋 Securities and ETF tracking (for PEA accounts)
- 📋 Investment portfolio management
- 📋 Cost basis tracking
- 📋 Dividend and interest recording
- 📋 Investment performance reporting

#### Advanced Account Types
- 📋 Investment accounts (PEA, Assurance Vie)
- 📋 Loan and mortgage tracking
- 📋 Credit card integration
- 📋 Business expense categorization

#### Performance and Scalability
- 📋 Database optimization for large datasets
- 📋 Account balance caching
- 📋 Lazy loading for better performance
- 📋 Memory usage optimization

**Phase 4 Success Criteria**:
- Full multi-currency support for eurozone accounts
- Investment tracking suitable for PEA management
- Performance optimized for years of financial data

---

### 🔮 Future Phases (Phase 5+)
**Long-term enhancements based on user feedback**

#### Cloud Sync (Optional)
- 📋 End-to-end encrypted cloud sync
- 📋 Multi-device synchronization
- 📋 Conflict resolution
- 📋 User-controlled encryption keys

#### Advanced Analytics
- 📋 Machine learning for expense categorization
- 📋 Spending pattern analysis
- 📋 Financial health scoring
- 📋 Predictive budgeting

#### Integration Features
- 📋 Bank API integration (PSD2 compliance)
- 📋 Tax software export
- 📋 Accounting software integration
- 📋 Mobile companion app

#### Enterprise Features
- 📋 Multi-user support
- 📋 Audit trails and compliance
- 📋 Advanced permission system
- 📋 Batch operations

## Risk Assessment and Mitigation

### Technical Risks
- **Database corruption**: Automatic backup before migrations, SQLite integrity checks
- **Performance with large datasets**: Start simple, add caching only if needed
- **Double-entry validation complexity**: Comprehensive test suite with property-based testing
- **Decimal precision**: Use rust_decimal for calculations, amount_minor for storage

### User Experience Risks
- **Steep learning curve**: Progressive disclosure, good documentation
- **Data migration fears**: Robust export/import, clear data ownership
- **Complex account setup**: Wizards and templates for common scenarios

### Scope Creep Risks
- **Feature bloat**: Stick to roadmap, defer non-essential features
- **Over-engineering**: Focus on user needs, not technical perfection
- **Platform proliferation**: Stay focused on desktop until MVP is solid

## Success Metrics

### MVP Metrics
- User can complete basic workflow (create accounts, enter transactions) in <10 minutes
- Zero data loss incidents
- Transaction entry time <30 seconds for typical transaction

### Phase 2+ Metrics
- CSV import success rate >95%
- Report generation time <2 seconds for 1 year of data
- User retention through budget cycles

### Long-term Metrics
- Support for 10,000+ transactions without performance degradation
- User satisfaction score >4.5/5
- Zero critical security incidents

This roadmap balances feature richness with development complexity, ensuring each phase delivers real value while building toward a comprehensive personal finance solution.