use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SellingError {
    #[error("Order has no lines")]
    EmptyOrder,

    #[error("Grand total must be greater than zero")]
    InvalidGrandTotal,

    #[error("Over-delivery violation: Item '{item_code}' ordered {ordered}, attempted to deliver {attempted}")]
    OverDelivery {
        item_code: String,
        ordered: rust_decimal::Decimal,
        attempted: rust_decimal::Decimal,
    },

    #[error("Core error: {0}")]
    Core(#[from] erp_core::CoreError),

    #[error("Stock error: {0}")]
    Stock(#[from] erp_stock::StockError),
}
