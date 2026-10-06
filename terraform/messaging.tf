# Cloud Pub/Sub & Cloud Tasks Messaging Pipeline

# 1. Dead Letter Topic for Failed Events
resource "google_pubsub_topic" "dead_letter" {
  name    = "erp-dead-letter-${var.environment}"
  project = var.project_id

  depends_on = [google_project_service.enabled_services]
}

# 2. Primary Domain Events Topic (e.g. sales.order.submitted, stock.movement.posted)
resource "google_pubsub_topic" "domain_events" {
  name    = "erp-domain-events-${var.environment}"
  project = var.project_id

  message_storage_policy {
    allowed_persistence_regions = [var.region]
  }

  depends_on = [google_project_service.enabled_services]
}

# 3. Pull Subscription for the Rust Async Worker Service
resource "google_pubsub_subscription" "worker_subscription" {
  name    = "erp-worker-sub-${var.environment}"
  topic   = google_pubsub_topic.domain_events.id
  project = var.project_id

  ack_deadline_seconds       = 60
  message_retention_duration = "604800s" # 7 days
  retain_acked_messages      = false

  retry_policy {
    minimum_backoff = "5s"
    maximum_backoff = "600s"
  }

  dead_letter_policy {
    dead_letter_topic     = google_pubsub_topic.dead_letter.id
    max_delivery_attempts = 5
  }

  enable_message_ordering = true # Guarantees order per partition key (e.g., company_id or voucher_no)
}

# 4. Cloud Tasks Queue for Scheduled & Throttled Background Jobs
resource "google_cloud_tasks_queue" "async_queue" {
  name     = "erp-async-queue-${var.environment}"
  location = var.region
  project  = var.project_id

  rate_limits {
    max_dispatches_per_second = 100
    max_concurrent_dispatches = 50
  }

  retry_config {
    max_attempts       = 5
    min_backoff        = "2s"
    max_backoff        = "300s"
    max_doublings      = 4
    max_retry_duration = "3600s"
  }

  depends_on = [google_project_service.enabled_services]
}
