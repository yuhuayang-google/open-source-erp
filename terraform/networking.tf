# Dedicated Virtual Private Cloud (VPC) for ERP
resource "google_compute_network" "erp_vpc" {
  name                    = "erp-vpc-${var.environment}"
  auto_create_subnetworks = false
  project                 = var.project_id

  depends_on = [google_project_service.enabled_services]
}

# Primary Subnet for private workloads
resource "google_compute_subnetwork" "erp_subnet" {
  name                     = "erp-subnet-${var.region}"
  ip_cidr_range            = var.vpc_cidr
  region                   = var.region
  network                  = google_compute_network.erp_vpc.id
  private_ip_google_access = true
  project                  = var.project_id
}

# Serverless VPC Access Connector
# Enables Cloud Run microservices to access Memorystore Redis and other private VPC resources
resource "google_vpc_access_connector" "serverless_connector" {
  name          = "erp-con-${var.environment}"
  region        = var.region
  network       = google_compute_network.erp_vpc.name
  ip_cidr_range = var.vpc_connector_cidr
  min_instances = 2
  max_instances = 10
  machine_type  = "e2-micro"
  project       = var.project_id

  depends_on = [
    google_project_service.enabled_services,
    google_compute_network.erp_vpc
  ]
}

# Private Service Access for Google Managed Services (Redis peering)
resource "google_compute_global_address" "private_service_access_ip" {
  name          = "erp-private-service-access"
  purpose       = "VPC_PEERING"
  address_type  = "INTERNAL"
  prefix_length = 16
  network       = google_compute_network.erp_vpc.id
  project       = var.project_id
}

resource "google_service_networking_connection" "private_vpc_connection" {
  network                 = google_compute_network.erp_vpc.id
  service                 = "servicenetworking.googleapis.com"
  reserved_peering_ranges = [google_compute_global_address.private_service_access_ip.name]

  depends_on = [google_compute_global_address.private_service_access_ip]
}
