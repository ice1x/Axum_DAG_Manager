use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::graph::would_create_cycle;
use crate::models::{CreateEdgePayload, Edge};

pub async fn create_edge(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateEdgePayload>,
) -> Result<impl IntoResponse> {
    // Verify that the DAG exists
    let dag_exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM dags WHERE id = $1)")
            .bind(payload.dag_id)
            .fetch_one(&pool)
            .await?;

    if !dag_exists {
        return Err(AppError::ValidationError(format!(
            "DAG with id {} does not exist",
            payload.dag_id
        )));
    }

    // Verify that both nodes exist and belong to the same DAG
    let source_node =
        sqlx::query_scalar::<_, Option<Uuid>>("SELECT dag_id FROM nodes WHERE id = $1")
            .bind(payload.source)
            .fetch_optional(&pool)
            .await?
            .flatten();

    let target_node =
        sqlx::query_scalar::<_, Option<Uuid>>("SELECT dag_id FROM nodes WHERE id = $1")
            .bind(payload.target)
            .fetch_optional(&pool)
            .await?
            .flatten();

    if source_node.is_none() {
        return Err(AppError::ValidationError(format!(
            "Source node with id {} does not exist",
            payload.source
        )));
    }

    if target_node.is_none() {
        return Err(AppError::ValidationError(format!(
            "Target node with id {} does not exist",
            payload.target
        )));
    }

    if source_node != Some(payload.dag_id) {
        return Err(AppError::ValidationError(
            "Source node does not belong to the specified DAG".to_string(),
        ));
    }

    if target_node != Some(payload.dag_id) {
        return Err(AppError::ValidationError(
            "Target node does not belong to the specified DAG".to_string(),
        ));
    }

    // Check for self-loop
    if payload.source == payload.target {
        return Err(AppError::ValidationError(
            "Cannot create an edge from a node to itself".to_string(),
        ));
    }

    // Fetch existing edges for this DAG to check for cycles
    let existing_edges =
        sqlx::query_as::<_, Edge>("SELECT id, source, target, dag_id FROM edges WHERE dag_id = $1")
            .bind(payload.dag_id)
            .fetch_all(&pool)
            .await?;

    // Check if adding this edge would create a cycle
    if would_create_cycle(&existing_edges, payload.source, payload.target) {
        return Err(AppError::CycleDetected);
    }

    let id = Uuid::new_v4();
    let edge = Edge {
        id,
        source: payload.source,
        target: payload.target,
        dag_id: payload.dag_id,
    };

    sqlx::query("INSERT INTO edges (id, source, target, dag_id) VALUES ($1, $2, $3, $4)")
        .bind(edge.id)
        .bind(edge.source)
        .bind(edge.target)
        .bind(edge.dag_id)
        .execute(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(edge)))
}

pub async fn list_edges(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse> {
    let edges = sqlx::query_as::<_, Edge>("SELECT id, source, target, dag_id FROM edges")
        .fetch_all(&pool)
        .await?;

    Ok(Json(edges))
}

pub async fn get_edge(
    Extension(pool): Extension<PgPool>,
    Path(edge_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let edge =
        sqlx::query_as::<_, Edge>("SELECT id, source, target, dag_id FROM edges WHERE id = $1")
            .bind(edge_id)
            .fetch_optional(&pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Edge with id {} not found", edge_id)))?;

    Ok(Json(edge))
}

pub async fn delete_edge(
    Extension(pool): Extension<PgPool>,
    Path(edge_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let result = sqlx::query("DELETE FROM edges WHERE id = $1")
        .bind(edge_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Edge with id {} not found",
            edge_id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
