use erp_core::Currency;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub company_id: Uuid,
    pub supplier_id: Uuid,
    pub supplier_name: String,
    pub supplier_group: String,
    pub default_currency: Currency,
    pub payment_terms: Option<String>,
}

impl Supplier {
    pub fn new(
        company_id: Uuid,
        supplier_name: impl Into<String>,
        default_currency: Currency,
    ) -> Self {
        Self {
            company_id,
            supplier_id: Uuid::new_v4(),
            supplier_name: supplier_name.into(),
            supplier_group: "All Supplier Groups".into(),
            default_currency,
            payment_terms: None,
        }
    }
}
