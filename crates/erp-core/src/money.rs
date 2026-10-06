use crate::error::{CoreError, DomainResult};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CAD,
    AUD,
    CHF,
    CNY,
    INR,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::CAD => "CAD",
            Currency::AUD => "AUD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::INR => "INR",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "USD" => Some(Currency::USD),
            "EUR" => Some(Currency::EUR),
            "GBP" => Some(Currency::GBP),
            "JPY" => Some(Currency::JPY),
            "CAD" => Some(Currency::CAD),
            "AUD" => Some(Currency::AUD),
            "CHF" => Some(Currency::CHF),
            "CNY" => Some(Currency::CNY),
            "INR" => Some(Currency::INR),
            _ => None,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    pub fn multiply_by(&self, factor: Decimal) -> Self {
        Self {
            amount: self.amount * factor,
            currency: self.currency,
        }
    }

    pub fn round_dp(&self, dp: u32) -> Self {
        Self {
            amount: self.amount.round_dp(dp),
            currency: self.currency,
        }
    }
}

impl Add for Money {
    type Output = DomainResult<Money>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            return Err(CoreError::CurrencyMismatch {
                expected: self.currency.to_string(),
                actual: rhs.currency.to_string(),
            });
        }
        Ok(Money {
            amount: self.amount + rhs.amount,
            currency: self.currency,
        })
    }
}

impl Sub for Money {
    type Output = DomainResult<Money>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            return Err(CoreError::CurrencyMismatch {
                expected: self.currency.to_string(),
                actual: rhs.currency.to_string(),
            });
        }
        Ok(Money {
            amount: self.amount - rhs.amount,
            currency: self.currency,
        })
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.currency, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_addition() {
        let m1 = Money::new(Decimal::new(1050, 2), Currency::USD); // 10.50 USD
        let m2 = Money::new(Decimal::new(2025, 2), Currency::USD); // 20.25 USD
        let sum = (m1 + m2).unwrap();
        assert_eq!(sum.amount, Decimal::new(3075, 2));
    }

    #[test]
    fn test_currency_mismatch() {
        let m1 = Money::new(Decimal::TEN, Currency::USD);
        let m2 = Money::new(Decimal::TEN, Currency::EUR);
        assert!((m1 + m2).is_err());
    }
}
