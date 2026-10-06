# Cloud KMS Keyring and Customer-Managed Encryption Keys (CMEK)
# Ensures enterprise-grade cryptographic isolation and SOX/HIPAA/GDPR compliance.

resource "google_kms_key_ring" "erp_keyring" {
  name     = "erp-keyring-${var.environment}-${random_id.resource_suffix.hex}"
  location = var.region
  project  = var.project_id

  depends_on = [google_project_service.enabled_services]
}

# 1. CMEK for Cloud Spanner
resource "google_kms_crypto_key" "spanner_key" {
  name            = "spanner-cmek"
  key_ring        = google_kms_key_ring.erp_keyring.id
  rotation_period = "7776000s" # 90 days

  lifecycle {
    prevent_destroy = false
  }
}

# 2. CMEK for Cloud Storage Buckets
resource "google_kms_crypto_key" "storage_key" {
  name            = "storage-cmek"
  key_ring        = google_kms_key_ring.erp_keyring.id
  rotation_period = "7776000s"

  lifecycle {
    prevent_destroy = false
  }
}

# 3. CMEK for BigQuery Analytics
resource "google_kms_crypto_key" "bigquery_key" {
  name            = "bigquery-cmek"
  key_ring        = google_kms_key_ring.erp_keyring.id
  rotation_period = "7776000s"

  lifecycle {
    prevent_destroy = false
  }
}

# 4. Key for Application-level Envelope Encryption (PII & sensitive accounting tokens)
resource "google_kms_crypto_key" "app_envelope_key" {
  name            = "app-envelope-key"
  key_ring        = google_kms_key_ring.erp_keyring.id
  rotation_period = "7776000s"
}

# 5. CMEK for GKE Application-Layer Secrets Encryption (etcd)
resource "google_kms_crypto_key" "gke_etcd_key" {
  name            = "gke-etcd-cmek"
  key_ring        = google_kms_key_ring.erp_keyring.id
  rotation_period = "7776000s"

  lifecycle {
    prevent_destroy = false
  }
}

# Retrieve Google Managed Service Accounts for CMEK Granting
data "google_project" "project" {
  project_id = var.project_id
}

# GKE Service Agent Encrypter/Decrypter for etcd encryption
resource "google_kms_crypto_key_iam_member" "gke_etcd_sa_key_user" {
  crypto_key_id = google_kms_crypto_key.gke_etcd_key.id
  role          = "roles/cloudkms.cryptoKeyEncrypterDecrypter"
  member        = "serviceAccount:service-${data.google_project.project.number}@container-engine-robot.iam.gserviceaccount.com"
}

# Spanner Service Agent Encrypter/Decrypter
resource "google_kms_crypto_key_iam_member" "spanner_sa_key_user" {
  crypto_key_id = google_kms_crypto_key.spanner_key.id
  role          = "roles/cloudkms.cryptoKeyEncrypterDecrypter"
  member        = "serviceAccount:service-${data.google_project.project.number}@gcp-sa-spanner.iam.gserviceaccount.com"
}

# Cloud Storage Service Agent Encrypter/Decrypter
data "google_storage_project_service_account" "gcs_account" {
  project = var.project_id
}

resource "google_kms_crypto_key_iam_member" "storage_sa_key_user" {
  crypto_key_id = google_kms_crypto_key.storage_key.id
  role          = "roles/cloudkms.cryptoKeyEncrypterDecrypter"
  member        = "serviceAccount:${data.google_storage_project_service_account.gcs_account.email_address}"
}

# BigQuery Service Agent Encrypter/Decrypter
resource "google_kms_crypto_key_iam_member" "bigquery_sa_key_user" {
  crypto_key_id = google_kms_crypto_key.bigquery_key.id
  role          = "roles/cloudkms.cryptoKeyEncrypterDecrypter"
  member        = "serviceAccount:bq-${data.google_project.project.number}@bigquery-encryption.iam.gserviceaccount.com"
}
