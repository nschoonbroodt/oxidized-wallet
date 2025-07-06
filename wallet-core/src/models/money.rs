use crate::errors::{CurrencyError, Result};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct Currency {
    code: String,
    minor_unit_scale: u8,
    symbol: String,
}

impl Currency {
    pub fn new(code: &str, minor_unit_scale: u8, symbol: &str) -> Result<Self> {
        if code.len() != 3 {
            return Err(CurrencyError::InvalidCurrencyCode(code.to_string()).into());
        };
        Ok(Self {
            code: code.to_string(),
            minor_unit_scale,
            symbol: symbol.to_string(),
        })
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn minor_unit_scale(&self) -> u8 {
        self.minor_unit_scale
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn from_code(code: &str) -> Result<Self> {
        match code.to_uppercase().as_str() {
            "EUR" => Ok(Self::eur()),
            "BTC" => Ok(Self::btc()),
            _ => Err(CurrencyError::InvalidCurrencyCode(code.to_string()).into()),
        }
    }

    pub fn eur() -> Self {
        Self::new("EUR", 2, "€").unwrap()
    }
    pub fn btc() -> Self {
        Self::new("BTC", 8, "₿").unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct Money {
    amount_minor: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        let scale_factor = 10_i64.pow(currency.minor_unit_scale() as u32);
        let amount_minor = (amount * Decimal::from(scale_factor))
            .round()
            .to_i64()
            .expect("Overflow here would indicate too much money ;-)");
        Self {
            amount_minor,
            currency,
        }
    }

    pub fn amount_minor(&self) -> i64 {
        self.amount_minor
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn to_decimal(&self) -> Decimal {
        // Convert amount_minor to Decimal based on currency's minor_unit_scale
        Decimal::new(self.amount_minor, self.currency.minor_unit_scale() as u32)
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount_minor: 0,
            currency,
        }
    }

    pub fn eur(amount: Decimal) -> Self {
        Self::new(amount, Currency::eur())
    }

    pub fn from_minor_units(amount_minor: i64, currency: Currency) -> Self {
        Self {
            amount_minor,
            currency,
        }
    }
}
