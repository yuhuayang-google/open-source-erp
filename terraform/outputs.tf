# Deployment Outputs for the Rust ERP Stack

output "compute_platform" {
  description = "Selected computing platform ('gke', 'cloud_run', or 'hybrid')"
  value       = var.compute_platform
}

output "gke_cluster_name" {
  description = "GKE Autopilot cluster name (when compute_platform is 'gke' or 'hybrid')"
  value       = length(google_container_cluster.erp_gke) > 0 ? google_container_cluster.erp_gke[0].name : null
}

output "gke_cluster_endpoint" {
  description = "GKE Autopilot control plane endpoint"
  value       = length(google_container_cluster.erp_gke) > 0 ? google_container_cluster.erp_gke[0].endpoint : null
  sensitive   = true
}

output "gke_get_credentials_command" {
  description = "Command to configure kubectl credentials for the GKE cluster"
  value       = length(google_container_cluster.erp_gke) > 0 ? "gcloud container clusters get-credentials ${google_container_cluster.erp_gke[0].name} --region ${var.region} --project ${var.project_id}" : null
}

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

output "cloud_run_api_service_url" {
  description = "Cloud Run API Service URL (when compute_platform is 'cloud_run' or 'hybrid')"
  value       = length(google_cloud_run_v2_service.erp_api) > 0 ? google_cloud_run_v2_service.erp_api[0].uri : null
}

output "cloud_run_worker_service_url" {
  description = "Cloud Run Worker Service URL (when compute_platform is 'cloud_run' or 'hybrid')"
  value       = length(google_cloud_run_v2_service.erp_worker) > 0 ? google_cloud_run_v2_service.erp_worker[0].uri : null
}
