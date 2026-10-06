use erp_core::Currency;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RootType {
    Asset,
    Liability,
    Equity,
    Income,
    Expense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportType {
    BalanceSheet,
    ProfitAndLoss,
}

impl RootType {
    pub fn report_type(&self) -> ReportType {
        match self {
            RootType::Asset | RootType::Liability | RootType::Equity => ReportType::BalanceSheet,
            RootType::Income | RootType::Expense => ReportType::ProfitAndLoss,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub company_id: Uuid,
    pub account_id: Uuid,
    pub account_name: String,
    pub account_number: Option<String>,
    pub parent_account_id: Option<Uuid>,
    pub root_type: RootType,
    pub report_type: ReportType,
    pub is_group: bool,
    pub currency: Currency,
}

impl Account {
    pub fn new_ledger(
        company_id: Uuid,
        account_name: impl Into<String>,
        root_type: RootType,
        currency: Currency,
    ) -> Self {
        Self {
            company_id,
            account_id: Uuid::new_v4(),
            account_name: account_name.into(),
            account_number: None,
            parent_account_id: None,
            root_type,
            report_type: root_type.report_type(),
            is_group: false,
            currency,
        }
    }

    pub fn new_group(
        company_id: Uuid,
        account_name: impl Into<String>,
        root_type: RootType,
        currency: Currency,
    ) -> Self {
        Self {
            company_id,
            account_id: Uuid::new_v4(),
            account_name: account_name.into(),
            account_number: None,
            parent_account_id: None,
            root_type,
            report_type: root_type.report_type(),
            is_group: true,
            currency,
        }
    }
}
