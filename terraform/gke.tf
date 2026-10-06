# Google Kubernetes Engine (GKE Autopilot) Cluster for Enterprise Rust ERP
# Provides VPC-native networking, Workload Identity Federation, Horizontal/Vertical Pod Autoscaling,
# persistent streaming workers, and Kubernetes CronJobs for MRP & Payroll batch processing.

resource "google_container_cluster" "erp_gke" {
  count    = contains(["gke", "hybrid"], var.compute_platform) ? 1 : 0
  name     = "erp-gke-${var.environment}-${random_id.resource_suffix.hex}"
  location = var.region # Regional cluster across 3 zones for 99.95%+ control plane & node SLA
  project  = var.project_id

  # Enable GKE Autopilot for hands-free node provisioning, security hardening, and bin-packing
  enable_autopilot = true

  network    = google_compute_network.erp_vpc.id
  subnetwork = google_compute_subnetwork.erp_subnet.id

  # VPC-Native (Alias IP) Routing: Pods talk directly to Memorystore Redis & Spanner without NAT/Connectors
  ip_allocation_policy {
    cluster_secondary_range_name  = "gke-pods-range"
    services_secondary_range_name = "gke-services-range"
  }

  # Private Cluster Configuration: Worker nodes have no public IP addresses
  private_cluster_config {
    enable_private_nodes    = true
    enable_private_endpoint = false
    master_ipv4_cidr_block  = var.gke_master_ipv4_cidr
  }

  # Release channel for automated Kubernetes security & minor version patching
  release_channel {
    channel = var.environment == "prod" ? "REGULAR" : "RAPID"
  }

  # Workload Identity Federation: Allows K8s ServiceAccounts to impersonate GCP IAM ServiceAccounts keylessly
  workload_identity_config {
    workload_pool = "${var.project_id}.svc.id.goog"
  }

  # Application-Layer Secrets Encryption (etcd encrypted via Cloud KMS)
  database_encryption {
    state    = "ENCRYPTED"
    key_name = google_kms_crypto_key.gke_etcd_key.id
  }

  deletion_protection = var.environment == "prod" ? true : false

  vertical_pod_autoscaling {
    enabled = true
  }

  depends_on = [
    google_project_service.enabled_services,
    google_compute_subnetwork.erp_subnet,
    google_kms_crypto_key_iam_member.gke_etcd_sa_key_user,
  ]
}
