use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{CreateNodePayload, Node, UpdateNodePayload};
use crate::validation::validate_label;

pub async fn create_node(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateNodePayload>,
) -> Result<impl IntoResponse> {
    validate_label(&payload.label)?;

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

    let id = Uuid::new_v4();
    let node = Node {
        id,
        dag_id: payload.dag_id,
        label: payload.label.clone(),
    };

    sqlx::query("INSERT INTO nodes (id, dag_id, label) VALUES ($1, $2, $3)")
        .bind(node.id)
        .bind(node.dag_id)
        .bind(&node.label)
        .execute(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(node)))
}

pub async fn list_nodes(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse> {
    let nodes = sqlx::query_as::<_, Node>("SELECT id, dag_id, label FROM nodes")
        .fetch_all(&pool)
        .await?;

    Ok(Json(nodes))
}

pub async fn get_node(
    Extension(pool): Extension<PgPool>,
    Path(node_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let node = sqlx::query_as::<_, Node>("SELECT id, dag_id, label FROM nodes WHERE id = $1")
        .bind(node_id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Node with id {} not found", node_id)))?;

    Ok(Json(node))
}

pub async fn update_node(
    Extension(pool): Extension<PgPool>,
    Path(node_id): Path<Uuid>,
    Json(payload): Json<UpdateNodePayload>,
) -> Result<impl IntoResponse> {
    validate_label(&payload.label)?;

    let result = sqlx::query("UPDATE nodes SET label = $1 WHERE id = $2")
        .bind(&payload.label)
        .bind(node_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Node with id {} not found",
            node_id
        )));
    }

    // Fetch the updated node
    let node = sqlx::query_as::<_, Node>("SELECT id, dag_id, label FROM nodes WHERE id = $1")
        .bind(node_id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(node))
}

pub async fn delete_node(
    Extension(pool): Extension<PgPool>,
    Path(node_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Delete associated edges
    sqlx::query("DELETE FROM edges WHERE source = $1 OR target = $1")
        .bind(node_id)
        .execute(&pool)
        .await?;

    // Delete the node
    let result = sqlx::query("DELETE FROM nodes WHERE id = $1")
        .bind(node_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Node with id {} not found",
            node_id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
