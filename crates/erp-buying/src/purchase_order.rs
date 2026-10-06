use chrono::NaiveDate;
use erp_core::{CancelledState, Currency, DocState, DraftState, SubmittedState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BuyingError {
    #[error("Purchase order has no items")]
    EmptyOrder,

    #[error("Total amount must be greater than zero")]
    InvalidTotal,

    #[error("Core error: {0}")]
    Core(#[from] erp_core::CoreError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderLine {
    pub line_id: Uuid,
    pub item_code: String,
    pub warehouse_id: Uuid,
    pub qty: Decimal,
    pub received_qty: Decimal,
    pub billed_qty: Decimal,
    pub rate: Decimal,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder<S: DocState> {
    pub company_id: Uuid,
    pub order_id: Uuid,
    pub naming_series: String,
    pub supplier_id: Uuid,
    pub order_date: NaiveDate,
    pub schedule_date: NaiveDate,
    pub currency: Currency,
    pub lines: Vec<PurchaseOrderLine>,
    pub net_total: Decimal,
    pub grand_total: Decimal,
    #[serde(skip)]
    pub _state: PhantomData<S>,
}

impl PurchaseOrder<DraftState> {
    pub fn new(
        company_id: Uuid,
        naming_series: String,
        supplier_id: Uuid,
        order_date: NaiveDate,
        schedule_date: NaiveDate,
        currency: Currency,
    ) -> Self {
        Self {
            company_id,
            order_id: Uuid::new_v4(),
            naming_series,
            supplier_id,
            order_date,
            schedule_date,
            currency,
            lines: Vec::new(),
            net_total: Decimal::ZERO,
            grand_total: Decimal::ZERO,
            _state: PhantomData,
        }
    }

    pub fn add_item(
        &mut self,
        item_code: impl Into<String>,
        warehouse_id: Uuid,
        qty: Decimal,
        rate: Decimal,
    ) {
        let amount = qty * rate;
        self.lines.push(PurchaseOrderLine {
            line_id: Uuid::new_v4(),
            item_code: item_code.into(),
            warehouse_id,
            qty,
            received_qty: Decimal::ZERO,
            billed_qty: Decimal::ZERO,
            rate,
            amount,
        });
        self.recalculate();
    }

    pub fn recalculate(&mut self) {
        self.net_total = self.lines.iter().map(|l| l.amount).sum();
        self.grand_total = self.net_total;
    }

    pub fn submit(self) -> Result<PurchaseOrder<SubmittedState>, BuyingError> {
        if self.lines.is_empty() {
            return Err(BuyingError::EmptyOrder);
        }
        if self.grand_total <= Decimal::ZERO {
            return Err(BuyingError::InvalidTotal);
        }

        Ok(PurchaseOrder {
            company_id: self.company_id,
            order_id: self.order_id,
            naming_series: self.naming_series,
            supplier_id: self.supplier_id,
            order_date: self.order_date,
            schedule_date: self.schedule_date,
            currency: self.currency,
            lines: self.lines,
            net_total: self.net_total,
            grand_total: self.grand_total,
            _state: PhantomData,
        })
    }
}

impl PurchaseOrder<SubmittedState> {
    pub fn cancel(self) -> PurchaseOrder<CancelledState> {
        PurchaseOrder {
            company_id: self.company_id,
            order_id: self.order_id,
            naming_series: self.naming_series,
            supplier_id: self.supplier_id,
            order_date: self.order_date,
            schedule_date: self.schedule_date,
            currency: self.currency,
            lines: self.lines,
            net_total: self.net_total,
            grand_total: self.grand_total,
            _state: PhantomData,
        }
    }
}
