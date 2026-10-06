terraform {
  required_version = ">= 1.5.0"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.30"
    }
    google-beta = {
      source  = "hashicorp/google-beta"
      version = "~> 5.30"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.5"
    }
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

provider "google-beta" {
  project = var.project_id
  region  = var.region
}

# Core GCP APIs required for the Rust ERP Stack
locals {
  services = [
    "spanner.googleapis.com",
    "pubsub.googleapis.com",
    "cloudtasks.googleapis.com",
    "bigquery.googleapis.com",
    "storage.googleapis.com",
    "redis.googleapis.com",
    "run.googleapis.com",
    "vpcaccess.googleapis.com",
    "compute.googleapis.com",
    "documentai.googleapis.com",
    "aiplatform.googleapis.com",     # Vertex AI
    "cloudkms.googleapis.com",
    "secretmanager.googleapis.com",
    "cloudtrace.googleapis.com",
    "cloudmonitoring.googleapis.com",
    "logging.googleapis.com",
    "iam.googleapis.com",
  ]
}

resource "google_project_service" "enabled_services" {
  for_each                   = toset(locals.services)
  project                    = var.project_id
  service                    = each.value
  disable_dependent_services = false
  disable_on_destroy         = false
}

resource "random_id" "resource_suffix" {
  byte_length = 4
}
