use thiserror::Error;

pub type Result<T> = std::result::Result<T, WalletError>;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error(transparent)]
    CurrencyError(#[from] CurrencyError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),
}

#[derive(Error, Debug)]
pub enum CurrencyError {
    #[error("Invalid currency code: {0}")]
    InvalidCurrencyCode(String),
}
