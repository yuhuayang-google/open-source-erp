# Dedicated Virtual Private Cloud (VPC) for ERP
resource "google_compute_network" "erp_vpc" {
  name                    = "erp-vpc-${var.environment}"
  auto_create_subnetworks = false
  project                 = var.project_id

  depends_on = [google_project_service.enabled_services]
}

# Primary Subnet for private workloads (with GKE VPC-native secondary ranges for Pods & Services)
resource "google_compute_subnetwork" "erp_subnet" {
  name                     = "erp-subnet-${var.region}"
  ip_cidr_range            = var.vpc_cidr
  region                   = var.region
  network                  = google_compute_network.erp_vpc.id
  private_ip_google_access = true
  project                  = var.project_id

  secondary_ip_range {
    range_name    = "gke-pods-range"
    ip_cidr_range = var.gke_pods_cidr
  }

  secondary_ip_range {
    range_name    = "gke-services-range"
    ip_cidr_range = var.gke_services_cidr
  }
}

# Cloud Router & Cloud NAT for Private GKE Nodes outbound traffic
resource "google_compute_router" "erp_router" {
  name    = "erp-router-${var.region}-${var.environment}"
  region  = var.region
  network = google_compute_network.erp_vpc.id
  project = var.project_id
}

resource "google_compute_router_nat" "erp_nat" {
  name                               = "erp-nat-${var.region}-${var.environment}"
  router                             = google_compute_router.erp_router.name
  region                             = var.region
  project                            = var.project_id
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = "ALL_SUBNETWORKS_ALL_IP_RANGES"
}

# Serverless VPC Access Connector (only needed when Cloud Run is enabled)
resource "google_vpc_access_connector" "serverless_connector" {
  count         = contains(["cloud_run", "hybrid"], var.compute_platform) ? 1 : 0
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

# Private Service Access for Google Managed Services (Memorystore Redis peering)
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
