use crate::error::AccountingError;
use chrono::{DateTime, NaiveDate, Utc};
use erp_core::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralLedgerEntry {
    pub company_id: Uuid,
    pub gle_id: Uuid,
    pub posting_date: NaiveDate,
    pub account_id: Uuid,
    pub cost_center_id: Option<Uuid>,
    pub party_type: Option<String>, // 'Customer', 'Supplier', 'Employee'
    pub party_id: Option<Uuid>,
    pub voucher_type: String, // 'Sales Invoice', 'Purchase Invoice', 'Journal Entry', etc.
    pub voucher_no: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub currency: Currency,
    pub is_cancelled: bool,
    pub reversal_of_gle_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GLEntryItem {
    pub account_id: Uuid,
    pub cost_center_id: Option<Uuid>,
    pub party_type: Option<String>,
    pub party_id: Option<Uuid>,
    pub debit: Decimal,
    pub credit: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GLPostingRequest {
    pub company_id: Uuid,
    pub posting_date: NaiveDate,
    pub voucher_type: String,
    pub voucher_no: String,
    pub currency: Currency,
    pub entries: Vec<GLEntryItem>,
}

pub struct GeneralLedgerEngine;

impl GeneralLedgerEngine {
    /// Validates fundamental double-entry invariant: Sum(Debits) == Sum(Credits)
    pub fn validate_and_balance(req: &GLPostingRequest) -> Result<(), AccountingError> {
        if req.entries.is_empty() {
            return Err(AccountingError::EmptyPostingRequest);
        }

        let mut total_debit = Decimal::ZERO;
        let mut total_credit = Decimal::ZERO;

        for entry in &req.entries {
            if entry.debit < Decimal::ZERO || entry.credit < Decimal::ZERO {
                return Err(AccountingError::NegativeAmount);
            }
            if entry.debit > Decimal::ZERO && entry.credit > Decimal::ZERO {
                return Err(AccountingError::SimultaneousDebitAndCredit);
            }
            total_debit += entry.debit;
            total_credit += entry.credit;
        }

        if total_debit != total_credit {
            return Err(AccountingError::UnbalancedEntry {
                debit: total_debit,
                credit: total_credit,
                difference: total_debit - total_credit,
            });
        }

        Ok(())
    }

    /// Converts a validated GLPostingRequest into persisted GeneralLedgerEntry rows
    pub fn create_entries(req: GLPostingRequest) -> Result<Vec<GeneralLedgerEntry>, AccountingError> {
        Self::validate_and_balance(&req)?;

        let now = Utc::now();
        let entries = req
            .entries
            .into_iter()
            .map(|item| GeneralLedgerEntry {
                company_id: req.company_id,
                gle_id: Uuid::new_v4(),
                posting_date: req.posting_date,
                account_id: item.account_id,
                cost_center_id: item.cost_center_id,
                party_type: item.party_type,
                party_id: item.party_id,
                voucher_type: req.voucher_type.clone(),
                voucher_no: req.voucher_no.clone(),
                debit: item.debit,
                credit: item.credit,
                currency: req.currency,
                is_cancelled: false,
                reversal_of_gle_id: None,
                created_at: now,
            })
            .collect();

        Ok(entries)
    }

    /// Generates strictly immutable reversing entries (swapping Debit & Credit) to cancel a voucher
    pub fn create_reversal_entries(
        original_entries: &[GeneralLedgerEntry],
        reversal_posting_date: NaiveDate,
    ) -> Vec<GeneralLedgerEntry> {
        let now = Utc::now();
        original_entries
            .iter()
            .map(|orig| GeneralLedgerEntry {
                company_id: orig.company_id,
                gle_id: Uuid::new_v4(),
                posting_date: reversal_posting_date,
                account_id: orig.account_id,
                cost_center_id: orig.cost_center_id,
                party_type: orig.party_type.clone(),
                party_id: orig.party_id,
                voucher_type: orig.voucher_type.clone(),
                voucher_no: orig.voucher_no.clone(),
                debit: orig.credit,  // Reversal: Credit becomes Debit
                credit: orig.debit,  // Reversal: Debit becomes Credit
                currency: orig.currency,
                is_cancelled: true,
                reversal_of_gle_id: Some(orig.gle_id),
                created_at: now,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_gl_posting() {
        let company_id = Uuid::new_v4();
        let ar_account = Uuid::new_v4();
        let rev_account = Uuid::new_v4();

        let req = GLPostingRequest {
            company_id,
            posting_date: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: "ACC-SINV-2026-0001".to_string(),
            currency: Currency::USD,
            entries: vec![
                GLEntryItem {
                    account_id: ar_account,
                    cost_center_id: None,
                    party_type: Some("Customer".into()),
                    party_id: Some(Uuid::new_v4()),
                    debit: Decimal::new(10000, 2), // 100.00
                    credit: Decimal::ZERO,
                },
                GLEntryItem {
                    account_id: rev_account,
                    cost_center_id: None,
                    party_type: None,
                    party_id: None,
                    debit: Decimal::ZERO,
                    credit: Decimal::new(10000, 2), // 100.00
                },
            ],
        };

        let result = GeneralLedgerEngine::create_entries(req);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 2);

        // Test reversal
        let reversals = GeneralLedgerEngine::create_reversal_entries(
            &entries,
            NaiveDate::from_ymd_opt(2026, 10, 2).unwrap(),
        );
        assert_eq!(reversals.len(), 2);
        assert_eq!(reversals[0].debit, Decimal::ZERO);
        assert_eq!(reversals[0].credit, Decimal::new(10000, 2));
    }

    #[test]
    fn test_unbalanced_gl_posting_rejected() {
        let req = GLPostingRequest {
            company_id: Uuid::new_v4(),
            posting_date: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
            voucher_type: "Journal Entry".to_string(),
            voucher_no: "JV-0001".to_string(),
            currency: Currency::USD,
            entries: vec![
                GLEntryItem {
                    account_id: Uuid::new_v4(),
                    cost_center_id: None,
                    party_type: None,
                    party_id: None,
                    debit: Decimal::new(100, 0),
                    credit: Decimal::ZERO,
                },
                GLEntryItem {
                    account_id: Uuid::new_v4(),
                    cost_center_id: None,
                    party_type: None,
                    party_id: None,
                    debit: Decimal::ZERO,
                    credit: Decimal::new(90, 0), // Unbalanced!
                },
            ],
        };

        assert!(GeneralLedgerEngine::validate_and_balance(&req).is_err());
    }
}
