use crate::error::StockError;
use crate::stock_ledger::StockLedgerEntry;
use chrono::{DateTime, Utc};
use erp_core::{CancelledState, DocState, DraftState, SubmittedState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StockEntryType {
    MaterialReceipt,
    MaterialIssue,
    MaterialTransfer,
    Manufacture,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockEntryItem {
    pub item_code: String,
    pub source_warehouse_id: Option<Uuid>,
    pub target_warehouse_id: Option<Uuid>,
    pub qty: Decimal,
    pub basic_rate: Decimal,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockEntry<S: DocState> {
    pub company_id: Uuid,
    pub entry_id: Uuid,
    pub entry_type: StockEntryType,
    pub naming_series: String,
    pub posting_datetime: DateTime<Utc>,
    pub items: Vec<StockEntryItem>,
    pub total_amount: Decimal,
    #[serde(skip)]
    pub _state: PhantomData<S>,
}

impl StockEntry<DraftState> {
    pub fn new(
        company_id: Uuid,
        entry_type: StockEntryType,
        naming_series: String,
        posting_datetime: DateTime<Utc>,
    ) -> Self {
        Self {
            company_id,
            entry_id: Uuid::new_v4(),
            entry_type,
            naming_series,
            posting_datetime,
            items: Vec::new(),
            total_amount: Decimal::ZERO,
            _state: PhantomData,
        }
    }

    pub fn add_item(
        &mut self,
        item_code: impl Into<String>,
        source_warehouse_id: Option<Uuid>,
        target_warehouse_id: Option<Uuid>,
        qty: Decimal,
        basic_rate: Decimal,
    ) {
        let amount = qty * basic_rate;
        self.items.push(StockEntryItem {
            item_code: item_code.into(),
            source_warehouse_id,
            target_warehouse_id,
            qty,
            basic_rate,
            amount,
        });
        self.total_amount = self.items.iter().map(|i| i.amount).sum();
    }

    pub fn submit(self) -> Result<(StockEntry<SubmittedState>, Vec<StockLedgerEntry>), StockError> {
        if self.items.is_empty() {
            return Err(StockError::Core(erp_core::CoreError::ValidationError(
                "Stock entry must contain at least one item".into(),
            )));
        }

        let now = Utc::now();
        let mut ledger_entries = Vec::new();

        for item in &self.items {
            match self.entry_type {
                StockEntryType::MaterialReceipt => {
                    let target = item.target_warehouse_id.ok_or_else(|| {
                        StockError::Core(erp_core::CoreError::ValidationError(
                            "Target warehouse required for Material Receipt".into(),
                        ))
                    })?;
                    ledger_entries.push(StockLedgerEntry {
                        company_id: self.company_id,
                        sle_id: Uuid::new_v4(),
                        item_code: item.item_code.clone(),
                        warehouse_id: target,
                        posting_datetime: self.posting_datetime,
                        voucher_type: "Stock Entry".to_string(),
                        voucher_no: self.naming_series.clone(),
                        actual_qty: item.qty, // positive
                        qty_after_transaction: item.qty,
                        incoming_rate: item.basic_rate,
                        valuation_rate: item.basic_rate,
                        stock_value: item.amount,
                        stock_value_difference: item.amount,
                        batch_no: None,
                        serial_no: None,
                        is_cancelled: false,
                        created_at: now,
                    });
                }
                StockEntryType::MaterialIssue => {
                    let source = item.source_warehouse_id.ok_or_else(|| {
                        StockError::Core(erp_core::CoreError::ValidationError(
                            "Source warehouse required for Material Issue".into(),
                        ))
                    })?;
                    ledger_entries.push(StockLedgerEntry {
                        company_id: self.company_id,
                        sle_id: Uuid::new_v4(),
                        item_code: item.item_code.clone(),
                        warehouse_id: source,
                        posting_datetime: self.posting_datetime,
                        voucher_type: "Stock Entry".to_string(),
                        voucher_no: self.naming_series.clone(),
                        actual_qty: -item.qty, // negative for issue
                        qty_after_transaction: Decimal::ZERO,
                        incoming_rate: Decimal::ZERO,
                        valuation_rate: item.basic_rate,
                        stock_value: Decimal::ZERO,
                        stock_value_difference: -item.amount,
                        batch_no: None,
                        serial_no: None,
                        is_cancelled: false,
                        created_at: now,
                    });
                }
                StockEntryType::MaterialTransfer => {
                    let source = item.source_warehouse_id.ok_or_else(|| {
                        StockError::Core(erp_core::CoreError::ValidationError(
                            "Source warehouse required for Transfer".into(),
                        ))
                    })?;
                    let target = item.target_warehouse_id.ok_or_else(|| {
                        StockError::Core(erp_core::CoreError::ValidationError(
                            "Target warehouse required for Transfer".into(),
                        ))
                    })?;
                    if source == target {
                        return Err(StockError::SameSourceAndTargetWarehouse(source.to_string()));
                    }

                    // Outward from Source
                    ledger_entries.push(StockLedgerEntry {
                        company_id: self.company_id,
                        sle_id: Uuid::new_v4(),
                        item_code: item.item_code.clone(),
                        warehouse_id: source,
                        posting_datetime: self.posting_datetime,
                        voucher_type: "Stock Entry".to_string(),
                        voucher_no: self.naming_series.clone(),
                        actual_qty: -item.qty,
                        qty_after_transaction: Decimal::ZERO,
                        incoming_rate: Decimal::ZERO,
                        valuation_rate: item.basic_rate,
                        stock_value: Decimal::ZERO,
                        stock_value_difference: -item.amount,
                        batch_no: None,
                        serial_no: None,
                        is_cancelled: false,
                        created_at: now,
                    });

                    // Inward to Target
                    ledger_entries.push(StockLedgerEntry {
                        company_id: self.company_id,
                        sle_id: Uuid::new_v4(),
                        item_code: item.item_code.clone(),
                        warehouse_id: target,
                        posting_datetime: self.posting_datetime,
                        voucher_type: "Stock Entry".to_string(),
                        voucher_no: self.naming_series.clone(),
                        actual_qty: item.qty,
                        qty_after_transaction: item.qty,
                        incoming_rate: item.basic_rate,
                        valuation_rate: item.basic_rate,
                        stock_value: item.amount,
                        stock_value_difference: item.amount,
                        batch_no: None,
                        serial_no: None,
                        is_cancelled: false,
                        created_at: now,
                    });
                }
                StockEntryType::Manufacture => {
                    // Similar to transfer/production
                }
            }
        }

        let submitted = StockEntry {
            company_id: self.company_id,
            entry_id: self.entry_id,
            entry_type: self.entry_type,
            naming_series: self.naming_series,
            posting_datetime: self.posting_datetime,
            items: self.items,
            total_amount: self.total_amount,
            _state: PhantomData,
        };

        Ok((submitted, ledger_entries))
    }
}

impl StockEntry<SubmittedState> {
    pub fn cancel(self) -> StockEntry<CancelledState> {
        StockEntry {
            company_id: self.company_id,
            entry_id: self.entry_id,
            entry_type: self.entry_type,
            naming_series: self.naming_series,
            posting_datetime: self.posting_datetime,
            items: self.items,
            total_amount: self.total_amount,
            _state: PhantomData,
        }
    }
}
