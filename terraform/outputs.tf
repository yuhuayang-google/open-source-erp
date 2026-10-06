# Deployment Outputs for the Rust ERP Stack

output "spanner_instance_name" {
  description = "Cloud Spanner instance name"
  value       = google_spanner_instance.erp_spanner.name
}

output "spanner_database_name" {
  description = "Cloud Spanner database name"
  value       = google_spanner_database.erp_database.name
}

output "spanner_database_uri" {
  description = "Fully-qualified Cloud Spanner database URI for Rust connection string"
  value       = "projects/${var.project_id}/instances/${google_spanner_instance.erp_spanner.name}/databases/${google_spanner_database.erp_database.name}"
}

output "redis_host" {
  description = "Memorystore Redis primary host IP"
  value       = google_redis_instance.erp_cache.host
}

output "redis_port" {
  description = "Memorystore Redis port"
  value       = google_redis_instance.erp_cache.port
}

output "pubsub_domain_events_topic" {
  description = "Cloud Pub/Sub topic ID for domain events"
  value       = google_pubsub_topic.domain_events.id
}

output "cloud_tasks_queue_id" {
  description = "Cloud Tasks queue ID for async jobs"
  value       = google_cloud_tasks_queue.async_queue.id
}

output "gcs_invoices_bucket" {
  description = "Cloud Storage bucket for incoming invoice OCR uploads"
  value       = google_storage_bucket.invoices_incoming.name
}

output "gcs_archive_bucket" {
  description = "Cloud Storage bucket for statutory 7-year audit archives"
  value       = google_storage_bucket.docs_archive.name
}

output "bigquery_dataset_id" {
  description = "BigQuery dataset ID for real-time analytics"
  value       = google_bigquery_dataset.erp_analytics.dataset_id
}

output "document_ai_processor_id" {
  description = "Vertex AI Document AI Invoice Processor ID"
  value       = google_document_ai_processor.invoice_processor.id
}

output "api_service_url" {
  description = "Cloud Run API Service URL"
  value       = google_cloud_run_v2_service.erp_api.uri
}

output "worker_service_url" {
  description = "Cloud Run Worker Service URL"
  value       = google_cloud_run_v2_service.erp_worker.uri
}
