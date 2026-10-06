# Vertex AI & Document AI Configuration for Automated Invoicing

# Document AI Invoice Processor for Accounts Payable Automation
resource "google_document_ai_processor" "invoice_processor" {
  location     = var.region == "us-central1" ? "us" : var.region # Document AI multi-region endpoints (us/eu)
  display_name = "erp-invoice-parser-${var.environment}"
  type         = "INVOICE_PROCESSOR"
  project      = var.project_id

  depends_on = [google_project_service.enabled_services]
}

# Secret Manager for External Integration Credentials (Payment gateways, bank APIs)
resource "google_secret_manager_secret" "banking_gateway_api_key" {
  secret_id = "erp-banking-gateway-key-${var.environment}"
  project   = var.project_id

  replication {
    auto {}
  }

  depends_on = [google_project_service.enabled_services]
}
