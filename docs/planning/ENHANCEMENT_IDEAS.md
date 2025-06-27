# Enhancement Ideas - Oxidized Wallet

This document tracks potential improvements and enhancements that could be implemented in future phases. These are not critical issues but ideas for making the codebase even better.

## 💰 Money & Currency Enhancements

### Currency Validation Improvements
**Priority**: Low  
**Phase**: Post-MVP

```rust
pub fn new(code: &str, minor_unit_scale: u8, symbol: &str) -> Result<Self> {
    if code.len() != 3 {
        return Err(CurrencyError::InvalidCurrencyCode(code.to_string()).into());
    }
    
    // Additional validations to consider:
    if !code.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(CurrencyError::InvalidCurrencyFormat(code.to_string()).into());
    }
    
    if minor_unit_scale > 10 {
        return Err(CurrencyError::InvalidScale(minor_unit_scale).into());
    }
    
    // Could also validate against ISO 4217 list
    
    Ok(Self { /* ... */ })
}
```

### Money Arithmetic Operations
**Priority**: Medium  
**Phase**: Phase 2

```rust
use std::ops::{Add, Sub, Mul, Div};

impl Add for Money {
    type Output = Result<Money>;
    
    fn add(self, other: Money) -> Result<Money> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency.code().to_string(),
                right: other.currency.code().to_string(),
            }.into());
        }
        
        let result = self.amount_minor.checked_add(other.amount_minor)
            .ok_or(MoneyError::Overflow)?;
            
        Ok(Money {
            amount_minor: result,
            currency: self.currency,
        })
    }
}

impl Sub for Money {
    type Output = Result<Money>;
    
    fn sub(self, other: Money) -> Result<Money> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency.code().to_string(),
                right: other.currency.code().to_string(),
            }.into());
        }
        
        let result = self.amount_minor.checked_sub(other.amount_minor)
            .ok_or(MoneyError::Underflow)?;
            
        Ok(Money {
            amount_minor: result,
            currency: self.currency,
        })
    }
}

// Usage examples:
// let total = money1 + money2?;
// let difference = money1 - money2?;
```

### Money Overflow Protection
**Priority**: Low  
**Phase**: Phase 3

```rust
impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self> {
        let scale_factor = 10_i64.pow(currency.minor_unit_scale() as u32);
        let scaled = amount * Decimal::from(scale_factor);
        
        // Check for overflow before converting
        if scaled > Decimal::from(i64::MAX) || scaled < Decimal::from(i64::MIN) {
            return Err(MoneyError::Overflow.into());
        }
        
        let amount_minor = scaled.round().to_i64()
            .ok_or(MoneyError::ConversionError)?;
        
        Ok(Self { amount_minor, currency })
    }
}
```

### Additional Money Constructors
**Priority**: Low  
**Phase**: Phase 2

```rust
impl Money {
    /// Create money from string representation
    pub fn from_str(amount_str: &str, currency: Currency) -> Result<Self> {
        let amount = Decimal::from_str(amount_str)
            .map_err(|_| MoneyError::InvalidAmountString(amount_str.to_string()))?;
        Self::new(amount, currency)
    }
    
    /// Create money directly from minor units (for database loading)
    pub fn from_minor(amount_minor: i64, currency: Currency) -> Self {
        Self { amount_minor, currency }
    }
    
    /// Currency-specific constructors
    pub fn usd(amount: Decimal) -> Self {
        Self::new(amount, Currency::usd())
    }
    
    pub fn btc_satoshi(satoshi: i64) -> Self {
        Self::from_minor(satoshi, Currency::btc())
    }
}
```

## 🏦 Account Enhancements

### Account Builder Pattern
**Priority**: Medium  
**Phase**: Phase 2

```rust
pub struct AccountBuilder {
    name: String,
    account_type: AccountType,
    parent_id: Option<i64>,
    currency: Currency,
    description: Option<String>,
}

impl AccountBuilder {
    pub fn new(name: impl Into<String>, account_type: AccountType) -> Self {
        Self {
            name: name.into(),
            account_type,
            parent_id: None,
            currency: Currency::eur(), // Default
            description: None,
        }
    }
    
    pub fn parent(mut self, parent_id: i64) -> Self {
        self.parent_id = Some(parent_id);
        self
    }
    
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = currency;
        self
    }
    
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    pub fn build(self) -> Account {
        Account {
            id: None,
            name: self.name,
            account_type: self.account_type,
            parent_id: self.parent_id,
            currency: self.currency,
            description: self.description,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

// Usage:
// let account = AccountBuilder::new("Compte Courant", AccountType::Asset)
//     .parent(bourso_bank_id)
//     .description("Main checking account")
//     .build();
```

### Account Path Display
**Priority**: Low  
**Phase**: Phase 2

```rust
impl Account {
    /// Get full hierarchical path (e.g., "BoursoBank → Compte Courant")
    pub async fn full_path(&self, repository: &AccountRepository) -> Result<String> {
        let mut path = vec![self.name.clone()];
        let mut current_parent = self.parent_id;
        
        while let Some(parent_id) = current_parent {
            let parent = repository.get_by_id(parent_id).await?;
            path.insert(0, parent.name.clone());
            current_parent = parent.parent_id;
        }
        
        Ok(path.join(" → "))
    }
}
```

## 💸 Transaction Enhancements

### Transaction Builder with Validation
**Priority**: High  
**Phase**: Phase 1 (could be added soon)

```rust
pub struct TransactionBuilder {
    description: String,
    date: Option<NaiveDate>,
    reference: Option<String>,
    entries: Vec<EntryInput>,
    tags: Vec<String>,
    notes: Option<String>,
}

#[derive(Debug)]
pub struct EntryInput {
    pub account_id: i64,
    pub amount: Money,
    pub entry_type: EntryType,
    pub description: Option<String>,
}

impl TransactionBuilder {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            date: None,
            reference: None,
            entries: Vec::new(),
            tags: Vec::new(),
            notes: None,
        }
    }
    
    pub fn date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }
    
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }
    
    pub fn debit(mut self, account_id: i64, amount: Money) -> Self {
        self.entries.push(EntryInput {
            account_id,
            amount,
            entry_type: EntryType::Debit,
            description: None,
        });
        self
    }
    
    pub fn credit(mut self, account_id: i64, amount: Money) -> Self {
        self.entries.push(EntryInput {
            account_id,
            amount,
            entry_type: EntryType::Credit,
            description: None,
        });
        self
    }
    
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    pub fn build(self) -> Result<Transaction> {
        // Validate double-entry
        self.validate_double_entry()?;
        
        let transaction_date = self.date.unwrap_or_else(|| Local::now().date_naive());
        
        Ok(Transaction {
            id: None,
            description: self.description,
            reference: self.reference,
            transaction_date,
            created_at: Utc::now(),
            tags: if self.tags.is_empty() { None } else { Some(serde_json::to_string(&self.tags)?) },
            notes: self.notes,
            entries: self.entries.into_iter().map(|input| TransactionEntry {
                id: None,
                transaction_id: 0, // Will be set by repository
                account_id: input.account_id,
                amount: input.amount,
                entry_type: input.entry_type,
                description: input.description,
                created_at: Utc::now(),
            }).collect(),
        })
    }
    
    fn validate_double_entry(&self) -> Result<()> {
        if self.entries.len() < 2 {
            return Err(ValidationError::InsufficientEntries.into());
        }
        
        // Group by currency and validate each currency separately
        let mut currency_totals: HashMap<String, (i64, i64)> = HashMap::new(); // (debits, credits)
        
        for entry in &self.entries {
            let currency_code = entry.amount.currency().code();
            let (debits, credits) = currency_totals.entry(currency_code.to_string()).or_insert((0, 0));
            
            match entry.entry_type {
                EntryType::Debit => *debits += entry.amount.amount_minor(),
                EntryType::Credit => *credits += entry.amount.amount_minor(),
            }
        }
        
        // Check each currency is balanced
        for (currency, (debits, credits)) in currency_totals {
            if debits != credits {
                return Err(ValidationError::UnbalancedTransaction { 
                    currency,
                    debits, 
                    credits 
                }.into());
            }
        }
        
        Ok(())
    }
}

// Usage:
// let transaction = TransactionBuilder::new("Salary payment")
//     .date(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
//     .reference("PAY-2025-01")
//     .credit(salary_account_id, Money::eur(dec!(2500.00)))
//     .debit(bank_account_id, Money::eur(dec!(2500.00)))
//     .tag("salary")
//     .tag("monthly")
//     .build()?;
```

## 🛠️ General Code Quality Enhancements

### Additional Error Types
**Priority**: Low  
**Phase**: Phase 2

```rust
#[derive(Error, Debug)]
pub enum MoneyError {
    #[error("Arithmetic overflow")]
    Overflow,
    
    #[error("Arithmetic underflow")]
    Underflow,
    
    #[error("Currency mismatch: {left} != {right}")]
    CurrencyMismatch { left: String, right: String },
    
    #[error("Invalid amount string: {0}")]
    InvalidAmountString(String),
    
    #[error("Conversion error")]
    ConversionError,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Transaction must have at least 2 entries")]
    InsufficientEntries,
    
    #[error("Transaction is not balanced for {currency}: debits {debits} != credits {credits}")]
    UnbalancedTransaction { currency: String, debits: i64, credits: i64 },
    
    #[error("Account hierarchy too deep (max 5 levels)")]
    HierarchyTooDeep,
    
    #[error("Duplicate account name '{name}' under parent {parent_id}")]
    DuplicateAccountName { name: String, parent_id: Option<i64> },
}
```

### Testing Utilities
**Priority**: Medium  
**Phase**: Phase 1 (when adding tests)

```rust
#[cfg(test)]
pub mod test_utils {
    use super::*;
    
    impl Currency {
        pub fn test_currency() -> Self {
            Self::new("TST", 2, "T").unwrap()
        }
    }
    
    impl Money {
        pub fn test_money(amount: i64) -> Self {
            Self::from_minor(amount, Currency::test_currency())
        }
    }
    
    impl Account {
        pub fn test_account(name: &str, account_type: AccountType) -> Self {
            Self {
                id: None,
                name: name.to_string(),
                account_type,
                parent_id: None,
                currency: Currency::test_currency(),
                description: None,
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        }
    }
}
```

## 📋 Implementation Priority

1. **Phase 1 Extensions**: Transaction Builder (high value for MVP)
2. **Phase 2**: Money arithmetic, Account Builder, enhanced error types
3. **Phase 3**: Advanced validation, testing utilities
4. **Future**: Performance optimizations, additional currency support

## 🏦 Account Service Enhancements

### Account Validation Improvements
**Priority**: High  
**Phase**: Phase 1 (MVP completion)

```rust
impl AccountService {
    pub async fn create_account(
        &self,
        name: String,
        account_type: AccountType,
        parent_id: Option<i64>,
        currency: Currency,
    ) -> Result<Account> {
        // 1. Validate parent exists if specified
        if let Some(parent_id) = parent_id {
            let _parent = self.repository.get_by_id(parent_id).await
                .map_err(|_| WalletError::ParentAccountNotFound(parent_id))?;
        }

        // 2. Validate account name
        if name.trim().is_empty() {
            return Err(WalletError::InvalidAccountName("Name cannot be empty".to_string()));
        }
        
        if name.len() > 255 {
            return Err(WalletError::InvalidAccountName("Name too long (max 255 characters)".to_string()));
        }

        // 3. Check name uniqueness within parent
        if self.repository.name_exists_under_parent(&name, parent_id).await? {
            return Err(WalletError::DuplicateAccountName { 
                name: name.clone(), 
                parent_id 
            });
        }

        // 4. Validate hierarchy depth (max 5 levels)
        if let Some(parent_id) = parent_id {
            let depth = self.calculate_account_depth(parent_id).await?;
            if depth >= 5 {
                return Err(WalletError::HierarchyTooDeep);
            }
        }

        // Rest of creation logic...
    }
    
    async fn calculate_account_depth(&self, account_id: i64) -> Result<u8> {
        let mut depth = 1;
        let mut current_id = Some(account_id);
        
        while let Some(id) = current_id {
            let account = self.repository.get_by_id(id).await?;
            current_id = account.parent_id;
            depth += 1;
            
            if depth > 10 { // Safety check to prevent infinite loops
                return Err(WalletError::HierarchyTooDeep);
            }
        }
        
        Ok(depth)
    }
}
```

### Account Repository Extensions
**Priority**: High  
**Phase**: Phase 1 (MVP completion)

```rust
impl AccountRepository {
    pub async fn name_exists_under_parent(&self, name: &str, parent_id: Option<i64>) -> Result<bool> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM accounts WHERE name = ?1 AND parent_id IS ?2 AND is_active = true"
        )
        .bind(name)
        .bind(parent_id)
        .fetch_one(&self.db.pool)
        .await?;
        
        Ok(count.0 > 0)
    }
    
    pub async fn get_by_parent(&self, parent_id: Option<i64>) -> Result<Vec<Account>> {
        let accounts = sqlx::query_as(
            "SELECT id, name, account_type, parent_id, currency, description, is_active, created_at, updated_at 
             FROM accounts WHERE parent_id IS ?1 AND is_active = true ORDER BY name"
        )
        .bind(parent_id)
        .fetch_all(&self.db.pool)
        .await?;
        
        Ok(accounts)
    }
    
    pub async fn update(&self, account: &Account) -> Result<Account> {
        sqlx::query(
            "UPDATE accounts SET name = ?1, account_type = ?2, parent_id = ?3, currency = ?4, 
             description = ?5, is_active = ?6, updated_at = ?7 WHERE id = ?8"
        )
        .bind(&account.name)
        .bind(&account.account_type)
        .bind(&account.parent_id)
        .bind(account.currency.code())
        .bind(&account.description)
        .bind(account.is_active)
        .bind(Utc::now())
        .bind(account.id)
        .execute(&self.db.pool)
        .await?;
        
        self.get_by_id(account.id.unwrap()).await
    }
    
    pub async fn soft_delete(&self, id: i64) -> Result<()> {
        sqlx::query("UPDATE accounts SET is_active = false, updated_at = ?1 WHERE id = ?2")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.db.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn get_account_tree(&self, root_id: Option<i64>) -> Result<Vec<Account>> {
        // Recursive CTE to get entire account hierarchy
        let accounts = sqlx::query_as(
            r#"
            WITH RECURSIVE account_tree AS (
                SELECT id, name, account_type, parent_id, currency, description, is_active, created_at, updated_at, 0 as level
                FROM accounts 
                WHERE parent_id IS ?1 AND is_active = true
                
                UNION ALL
                
                SELECT a.id, a.name, a.account_type, a.parent_id, a.currency, a.description, a.is_active, a.created_at, a.updated_at, at.level + 1
                FROM accounts a
                JOIN account_tree at ON a.parent_id = at.id
                WHERE a.is_active = true AND at.level < 5
            )
            SELECT id, name, account_type, parent_id, currency, description, is_active, created_at, updated_at
            FROM account_tree 
            ORDER BY level, name
            "#
        )
        .bind(root_id)
        .fetch_all(&self.db.pool)
        .await?;
        
        Ok(accounts)
    }
}
```

### Missing Error Types
**Priority**: High  
**Phase**: Phase 1 (MVP completion)

```rust
#[derive(Error, Debug)]
pub enum WalletError {
    // ... existing errors
    #[error("Parent account with ID {0} not found")]
    ParentAccountNotFound(i64),
    
    #[error("Invalid account name: {0}")]
    InvalidAccountName(String),
    
    #[error("Duplicate account name '{name}' under parent {parent_id:?}")]
    DuplicateAccountName { name: String, parent_id: Option<i64> },
    
    #[error("Account hierarchy too deep (max 5 levels)")]
    HierarchyTooDeep,
    
    #[error("Cannot delete account with ID {0}: has child accounts")]
    AccountHasChildren(i64),
    
    #[error("Cannot delete account with ID {0}: has transactions")]
    AccountHasTransactions(i64),
}
```

### Account Service Complete Implementation
**Priority**: High  
**Phase**: Phase 1 (MVP completion)

```rust
impl AccountService {
    pub async fn update_account(&self, account: &Account) -> Result<Account> {
        // Validate account exists
        let existing = self.repository.get_by_id(account.id.unwrap()).await?;
        
        // Validate name uniqueness if name changed
        if existing.name != account.name {
            if self.repository.name_exists_under_parent(&account.name, account.parent_id).await? {
                return Err(WalletError::DuplicateAccountName { 
                    name: account.name.clone(), 
                    parent_id: account.parent_id 
                });
            }
        }
        
        // Validate parent change doesn't create cycles
        if existing.parent_id != account.parent_id {
            if let Some(new_parent_id) = account.parent_id {
                self.validate_no_circular_reference(account.id.unwrap(), new_parent_id).await?;
            }
        }
        
        self.repository.update(account).await
    }
    
    pub async fn deactivate_account(&self, id: i64) -> Result<()> {
        // Check for child accounts
        let children = self.repository.get_by_parent(Some(id)).await?;
        if !children.is_empty() {
            return Err(WalletError::AccountHasChildren(id));
        }
        
        // TODO: Check for transactions when transaction system is implemented
        
        self.repository.soft_delete(id).await
    }
    
    pub async fn get_children(&self, parent_id: i64) -> Result<Vec<Account>> {
        self.repository.get_by_parent(Some(parent_id)).await
    }
    
    pub async fn get_account_hierarchy(&self, root_id: Option<i64>) -> Result<Vec<Account>> {
        self.repository.get_account_tree(root_id).await
    }
    
    async fn validate_no_circular_reference(&self, account_id: i64, new_parent_id: i64) -> Result<()> {
        let mut current_id = Some(new_parent_id);
        
        while let Some(id) = current_id {
            if id == account_id {
                return Err(WalletError::InvalidAccountName("Circular reference detected".to_string()));
            }
            
            let account = self.repository.get_by_id(id).await?;
            current_id = account.parent_id;
        }
        
        Ok(())
    }
    
    pub async fn calculate_balance(&self, account_id: i64) -> Result<Money> {
        // This will be implemented when transaction system is ready
        // Should include:
        // 1. Sum all transaction entries for this account
        // 2. Apply debit/credit rules based on account type
        // 3. Include child account balances
        todo!("Implement after transaction repository is complete")
    }
}
```

### Account Service Tests
**Priority**: Medium  
**Phase**: Phase 1 (MVP completion)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_account_validates_parent_exists() {
        // Test parent validation
    }
    
    #[tokio::test]
    async fn test_create_account_validates_name_uniqueness() {
        // Test name uniqueness within parent
    }
    
    #[tokio::test]
    async fn test_create_account_validates_hierarchy_depth() {
        // Test max 5 levels depth
    }
    
    #[tokio::test]
    async fn test_update_account_prevents_circular_reference() {
        // Test circular reference prevention
    }
    
    #[tokio::test]
    async fn test_deactivate_account_with_children_fails() {
        // Test cannot delete account with children
    }
}
```

---

*This document should be updated as new enhancement ideas emerge during development.*