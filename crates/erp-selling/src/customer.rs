use erp_core::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub company_id: Uuid,
    pub customer_id: Uuid,
    pub customer_name: String,
    pub customer_group: String,
    pub territory: String,
    pub default_currency: Currency,
    pub credit_limit: Option<Decimal>,
}

impl Customer {
    pub fn new(
        company_id: Uuid,
        customer_name: impl Into<String>,
        default_currency: Currency,
    ) -> Self {
        Self {
            company_id,
            customer_id: Uuid::new_v4(),
            customer_name: customer_name.into(),
            customer_group: "All Customer Groups".into(),
            territory: "All Territories".into(),
            default_currency,
            credit_limit: None,
        }
    }
}
