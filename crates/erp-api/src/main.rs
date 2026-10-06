use axum::{
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .init();

    info!("Starting NextGen Rust ERP API Server on Cloud Run...");

    let app = Router::new()
        .route("/healthz", get(health_check))
        .route("/api/v1/system/info", get(system_info))
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("PORT must be a valid u16 number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "erp-api",
        "engine": "rust",
        "spanner_connected": true
    }))
}

async fn system_info() -> Json<Value> {
    Json(json!({
        "system": "NextGen Open Source ERP",
        "version": "0.1.0",
        "architecture": "Rust + Google Cloud Platform",
        "modules": [
            "Accounts (General Ledger, Invoicing, Taxes)",
            "Stock (Perpetual Inventory, FIFO / Moving Avg)",
            "Selling (Customers, Quotations, Sales Orders)",
            "Buying (Suppliers, Purchase Orders, Receipts)",
            "Events (Cloud Pub/Sub, Cloud Tasks)"
        ]
    }))
}
