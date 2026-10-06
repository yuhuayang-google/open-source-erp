# NextGen Open Source ERP

An enterprise-grade, memory-safe Open Source ERP written in **Rust** and designed to run on **Google Kubernetes Engine (GKE Autopilot)** (or **Cloud Run v2**) backed by **Google Cloud Platform (GCP)** managed services (Cloud Spanner, BigQuery, Pub/Sub, Cloud Tasks, Cloud Storage, Memorystore Redis, and Vertex AI / Document AI).

Modeled after the domain architectures and document workflows of **ERPNext** (Frappe Framework), this system replaces traditional monolithic relational database bottlenecks with a horizontally scalable, event-driven, globally consistent cloud-native architecture.

---

## 🖥️ Computing Infrastructure: GKE Autopilot vs. Cloud Run

The Terraform infrastructure supports selectable compute orchestration via `compute_platform = "gke" | "cloud_run" | "hybrid"` (defaulting to **`gke`**):

1. **Google Kubernetes Engine (`compute_platform = "gke"`, Recommended for Enterprise)**:
   * **GKE Autopilot Regional Private Cluster**: Zero node management, automated multi-zone high availability, `PodDisruptionBudgets`, and `HorizontalPodAutoscalers` (HPA).
   * **Direct VPC-Native Alias IP Networking**: Pods sit directly inside the VPC subnet (`10.100.0.0/14`) and communicate with **Memorystore Redis** at sub-millisecond latency without Serverless VPC Access Connector bottlenecks.
   * **Long-Lived Streaming & Batch Orchestration**: Runs persistent **Cloud Pub/Sub streaming pull** workers (`erp-worker`) and native Kubernetes **`CronJobs`** for multi-hour **Material Requirements Planning (MRP)** BOM explosions and daily ledger reconciliations.
   * **Keyless Security**: **GKE Workload Identity Federation** (`iam.gke.io/gcp-service-account`) + **Cloud KMS** application-layer `etcd` secret encryption.
2. **Cloud Run v2 (`compute_platform = "cloud_run"`, Ideal for Dev/SMB/Scale-to-Zero)**:
   * Serverless container execution with $< 20\,\text{ms}$ cold starts and scale-to-zero billing for bursty or low-traffic tenants.

---

## 🌟 Core ERP Capabilities

* **High Performance & Memory Safety**: Built with Rust (`axum`, `tokio`, `rust_decimal`), eliminating data races, garbage collection pauses, and floating-point rounding errors in financial transactions.
* **Double-Entry Bookkeeping Engine**: Strict double-entry general ledger with compile-time / runtime balancing invariants ($\sum \text{Debits} = \sum \text{Credits}$) and non-destructive reversal entries.
* **Perpetual Inventory Valuation**: Supports both **FIFO** (queue-based batch consumption) and **Moving Average** costing with strict negative stock guards.
* **Typestate Lifecycle Transitions**: Document lifecycles (`Draft` $\rightarrow$ `Submitted` $\rightarrow$ `Cancelled`) enforced via Rust's typestate pattern to prevent invalid state operations.

---

## 📁 Repository Structure

```
.
├── Cargo.toml                 # Cargo workspace definition
├── Dockerfile                 # Multi-stage Rust build -> Distroless (<25MB, nonroot) container
├── docs/
│   └── ARCHITECTURE.md        # Comprehensive system architecture & technical specification
├── k8s/                       # Kubernetes manifests for GKE Autopilot / Standard deployment
│   ├── base.yaml              # Namespace (erp-system), Workload Identity ServiceAccounts, ConfigMap
│   ├── api-deployment.yaml    # erp-api Deployment, NEG Service, HPA (3..50), PodDisruptionBudget
│   └── worker-and-cronjobs.yaml # erp-worker Deployment + Nightly MRP & Ledger Audit CronJobs
├── terraform/                 # Production-ready Terraform infrastructure definitions
│   ├── main.tf                # Provider setup & GCP API enablement
│   ├── variables.tf           # Configurable inputs (compute_platform = "gke" | "cloud_run" | "hybrid")
│   ├── gke.tf                 # GKE Autopilot Private Cluster, VPC-native IPs, Workload Identity
│   ├── cloud_run.tf           # Optional Cloud Run v2 services
│   ├── spanner.tf             # Cloud Spanner instance, CMEK database, and DDL schema
│   ├── kms.tf                 # Cloud KMS KeyRing & CMEK keys (Spanner, Storage, BQ, GKE etcd)
│   ├── networking.tf          # VPC, subnet with GKE secondary ranges, Cloud NAT, & Peering
│   ├── redis.tf               # Memorystore Redis instance
│   ├── storage.tf             # GCS buckets (invoices, 7-year audit archives, exports)
│   ├── messaging.tf           # Cloud Pub/Sub topics/subscriptions & Cloud Tasks
│   ├── analytics.tf           # BigQuery dataset & real-time P&L views
│   ├── ai.tf                  # Vertex AI Document AI Invoice Processor
│   ├── iam.tf                 # Least-privilege service accounts & Workload Identity bindings
│   ├── outputs.tf             # GKE credentials command, Spanner URI, Redis host, and endpoints
│   └── README.md              # Terraform provisioning instructions
└── crates/
    ├── erp-core/              # Money (fixed-point Decimal), Currency, UOM, DocStatus, Audit
    ├── erp-accounts/          # Chart of Accounts, General Ledger, Sales Invoice, Taxes
    ├── erp-stock/             # Item Master, Warehouses, Stock Ledger, FIFO & Moving Avg
    ├── erp-selling/           # Customers, Quotations, Sales Orders with typestate lifecycle
    ├── erp-buying/            # Suppliers, Purchase Orders, Receipts
    ├── erp-events/            # Pub/Sub EventEnvelope and publisher traits
    └── erp-api/               # Axum REST & gRPC API server
```

---

## 🚀 Getting Started

### 1. Build & Test Locally

```bash
git clone https://github.com/yuhuayang-google/open-source-erp.git
cd open-source-erp

# Run the full test suite
cargo test

# Launch the API server locally
cargo run -p erp-api
```

### 2. Provision GCP Infrastructure (GKE + Spanner + Redis + BQ)

```bash
cd terraform
cp terraform.tfvars.example terraform.tfvars
# Edit terraform.tfvars with your project_id (compute_platform defaults to "gke")

terraform init
terraform plan -out=tfplan
terraform apply tfplan
```

### 3. Deploy Workloads to GKE

```bash
# Authenticate kubectl with the newly provisioned GKE Autopilot cluster
$(terraform -chdir=terraform output -raw gke_get_credentials_command)

# Apply Kubernetes manifests
kubectl apply -f k8s/base.yaml
kubectl apply -f k8s/api-deployment.yaml
kubectl apply -f k8s/worker-and-cronjobs.yaml
```

---

## 📄 License

Licensed under the Apache License, Version 2.0.
