mod db;
mod error;
mod graph;
mod handlers;
mod models;
mod validation;

use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::setup_database().await;

    let app = Router::new()
        // DAG routes
        .route("/dags", post(handlers::dag::create_dag))
        .route("/dags", get(handlers::dag::list_dags))
        .route("/dags/:id", get(handlers::dag::get_dag))
        .route(
            "/dags/:id/details",
            get(handlers::dag::get_dag_with_details),
        )
        .route("/dags/:id", put(handlers::dag::update_dag))
        .route("/dags/:id", delete(handlers::dag::delete_dag))
        // Node routes
        .route("/nodes", post(handlers::node::create_node))
        .route("/nodes", get(handlers::node::list_nodes))
        .route("/nodes/:id", get(handlers::node::get_node))
        .route("/nodes/:id", put(handlers::node::update_node))
        .route("/nodes/:id", delete(handlers::node::delete_node))
        // Edge routes
        .route("/edges", post(handlers::edge::create_edge))
        .route("/edges", get(handlers::edge::list_edges))
        .route("/edges/:id", get(handlers::edge::get_edge))
        .route("/edges/:id", delete(handlers::edge::delete_edge))
        .layer(Extension(pool));

    let addr = "127.0.0.1:3000";
    println!("🚀 Server running at http://{}", addr);
    println!("📊 DAG Manager API v0.1.0");
    println!("\nAvailable endpoints:");
    println!("  DAGs:   POST/GET/PUT/DELETE /dags");
    println!("  Nodes:  POST/GET/PUT/DELETE /nodes");
    println!("  Edges:  POST/GET/DELETE /edges");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
