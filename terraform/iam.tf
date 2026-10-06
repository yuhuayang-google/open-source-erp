# IAM Service Accounts & Role Assignments (Least Privilege)

# 1. API Service Account (Cloud Run API Service)
resource "google_service_account" "api_sa" {
  account_id   = "erp-api-${var.environment}"
  display_name = "Rust ERP API Service Account"
  project      = var.project_id
}

# 2. Worker Service Account (Background Worker / Cloud Tasks Consumer)
resource "google_service_account" "worker_sa" {
  account_id   = "erp-worker-${var.environment}"
  display_name = "Rust ERP Worker Service Account"
  project      = var.project_id
}

# Roles for API Service Account
locals {
  api_roles = [
    "roles/spanner.databaseUser",
    "roles/pubsub.publisher",
    "roles/cloudtasks.enqueuer",
    "roles/storage.objectUser",
    "roles/aiplatform.user",
    "roles/documentai.apiUser",
    "roles/secretmanager.secretAccessor",
    "roles/cloudtrace.agent",
    "roles/monitoring.metricWriter",
    "roles/logging.logWriter",
  ]
}

resource "google_project_iam_member" "api_roles" {
  for_each = toset(locals.api_roles)
  project  = var.project_id
  role     = each.value
  member   = "serviceAccount:${google_service_account.api_sa.email}"
}

# App-level Envelope Encryption Grant for API Service Account
resource "google_kms_crypto_key_iam_member" "api_envelope_key_user" {
  crypto_key_id = google_kms_crypto_key.app_envelope_key.id
  role          = "roles/cloudkms.cryptoKeyEncrypterDecrypter"
  member        = "serviceAccount:${google_service_account.api_sa.email}"
}

# Roles for Worker Service Account
locals {
  worker_roles = [
    "roles/spanner.databaseUser",
    "roles/pubsub.subscriber",
    "roles/cloudtasks.taskViewer",
    "roles/storage.objectAdmin",
    "roles/bigquery.dataEditor",
    "roles/cloudtrace.agent",
    "roles/monitoring.metricWriter",
    "roles/logging.logWriter",
  ]
}

resource "google_project_iam_member" "worker_roles" {
  for_each = toset(locals.worker_roles)
  project  = var.project_id
  role     = each.value
  member   = "serviceAccount:${google_service_account.worker_sa.email}"
}
