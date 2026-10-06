use crate::error::SellingError;
use chrono::NaiveDate;
use erp_core::{CancelledState, Currency, DocState, DraftState, SubmittedState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrderLine {
    pub line_id: Uuid,
    pub item_code: String,
    pub warehouse_id: Uuid,
    pub qty: Decimal,
    pub delivered_qty: Decimal,
    pub billed_qty: Decimal,
    pub rate: Decimal,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrder<S: DocState> {
    pub company_id: Uuid,
    pub order_id: Uuid,
    pub naming_series: String,
    pub customer_id: Uuid,
    pub order_date: NaiveDate,
    pub delivery_date: NaiveDate,
    pub currency: Currency,
    pub lines: Vec<SalesOrderLine>,
    pub net_total: Decimal,
    pub grand_total: Decimal,
    #[serde(skip)]
    pub _state: PhantomData<S>,
}

impl SalesOrder<DraftState> {
    pub fn new(
        company_id: Uuid,
        naming_series: String,
        customer_id: Uuid,
        order_date: NaiveDate,
        delivery_date: NaiveDate,
        currency: Currency,
    ) -> Self {
        Self {
            company_id,
            order_id: Uuid::new_v4(),
            naming_series,
            customer_id,
            order_date,
            delivery_date,
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
        self.lines.push(SalesOrderLine {
            line_id: Uuid::new_v4(),
            item_code: item_code.into(),
            warehouse_id,
            qty,
            delivered_qty: Decimal::ZERO,
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

    pub fn submit(self) -> Result<SalesOrder<SubmittedState>, SellingError> {
        if self.lines.is_empty() {
            return Err(SellingError::EmptyOrder);
        }
        if self.grand_total <= Decimal::ZERO {
            return Err(SellingError::InvalidGrandTotal);
        }

        Ok(SalesOrder {
            company_id: self.company_id,
            order_id: self.order_id,
            naming_series: self.naming_series,
            customer_id: self.customer_id,
            order_date: self.order_date,
            delivery_date: self.delivery_date,
            currency: self.currency,
            lines: self.lines,
            net_total: self.net_total,
            grand_total: self.grand_total,
            _state: PhantomData,
        })
    }
}

impl SalesOrder<SubmittedState> {
    pub fn cancel(self) -> SalesOrder<CancelledState> {
        SalesOrder {
            company_id: self.company_id,
            order_id: self.order_id,
            naming_series: self.naming_series,
            customer_id: self.customer_id,
            order_date: self.order_date,
            delivery_date: self.delivery_date,
            currency: self.currency,
            lines: self.lines,
            net_total: self.net_total,
            grand_total: self.grand_total,
            _state: PhantomData,
        }
    }
}
