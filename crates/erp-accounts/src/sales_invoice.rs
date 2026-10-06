use crate::error::AccountingError;
use crate::general_ledger::{GLEntryItem, GLPostingRequest};
use chrono::NaiveDate;
use erp_core::{CancelledState, Currency, DocState, DraftState, SubmittedState};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub item_id: Uuid,
    pub item_code: String,
    pub description: String,
    pub income_account_id: Uuid,
    pub qty: Decimal,
    pub rate: Decimal,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxItem {
    pub tax_account_id: Uuid,
    pub description: String,
    pub rate_percentage: Decimal,
    pub tax_amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesInvoice<S: DocState> {
    pub company_id: Uuid,
    pub invoice_id: Uuid,
    pub naming_series: String,
    pub customer_id: Uuid,
    pub receivable_account_id: Uuid,
    pub posting_date: NaiveDate,
    pub due_date: NaiveDate,
    pub currency: Currency,
    pub items: Vec<InvoiceItem>,
    pub taxes: Vec<TaxItem>,
    pub net_total: Decimal,
    pub total_tax: Decimal,
    pub grand_total: Decimal,
    pub outstanding_amount: Decimal,
    #[serde(skip)]
    pub _state: PhantomData<S>,
}

impl SalesInvoice<DraftState> {
    pub fn new(
        company_id: Uuid,
        naming_series: String,
        customer_id: Uuid,
        receivable_account_id: Uuid,
        posting_date: NaiveDate,
        due_date: NaiveDate,
        currency: Currency,
    ) -> Self {
        Self {
            company_id,
            invoice_id: Uuid::new_v4(),
            naming_series,
            customer_id,
            receivable_account_id,
            posting_date,
            due_date,
            currency,
            items: Vec::new(),
            taxes: Vec::new(),
            net_total: Decimal::ZERO,
            total_tax: Decimal::ZERO,
            grand_total: Decimal::ZERO,
            outstanding_amount: Decimal::ZERO,
            _state: PhantomData,
        }
    }

    pub fn add_item(
        &mut self,
        item_code: impl Into<String>,
        description: impl Into<String>,
        income_account_id: Uuid,
        qty: Decimal,
        rate: Decimal,
    ) {
        let amount = qty * rate;
        self.items.push(InvoiceItem {
            item_id: Uuid::new_v4(),
            item_code: item_code.into(),
            description: description.into(),
            income_account_id,
            qty,
            rate,
            amount,
        });
        self.recalculate();
    }

    pub fn add_tax(
        &mut self,
        tax_account_id: Uuid,
        description: impl Into<String>,
        rate_percentage: Decimal,
    ) {
        self.taxes.push(TaxItem {
            tax_account_id,
            description: description.into(),
            rate_percentage,
            tax_amount: Decimal::ZERO, // recalculated below
        });
        self.recalculate();
    }

    pub fn recalculate(&mut self) {
        self.net_total = self.items.iter().map(|item| item.amount).sum();

        let mut computed_tax = Decimal::ZERO;
        for tax in &mut self.taxes {
            tax.tax_amount = (self.net_total * tax.rate_percentage) / Decimal::new(100, 0);
            computed_tax += tax.tax_amount;
        }

        self.total_tax = computed_tax;
        self.grand_total = self.net_total + self.total_tax;
        self.outstanding_amount = self.grand_total;
    }

    /// Submits the invoice and produces the double-entry GL Posting Request
    pub fn submit(
        self,
    ) -> Result<(SalesInvoice<SubmittedState>, GLPostingRequest), AccountingError> {
        if self.items.is_empty() {
            return Err(AccountingError::InvalidState(
                "Cannot submit an invoice with zero items".into(),
            ));
        }
        if self.grand_total <= Decimal::ZERO {
            return Err(AccountingError::InvalidState(
                "Grand total must be positive".into(),
            ));
        }

        // Construct GL entries:
        // Debit: Accounts Receivable (grand_total)
        // Credit: Each Item's Income Account (item.amount)
        // Credit: Tax Accounts (tax.tax_amount)
        let mut gl_entries = Vec::new();

        // 1. Debit Accounts Receivable
        gl_entries.push(GLEntryItem {
            account_id: self.receivable_account_id,
            cost_center_id: None,
            party_type: Some("Customer".into()),
            party_id: Some(self.customer_id),
            debit: self.grand_total,
            credit: Decimal::ZERO,
        });

        // 2. Credit Income Accounts
        for item in &self.items {
            gl_entries.push(GLEntryItem {
                account_id: item.income_account_id,
                cost_center_id: None,
                party_type: None,
                party_id: None,
                debit: Decimal::ZERO,
                credit: item.amount,
            });
        }

        // 3. Credit Tax Accounts
        for tax in &self.taxes {
            if tax.tax_amount > Decimal::ZERO {
                gl_entries.push(GLEntryItem {
                    account_id: tax.tax_account_id,
                    cost_center_id: None,
                    party_type: None,
                    party_id: None,
                    debit: Decimal::ZERO,
                    credit: tax.tax_amount,
                });
            }
        }

        let gl_request = GLPostingRequest {
            company_id: self.company_id,
            posting_date: self.posting_date,
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: self.naming_series.clone(),
            currency: self.currency,
            entries: gl_entries,
        };

        let submitted = SalesInvoice {
            company_id: self.company_id,
            invoice_id: self.invoice_id,
            naming_series: self.naming_series,
            customer_id: self.customer_id,
            receivable_account_id: self.receivable_account_id,
            posting_date: self.posting_date,
            due_date: self.due_date,
            currency: self.currency,
            items: self.items,
            taxes: self.taxes,
            net_total: self.net_total,
            total_tax: self.total_tax,
            grand_total: self.grand_total,
            outstanding_amount: self.outstanding_amount,
            _state: PhantomData,
        };

        Ok((submitted, gl_request))
    }
}

impl SalesInvoice<SubmittedState> {
    pub fn cancel(self) -> SalesInvoice<CancelledState> {
        SalesInvoice {
            company_id: self.company_id,
            invoice_id: self.invoice_id,
            naming_series: self.naming_series,
            customer_id: self.customer_id,
            receivable_account_id: self.receivable_account_id,
            posting_date: self.posting_date,
            due_date: self.due_date,
            currency: self.currency,
            items: self.items,
            taxes: self.taxes,
            net_total: self.net_total,
            total_tax: self.total_tax,
            grand_total: self.grand_total,
            outstanding_amount: Decimal::ZERO,
            _state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::general_ledger::GeneralLedgerEngine;

    #[test]
    fn test_sales_invoice_lifecycle_and_gl_generation() {
        let company_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let ar_account = Uuid::new_v4();
        let income_account = Uuid::new_v4();
        let tax_account = Uuid::new_v4();

        let mut inv = SalesInvoice::new(
            company_id,
            "SINV-2026-00001".into(),
            customer_id,
            ar_account,
            NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 10, 31).unwrap(),
            Currency::USD,
        );

        // Add 2 widgets @ $50 each = $100
        inv.add_item("ITEM-WIDGET", "Enterprise Widget", income_account, Decimal::new(2, 0), Decimal::new(50, 0));
        // Add 10% VAT = $10
        inv.add_tax(tax_account, "VAT 10%", Decimal::new(10, 0));

        assert_eq!(inv.net_total, Decimal::new(100, 0));
        assert_eq!(inv.total_tax, Decimal::new(10, 0));
        assert_eq!(inv.grand_total, Decimal::new(110, 0));

        let (submitted_inv, gl_req) = inv.submit().expect("Should submit successfully");
        assert_eq!(submitted_inv.grand_total, Decimal::new(110, 0));

        // Ensure GL entries generated by the invoice balance perfectly!
        let balance_check = GeneralLedgerEngine::validate_and_balance(&gl_req);
        assert!(balance_check.is_ok());

        let cancelled_inv = submitted_inv.cancel();
        assert_eq!(cancelled_inv.outstanding_amount, Decimal::ZERO);
    }
}
