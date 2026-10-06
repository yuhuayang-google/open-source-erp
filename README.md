# NextGen Open Source ERP

An enterprise-grade, memory-safe Open Source ERP written in **Rust** and designed to leverage **Google Cloud Platform (GCP)** managed services (Cloud Spanner, BigQuery, Pub/Sub, Cloud Tasks, Cloud Storage, Memorystore, Cloud Run, and Vertex AI / Document AI).

Modeled after the domain architectures and document workflows of **ERPNext** (Frappe Framework), this system replaces traditional monolithic relational database bottlenecks with a horizontally scalable, event-driven, globally consistent cloud-native architecture.

---

## 🌟 Key Features

* **High Performance & Memory Safety**: Built with Rust (`axum`, `tokio`, `rust_decimal`), eliminating data races, garbage collection pauses, and rounding errors in financial transactions.
* **Double-Entry Bookkeeping Engine**: Strict double-entry general ledger with compile-time / runtime balancing invariants (`Debits == Credits`) and non-destructive reversal entries.
* **Perpetual Inventory Valuation**: Supports both **FIFO** (queue-based consumption) and **Moving Average** costing with strict negative stock guards.
* **Typestate Lifecycle Transitions**: Document lifecycles (`Draft` $\rightarrow$ `Submitted` $\rightarrow$ `Cancelled`) enforced via Rust's typestate pattern to prevent invalid state operations.
* **Google Cloud Native**:
  * **Cloud Spanner**: Globally distributed ACID transactions, parent-child interleaved tables (`SalesOrderItems` in `SalesOrders`), and commit timestamps.
  * **Memorystore Redis**: Distributed locking, rapid sequence generation, and session caching.
  * **Cloud Pub/Sub & Cloud Tasks**: Event-driven decoupling and throttled asynchronous background processing.
  * **BigQuery CDC**: Zero-ETL continuous streaming of transactional ledgers for sub-second financial reporting.
  * **Vertex AI Document AI**: Automated Accounts Payable OCR and receipt processing into draft purchase invoices.
  * **Cloud Run v2**: Stateless, containerized microservices scaling from zero with $< 20\,\text{ms}$ cold starts.
  * **Cloud KMS**: CMEK encryption and envelope encryption for sensitive banking and PII data.

---

## 📁 Repository Structure

```
.
├── Cargo.toml                 # Cargo workspace definition
├── .gitignore                 # Rust & Terraform ignore rules
├── docs/
│   └── ARCHITECTURE.md        # Comprehensive system architecture & technical specification
├── terraform/                 # Production-ready Terraform infrastructure definitions
│   ├── main.tf                # Provider setup & GCP API enablement
│   ├── variables.tf           # Configurable inputs
│   ├── spanner.tf             # Cloud Spanner instance, CMEK database, and DDL schema
│   ├── kms.tf                 # Cloud KMS KeyRing & CMEK keys
│   ├── networking.tf          # VPC, subnet, and Serverless VPC Access connector
│   ├── redis.tf               # Memorystore Redis instance
│   ├── storage.tf             # GCS buckets (invoices, audit archives, exports)
│   ├── messaging.tf           # Cloud Pub/Sub topics/subscriptions & Cloud Tasks
│   ├── analytics.tf           # BigQuery dataset & real-time P&L views
│   ├── ai.tf                  # Vertex AI Document AI Invoice Processor
│   ├── iam.tf                 # Least-privilege service accounts & IAM bindings
│   ├── cloud_run.tf           # Cloud Run v2 services for API and Worker
│   ├── outputs.tf             # Endpoints, database URIs, and resource IDs
│   └── README.md              # Terraform provisioning instructions
└── crates/
    ├── erp-core/              # Money (fixed-point Decimal), Currency, UOM, DocStatus, Audit
    ├── erp-accounts/          # Chart of Accounts, General Ledger, Sales Invoice, Taxes
    ├── erp-stock/             # Item Master, Warehouses, Stock Ledger, FIFO & Moving Avg
    ├── erp-selling/           # Customers, Quotations, Sales Orders with typestate lifecycle
    ├── erp-buying/            # Suppliers, Purchase Orders, Receipts
    ├── erp-events/            # Pub/Sub EventEnvelope and publisher traits
    └── erp-api/               # Axum REST & gRPC API server for Cloud Run
```

---

## 🚀 Getting Started

### 1. Build & Test Locally

Ensure you have Rust (>= 1.75) installed:

```bash
# Clone the repository
git clone https://github.com/yuhuayang-google/open-source-erp.git
cd open-source-erp

# Run the full test suite
cargo test

# Launch the API server locally
cargo run -p erp-api
```

Test the local health check:
```bash
curl http://localhost:8080/healthz
```

---

### 2. Deploy Infrastructure to GCP with Terraform

```bash
cd terraform
cp terraform.tfvars.example terraform.tfvars

# Edit terraform.tfvars with your GCP project ID and region
terraform init
terraform plan -out=tfplan
terraform apply tfplan
```

Refer to [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) and [`terraform/README.md`](terraform/README.md) for detailed configuration options.

---

## 📄 License

Licensed under the Apache License, Version 2.0.
