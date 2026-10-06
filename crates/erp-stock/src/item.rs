use erp_core::Uom;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuationMethod {
    FIFO,
    MovingAverage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub company_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub item_group: String,
    pub stock_uom: Uom,
    pub is_stock_item: bool,
    pub valuation_method: ValuationMethod,
    pub standard_rate: Decimal,
    pub safety_stock: Decimal,
}

impl Item {
    pub fn new(
        company_id: Uuid,
        item_code: impl Into<String>,
        item_name: impl Into<String>,
        stock_uom: Uom,
        valuation_method: ValuationMethod,
        standard_rate: Decimal,
    ) -> Self {
        Self {
            company_id,
            item_code: item_code.into(),
            item_name: item_name.into(),
            item_group: "All Item Groups".into(),
            stock_uom,
            is_stock_item: true,
            valuation_method,
            standard_rate,
            safety_stock: Decimal::ZERO,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub company_id: Uuid,
    pub warehouse_id: Uuid,
    pub warehouse_name: String,
    pub is_group: bool,
    pub parent_warehouse_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
}

impl Warehouse {
    pub fn new(company_id: Uuid, warehouse_name: impl Into<String>) -> Self {
        Self {
            company_id,
            warehouse_id: Uuid::new_v4(),
            warehouse_name: warehouse_name.into(),
            is_group: false,
            parent_warehouse_id: None,
            account_id: None,
        }
    }
}
