use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AccountingError {
    #[error("Debit and credit entries cannot be negative")]
    NegativeAmount,

    #[error("Single entry line cannot have both debit and credit greater than zero")]
    SimultaneousDebitAndCredit,

    #[error("Unbalanced general ledger entry: Debits={debit}, Credits={credit}, Difference={difference}")]
    UnbalancedEntry {
        debit: Decimal,
        credit: Decimal,
        difference: Decimal,
    },

    #[error("No ledger entries provided for posting")]
    EmptyPostingRequest,

    #[error("Currency mismatch in GL entry")]
    CurrencyMismatch,

    #[error("Invalid state transition for invoice: {0}")]
    InvalidState(String),

    #[error("Core error: {0}")]
    Core(#[from] erp_core::CoreError),
}
