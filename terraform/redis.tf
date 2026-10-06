# Memorystore Redis for Distributed Cache, Rate Limiting, and Document Numbering Locks
resource "google_redis_instance" "erp_cache" {
  name           = "erp-cache-${var.environment}-${random_id.resource_suffix.hex}"
  tier           = var.environment == "prod" ? "STANDARD_HA" : "BASIC"
  memory_size_gb = var.redis_memory_size_gb
  region         = var.region
  project        = var.project_id

  authorized_network = google_compute_network.erp_vpc.id
  connect_mode       = "PRIVATE_SERVICE_ACCESS"

  transit_encryption_mode = "SERVER_AUTHENTICATION"
  auth_enabled            = true

  labels = {
    environment = var.environment
    system      = "rust-erp"
  }

  depends_on = [
    google_service_networking_connection.private_vpc_connection
  ]
}
