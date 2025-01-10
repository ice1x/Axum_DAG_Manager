use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use dotenvy::dotenv;
use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{post},
    Router,
    http::StatusCode,
};

// Models
#[derive(Serialize, Deserialize, FromRow)]
struct DAG {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
struct CreateDAGPayload {
    name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Node {
    id: Uuid,
    dag_id: Uuid,
    label: String,
}

#[derive(Deserialize)]
struct CreateNodePayload {
    // name: String,
    dag_id: Uuid,
    label: String,
}


#[derive(Serialize, Deserialize, FromRow)]
struct Edge {
    id: Uuid,
    source: Uuid,
    target: Uuid,
    dag_id: Uuid,
}

#[derive(Serialize, Deserialize, FromRow)]
struct CreateEdgePayload {
    source: Uuid,
    target: Uuid,
    dag_id: Uuid,
}

// Database Setup
async fn setup_database() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

//CRUD Handlers for DAG
async fn create_dag(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateDAGPayload>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let dag = DAG {
        id,
        name: payload.name,
    };

    match sqlx::query!(
        "INSERT INTO dags (id, name) VALUES ($1, $2)",
        dag.id,
        dag.name
    )
        .execute(&pool)
        .await
    {
        Ok(_) => Json(dag).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create DAG: {}", e),
        ).into_response(),
    }
}

async fn list_dags(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match sqlx::query_as::<_, DAG>("SELECT id, name FROM dags")
        .fetch_all(&pool)
        .await
    {
        Ok(dags) => Json(dags).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch DAGs: {}", e),
        ).into_response(),
    }
}


async fn create_node(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateNodePayload>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let node = Node {
        id,
        dag_id: payload.dag_id,
        label: payload.label,
    };

    match sqlx::query!(
        "INSERT INTO nodes (id, dag_id, label) VALUES ($1, $2, $3)",
        node.id,
        node.dag_id,
        node.label
    )
        .execute(&pool)
        .await
    {
        Ok(_) => Json(node).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create Node: {}", e),
        )
            .into_response(),
    }
}

async fn list_nodes(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Node>("SELECT id, dag_id, label FROM nodes")
        .fetch_all(&pool)
        .await
    {
        Ok(nodes) => Json(nodes).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch Nodes: {}", e),
        ).into_response(),
    }
}

// CRUD Handlers for Edge
async fn create_edge(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateEdgePayload>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let edge = Edge {
        id,
        source: payload.source,
        target: payload.target,
        dag_id: payload.dag_id,
    };

    match sqlx::query!(
        "INSERT INTO edges (id, source, target, dag_id) VALUES ($1, $2, $3, $4)",
        edge.id,
        edge.source,
        edge.target,
        edge.dag_id
    )
        .execute(&pool)
        .await
    {
        Ok(_) => Json(edge).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create Edge: {}", e),
        ).into_response(),
    }
}

async fn list_edges(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Edge>("SELECT id, source, target, dag_id FROM edges")
        .fetch_all(&pool)
        .await
    {
        Ok(edges) => Json(edges).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch Edges: {}", e),
        ).into_response(),
    }
}

// Main Application
#[tokio::main]
async fn main() {
    let pool = setup_database().await;

    let app = Router::new()
        .route("/dags", post(create_dag).get(list_dags))
        .route("/nodes", post(create_node).get(list_nodes))
        .route("/edges", post(create_edge).get(list_edges))
        .layer(Extension(pool));

    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
