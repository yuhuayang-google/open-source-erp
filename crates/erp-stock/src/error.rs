use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum StockError {
    #[error("Negative stock violation: Item '{item_code}' in warehouse '{warehouse_id}' has {available} available, cannot issue {requested}")]
    NegativeStock {
        item_code: String,
        warehouse_id: String,
        available: Decimal,
        requested: Decimal,
    },

    #[error("Valuation rate must be non-negative: found {0}")]
    InvalidValuationRate(Decimal),

    #[error("Cannot transfer to identical warehouse '{0}'")]
    SameSourceAndTargetWarehouse(String),

    #[error("Core error: {0}")]
    Core(#[from] erp_core::CoreError),
}
