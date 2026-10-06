# Cloud Spanner Instance
resource "google_spanner_instance" "erp_spanner" {
  name             = "erp-spanner-${var.environment}-${random_id.resource_suffix.hex}"
  config           = var.spanner_config
  display_name     = "NextGen Rust ERP Spanner (${var.environment})"
  processing_units = var.spanner_processing_units
  project          = var.project_id

  labels = {
    environment = var.environment
    system      = "rust-erp"
  }

  depends_on = [google_project_service.enabled_services]
}

# Cloud Spanner Database with CMEK and Comprehensive Enterprise DDL Schema
resource "google_spanner_database" "erp_database" {
  instance = google_spanner_instance.erp_spanner.name
  name     = "erp_core_db"
  project  = var.project_id

  encryption_config {
    kms_key_name = google_kms_crypto_key.spanner_key.id
  }

  version_retention_period = "7d" # Allows point-in-time recovery (PITR) up to 7 days

  ddl = [
    <<-EOT
    -- 1. Company Master & Multi-Tenancy Anchor
    CREATE TABLE Companies (
        company_id STRING(36) NOT NULL,
        company_name STRING(128) NOT NULL,
        default_currency STRING(3) NOT NULL,
        country STRING(3) NOT NULL,
        fiscal_year_start_month INT64 NOT NULL,
        created_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true),
        updated_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true)
    ) PRIMARY KEY (company_id);
    EOT
    ,
    <<-EOT
    -- 2. Chart of Accounts Master
    CREATE TABLE Accounts (
        company_id STRING(36) NOT NULL,
        account_id STRING(36) NOT NULL,
        account_name STRING(128) NOT NULL,
        account_number STRING(32),
        parent_account_id STRING(36),
        root_type STRING(32) NOT NULL, -- Asset, Liability, Equity, Income, Expense
        report_type STRING(32) NOT NULL, -- Balance Sheet, Profit and Loss
        is_group BOOL NOT NULL,
        currency STRING(3) NOT NULL,
        created_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true),
        updated_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true)
    ) PRIMARY KEY (company_id, account_id),
      INTERLEAVE IN PARENT Companies ON DELETE CASCADE;
    EOT
    ,
    <<-EOT
    -- 3. Item Master & Inventory Warehouses
    CREATE TABLE Items (
        company_id STRING(36) NOT NULL,
        item_code STRING(64) NOT NULL,
        item_name STRING(128) NOT NULL,
        item_group STRING(64) NOT NULL,
        stock_uom STRING(16) NOT NULL,
        is_stock_item BOOL NOT NULL,
        valuation_method STRING(16) NOT NULL, -- 'FIFO', 'MovingAverage'
        standard_rate NUMERIC NOT NULL,
        safety_stock NUMERIC NOT NULL,
        created_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true),
        updated_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true)
    ) PRIMARY KEY (company_id, item_code),
      INTERLEAVE IN PARENT Companies ON DELETE CASCADE;
    EOT
    ,
    <<-EOT
    CREATE TABLE Warehouses (
        company_id STRING(36) NOT NULL,
        warehouse_id STRING(36) NOT NULL,
        warehouse_name STRING(128) NOT NULL,
        is_group BOOL NOT NULL,
        parent_warehouse_id STRING(36),
        account_id STRING(36),
        created_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true)
    ) PRIMARY KEY (company_id, warehouse_id),
      INTERLEAVE IN PARENT Companies ON DELETE CASCADE;
    EOT
    ,
    <<-EOT
    -- 4. Sales Orders (Parent Table)
    CREATE TABLE SalesOrders (
        company_id STRING(36) NOT NULL,
        sales_order_id STRING(36) NOT NULL,
        naming_series STRING(64) NOT NULL,
        customer_id STRING(36) NOT NULL,
        order_date DATE NOT NULL,
        delivery_date DATE NOT NULL,
        doc_status INT64 NOT NULL, -- 0: Draft, 1: Submitted, 2: Cancelled
        currency STRING(3) NOT NULL,
        conversion_rate NUMERIC NOT NULL,
        total_qty NUMERIC NOT NULL,
        net_total NUMERIC NOT NULL,
        grand_total NUMERIC NOT NULL,
        billing_status STRING(32) NOT NULL,
        delivery_status STRING(32) NOT NULL,
        created_by STRING(128) NOT NULL,
        created_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true),
        updated_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true)
    ) PRIMARY KEY (company_id, sales_order_id),
      INTERLEAVE IN PARENT Companies ON DELETE CASCADE;
    EOT
    ,
    <<-EOT
    -- Sales Order Line Items (Physically Interleaved in SalesOrders for 0-hop joins)
    CREATE TABLE SalesOrderItems (
        company_id STRING(36) NOT NULL,
        sales_order_id STRING(36) NOT NULL,
        line_item_id STRING(36) NOT NULL,
        item_code STRING(64) NOT NULL,
        warehouse_id STRING(36) NOT NULL,
        qty NUMERIC NOT NULL,
        delivered_qty NUMERIC NOT NULL,
        billed_qty NUMERIC NOT NULL,
        rate NUMERIC NOT NULL,
        amount NUMERIC NOT NULL,
        uom STRING(16) NOT NULL
    ) PRIMARY KEY (company_id, sales_order_id, line_item_id),
      INTERLEAVE IN PARENT SalesOrders ON DELETE CASCADE;
    EOT
    ,
    <<-EOT
    -- 5. Append-Only General Ledger (Double-Entry Bookkeeping)
    CREATE TABLE GeneralLedgerEntries (
        company_id STRING(36) NOT NULL,
        gle_id STRING(36) NOT NULL,
        posting_date DATE NOT NULL,
        account_id STRING(36) NOT NULL,
        cost_center_id STRING(36),
        party_type STRING(32), -- 'Customer', 'Supplier', 'Employee'
        party_id STRING(36),
        voucher_type STRING(32) NOT NULL, -- 'Sales Invoice', 'Payment Entry', 'Journal Entry'
        voucher_no STRING(64) NOT NULL,
        debit NUMERIC NOT NULL,
        credit NUMERIC NOT NULL,
        currency STRING(3) NOT NULL,
        is_cancelled BOOL NOT NULL,
        reversal_of_gle_id STRING(36),
        created_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true)
    ) PRIMARY KEY (company_id, gle_id),
      INTERLEAVE IN PARENT Companies ON DELETE CASCADE;
    EOT
    ,
    <<-EOT
    CREATE INDEX Idx_GLE_Account_Date 
    ON GeneralLedgerEntries(company_id, account_id, posting_date DESC);
    EOT
    ,
    <<-EOT
    -- 6. Append-Only Perpetual Stock Ledger
    CREATE TABLE StockLedgerEntries (
        company_id STRING(36) NOT NULL,
        sle_id STRING(36) NOT NULL,
        item_code STRING(64) NOT NULL,
        warehouse_id STRING(36) NOT NULL,
        posting_datetime TIMESTAMP NOT NULL,
        voucher_type STRING(32) NOT NULL,
        voucher_no STRING(64) NOT NULL,
        actual_qty NUMERIC NOT NULL,
        qty_after_transaction NUMERIC NOT NULL,
        incoming_rate NUMERIC NOT NULL,
        valuation_rate NUMERIC NOT NULL,
        stock_value NUMERIC NOT NULL,
        stock_value_difference NUMERIC NOT NULL,
        batch_no STRING(64),
        serial_no STRING(64),
        is_cancelled BOOL NOT NULL,
        created_at TIMESTAMP NOT NULL OPTIONS (allow_commit_timestamp = true)
    ) PRIMARY KEY (company_id, sle_id),
      INTERLEAVE IN PARENT Companies ON DELETE CASCADE;
    EOT
    ,
    <<-EOT
    CREATE INDEX Idx_SLE_Item_Warehouse 
    ON StockLedgerEntries(company_id, item_code, warehouse_id, posting_datetime DESC);
    EOT
    ,
    <<-EOT
    -- 7. Change Stream for Real-time BigQuery Streaming CDC
    CREATE CHANGE STREAM ErpChangeStream FOR ALL;
    EOT
  ]

  depends_on = [
    google_kms_crypto_key_iam_member.spanner_sa_key_user,
    google_spanner_instance.erp_spanner
  ]
}
