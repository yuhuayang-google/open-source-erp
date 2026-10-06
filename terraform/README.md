# Infrastructure as Code: Rust ERP on Google Cloud Platform

This directory contains modular, production-ready Terraform configurations to provision all backend resources for the Rust-based ERP system modeled after ERPNext.

---

## Architecture Components Provisioned

1. **Cloud Spanner**:
   - High-throughput, globally consistent ACID relational database.
   - CMEK encryption via Cloud KMS.
   - Point-in-time recovery (PITR) with a 7-day version retention period.
   - Comprehensive DDL schema (`Companies`, `Accounts`, `Items`, `Warehouses`, `SalesOrders`, `SalesOrderItems`, `GeneralLedgerEntries`, `StockLedgerEntries`, and `ErpChangeStream`).
2. **Cloud KMS**:
   - Dedicated KeyRing and CryptoKeys for Spanner, Cloud Storage, BigQuery, and application-layer envelope encryption.
3. **Networking & Security**:
   - Custom VPC network with private subnets.
   - Serverless VPC Access connector allowing Cloud Run services to securely reach Memorystore Redis over internal IP addresses.
   - Service Networking peering for managed Google services.
4. **Memorystore for Redis**:
   - In-memory distributed cache, atomic sequence locking, and rate limiting with TLS and authentication.
5. **Cloud Storage (GCS)**:
   - `erp-invoices-incoming`: Upload bucket for automated OCR ingestion.
   - `erp-docs-archive`: Regulatory 7-year audit retention with lifecycle rules.
   - `erp-exports`: Ephemeral export dumps with auto-deletion after 14 days.
6. **Cloud Pub/Sub & Cloud Tasks**:
   - `erp-domain-events`: Event-driven choreography for order submission, stock movements, and financial reconciliations.
   - `erp-dead-letter`: Dead-letter queue for unprocessable events.
   - `erp-async-queue`: Throttled task queue with exponential backoff for PDF generation, emails, and background jobs.
7. **BigQuery (OLAP)**:
   - CMEK-encrypted real-time analytics dataset.
   - Pre-configured financial reporting view (`v_profit_and_loss_monthly`).
8. **Vertex AI & Document AI**:
   - `INVOICE_PROCESSOR` processor for parsing supplier bills and receipts.
9. **IAM & Service Accounts**:
   - Least-privilege dedicated service accounts (`erp-api` and `erp-worker`).
10. **Cloud Run v2**:
    - Auto-scaling stateless container services (`erp-api` and `erp-worker`) connected to the VPC.

---

## Prerequisites

- [Terraform](https://developer.hashicorp.com/terraform/downloads) >= 1.5.0
- [Google Cloud SDK (`gcloud`)](https://cloud.google.com/sdk/docs/install) authenticated with permissions to create resources in your target project:
  ```bash
  gcloud auth application-default login
  gcloud config set project YOUR_PROJECT_ID
  ```

---

## Deployment Steps

### 1. Initialize Configuration
Copy the example variables file and adjust parameters:
```bash
cp terraform.tfvars.example terraform.tfvars
```
Edit `terraform.tfvars` with your GCP project ID and preferred region:
```hcl
project_id               = "my-gcp-erp-project"
region                   = "us-central1"
spanner_config           = "regional-us-central1"
spanner_processing_units = 1000
environment              = "prod"
```

### 2. Initialize Terraform
```bash
terraform init
```

### 3. Review Plan
```bash
terraform plan -out=tfplan
```

### 4. Apply Configuration
```bash
terraform apply tfplan
```

### 5. Inspect Outputs
Once completed, the connection strings, Spanner database URI, and endpoints will be printed:
```bash
terraform output
```
