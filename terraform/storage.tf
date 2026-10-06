# Cloud Storage (GCS) Buckets with CMEK & Lifecycle Policies

# 1. Incoming Invoices & Receipts (Trigger for Vertex AI Document AI)
resource "google_storage_bucket" "invoices_incoming" {
  name                        = "erp-invoices-incoming-${var.project_id}-${random_id.resource_suffix.hex}"
  location                    = var.region
  project                     = var.project_id
  uniform_bucket_level_access = true

  encryption {
    default_kms_key_name = google_kms_crypto_key.storage_key.id
  }

  versioning {
    enabled = true
  }

  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      age = 90 # Raw incoming uploads cleaned up after ingestion
    }
  }

  depends_on = [google_kms_crypto_key_iam_member.storage_sa_key_user]
}

# 2. Immutable Tax and Audit Document Archive (7-year statutory retention)
resource "google_storage_bucket" "docs_archive" {
  name                        = "erp-docs-archive-${var.project_id}-${random_id.resource_suffix.hex}"
  location                    = var.region
  project                     = var.project_id
  uniform_bucket_level_access = true

  encryption {
    default_kms_key_name = google_kms_crypto_key.storage_key.id
  }

  versioning {
    enabled = true
  }

  retention_policy {
    is_locked        = false # Set to true in production once locked retention is verified
    retention_period = 220752000 # 7 years in seconds (statutory tax requirement)
  }

  lifecycle_rule {
    action {
      type          = "SetStorageClass"
      storage_class = "ARCHIVE"
    }
    condition {
      age = 365 # Transition to low-cost Archive tier after 1 year
    }
  }

  depends_on = [google_kms_crypto_key_iam_member.storage_sa_key_user]
}

# 3. Export Dumps & Ephemeral Reports
resource "google_storage_bucket" "exports" {
  name                        = "erp-exports-${var.project_id}-${random_id.resource_suffix.hex}"
  location                    = var.region
  project                     = var.project_id
  uniform_bucket_level_access = true

  encryption {
    default_kms_key_name = google_kms_crypto_key.storage_key.id
  }

  lifecycle_rule {
    action {
      type = "Delete"
    }
    condition {
      age = 14 # Temporary exports auto-expire in 14 days
    }
  }

  depends_on = [google_kms_crypto_key_iam_member.storage_sa_key_user]
}
