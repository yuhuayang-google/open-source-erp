# BigQuery Dataset for Real-time Financial Reporting & Machine Learning (OLAP)

resource "google_bigquery_dataset" "erp_analytics" {
  dataset_id                  = "erp_analytics_${var.environment}"
  friendly_name               = "ERP Financial and Operational Analytics"
  description                 = "Real-time CDC and analytical warehouse for Rust ERP system"
  location                    = var.region
  project                     = var.project_id
  default_table_expiration_ms = null # Retain data indefinitely

  default_encryption_configuration {
    kms_key_name = google_kms_crypto_key.bigquery_key.id
  }

  labels = {
    environment = var.environment
    system      = "rust-erp"
  }

  depends_on = [
    google_kms_crypto_key_iam_member.bigquery_sa_key_user,
    google_project_service.enabled_services
  ]
}

# Real-time Reporting View: Profit & Loss Statement (P&L Rollup)
resource "google_bigquery_table" "view_profit_and_loss" {
  dataset_id = google_bigquery_dataset.erp_analytics.dataset_id
  table_id   = "v_profit_and_loss_monthly"
  project    = var.project_id

  view {
    query          = <<-EOT
      SELECT
        company_id,
        EXTRACT(YEAR FROM posting_date) AS fiscal_year,
        EXTRACT(MONTH FROM posting_date) AS fiscal_month,
        root_type,
        account_name,
        SUM(credit) - SUM(debit) AS net_income_contribution
      FROM
        `${var.project_id}.${google_bigquery_dataset.erp_analytics.dataset_id}.general_ledger_entries`
      WHERE
        root_type IN ('Income', 'Expense')
        AND is_cancelled = FALSE
      GROUP BY
        company_id, fiscal_year, fiscal_month, root_type, account_name
    EOT
    use_legacy_sql = false
  }

  deletion_protection = false
}
