use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// DAG Models
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct DAG {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateDAGPayload {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateDAGPayload {
    pub name: String,
}

// Node Models
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Node {
    pub id: Uuid,
    pub dag_id: Uuid,
    pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateNodePayload {
    pub dag_id: Uuid,
    pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateNodePayload {
    pub label: String,
}

// Edge Models
#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Edge {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Uuid,
    pub dag_id: Uuid,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CreateEdgePayload {
    pub source: Uuid,
    pub target: Uuid,
    pub dag_id: Uuid,
}

// Response Models
#[derive(Serialize, Debug)]
pub struct DAGDetails {
    pub dag: DAG,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}
