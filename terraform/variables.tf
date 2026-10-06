variable "project_id" {
  description = "The GCP project ID where ERP resources will be provisioned"
  type        = string
}

variable "region" {
  description = "Primary GCP region for regional services (e.g. us-central1)"
  type        = string
  default     = "us-central1"
}

variable "compute_platform" {
  description = "Target computing infrastructure: 'gke' (GKE Autopilot cluster), 'cloud_run' (Serverless containers), or 'hybrid' (both GKE and Cloud Run)"
  type        = string
  default     = "gke"

  validation {
    condition     = contains(["gke", "cloud_run", "hybrid"], var.compute_platform)
    error_message = "compute_platform must be one of: 'gke', 'cloud_run', or 'hybrid'."
  }
}

variable "spanner_config" {
  description = "Cloud Spanner instance configuration (e.g., regional-us-central1, nam3, or multi-region)"
  type        = string
  default     = "regional-us-central1"
}

variable "spanner_processing_units" {
  description = "Processing units for Cloud Spanner (1000 PU = 1 node; minimum is 100 for dev)"
  type        = number
  default     = 1000
}

variable "environment" {
  description = "Environment tier (dev, staging, prod)"
  type        = string
  default     = "prod"
}

variable "vpc_cidr" {
  description = "CIDR block for the primary ERP VPC subnet"
  type        = string
  default     = "10.10.0.0/20"
}

variable "gke_pods_cidr" {
  description = "Secondary CIDR block for GKE Autopilot/Standard Pods (VPC-native alias IPs)"
  type        = string
  default     = "10.100.0.0/14"
}

variable "gke_services_cidr" {
  description = "Secondary CIDR block for GKE ClusterIP Services"
  type        = string
  default     = "10.104.0.0/20"
}

variable "gke_master_ipv4_cidr" {
  description = "CIDR block (/28) for the GKE private control plane"
  type        = string
  default     = "172.16.0.0/28"
}

variable "vpc_connector_cidr" {
  description = "CIDR block (/28 required) for the Serverless VPC Access connector (used when compute_platform is 'cloud_run' or 'hybrid')"
  type        = string
  default     = "10.10.16.0/28"
}

variable "redis_memory_size_gb" {
  description = "Memory size in GB for the Memorystore Redis cache"
  type        = number
  default     = 2
}

variable "api_container_image" {
  description = "Container image for the Rust ERP API service"
  type        = string
  default     = "gcr.io/cloudrun/hello" # Placeholder until custom Rust image is pushed
}

variable "worker_container_image" {
  description = "Container image for the Rust ERP Worker service"
  type        = string
  default     = "gcr.io/cloudrun/hello" # Placeholder until custom Rust image is pushed
}
