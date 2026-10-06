# Cloud Run v2 Services for Rust ERP Application

# 1. Rust API Service (Axum / Tonic HTTP/gRPC)
resource "google_cloud_run_v2_service" "erp_api" {
  name     = "erp-api-${var.environment}"
  location = var.region
  project  = var.project_id

  template {
    service_account = google_service_account.api_sa.email

    scaling {
      min_instance_count = var.environment == "prod" ? 2 : 0
      max_instance_count = 50
    }

    vpc_access {
      connector = google_vpc_access_connector.serverless_connector.id
      egress    = "PRIVATE_RANGES_ONLY"
    }

    containers {
      image = var.api_container_image

      resources {
        limits = {
          cpu    = "2000m"
          memory = "1024Mi" # Lightweight Rust container
        }
        cpu_idle = true
      }

      ports {
        container_port = 8080
      }

      env {
        name  = "ENVIRONMENT"
        value = var.environment
      }
      env {
        name  = "PROJECT_ID"
        value = var.project_id
      }
      env {
        name  = "SPANNER_DATABASE"
        value = "projects/${var.project_id}/instances/${google_spanner_instance.erp_spanner.name}/databases/${google_spanner_database.erp_database.name}"
      }
      env {
        name  = "REDIS_HOST"
        value = google_redis_instance.erp_cache.host
      }
      env {
        name  = "REDIS_PORT"
        value = tostring(google_redis_instance.erp_cache.port)
      }
      env {
        name  = "PUBSUB_EVENTS_TOPIC"
        value = google_pubsub_topic.domain_events.id
      }
      env {
        name  = "TASKS_QUEUE_ID"
        value = google_cloud_tasks_queue.async_queue.id
      }
      env {
        name  = "INVOICE_PROCESSOR_ID"
        value = google_document_ai_processor.invoice_processor.id
      }
      env {
        name  = "GCS_INVOICES_BUCKET"
        value = google_storage_bucket.invoices_incoming.name
      }
      env {
        name  = "GCS_ARCHIVE_BUCKET"
        value = google_storage_bucket.docs_archive.name
      }
      env {
        name  = "KMS_ENVELOPE_KEY_ID"
        value = google_kms_crypto_key.app_envelope_key.id
      }
    }
  }

  traffic {
    type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    percent = 100
  }

  depends_on = [
    google_vpc_access_connector.serverless_connector,
    google_spanner_database.erp_database
  ]
}

# 2. Rust Worker Service (Async Pub/Sub Consumer & Cloud Tasks Processor)
resource "google_cloud_run_v2_service" "erp_worker" {
  name     = "erp-worker-${var.environment}"
  location = var.region
  project  = var.project_id

  template {
    service_account = google_service_account.worker_sa.email

    scaling {
      min_instance_count = 1
      max_instance_count = 20
    }

    vpc_access {
      connector = google_vpc_access_connector.serverless_connector.id
      egress    = "PRIVATE_RANGES_ONLY"
    }

    containers {
      image = var.worker_container_image

      resources {
        limits = {
          cpu    = "1000m"
          memory = "512Mi"
        }
      }

      env {
        name  = "ENVIRONMENT"
        value = var.environment
      }
      env {
        name  = "PROJECT_ID"
        value = var.project_id
      }
      env {
        name  = "SPANNER_DATABASE"
        value = "projects/${var.project_id}/instances/${google_spanner_instance.erp_spanner.name}/databases/${google_spanner_database.erp_database.name}"
      }
      env {
        name  = "PUBSUB_SUBSCRIPTION"
        value = google_pubsub_subscription.worker_subscription.id
      }
      env {
        name  = "BIGQUERY_DATASET"
        value = google_bigquery_dataset.erp_analytics.dataset_id
      }
      env {
        name  = "GCS_ARCHIVE_BUCKET"
        value = google_storage_bucket.docs_archive.name
      }
    }
  }

  traffic {
    type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    percent = 100
  }

  depends_on = [
    google_vpc_access_connector.serverless_connector,
    google_spanner_database.erp_database
  ]
}
