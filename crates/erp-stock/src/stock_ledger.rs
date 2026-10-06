use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLedgerEntry {
    pub company_id: Uuid,
    pub sle_id: Uuid,
    pub item_code: String,
    pub warehouse_id: Uuid,
    pub posting_datetime: DateTime<Utc>,
    pub voucher_type: String, // 'Stock Entry', 'Delivery Note', 'Purchase Receipt'
    pub voucher_no: String,
    pub actual_qty: Decimal, // Positive = inbound, Negative = outbound
    pub qty_after_transaction: Decimal,
    pub incoming_rate: Decimal,
    pub valuation_rate: Decimal,
    pub stock_value: Decimal,
    pub stock_value_difference: Decimal,
    pub batch_no: Option<String>,
    pub serial_no: Option<String>,
    pub is_cancelled: bool,
    pub created_at: DateTime<Utc>,
}
