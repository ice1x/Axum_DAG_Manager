use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::{CreateDAGPayload, DAGDetails, Edge, Node, UpdateDAGPayload, DAG};
use crate::validation::validate_name;

pub async fn create_dag(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateDAGPayload>,
) -> Result<impl IntoResponse> {
    validate_name(&payload.name)?;

    let id = Uuid::new_v4();
    let dag = DAG {
        id,
        name: payload.name.clone(),
    };

    sqlx::query("INSERT INTO dags (id, name) VALUES ($1, $2)")
        .bind(dag.id)
        .bind(&dag.name)
        .execute(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(dag)))
}

pub async fn list_dags(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse> {
    let dags = sqlx::query_as::<_, DAG>("SELECT id, name FROM dags")
        .fetch_all(&pool)
        .await?;

    Ok(Json(dags))
}

pub async fn get_dag(
    Extension(pool): Extension<PgPool>,
    Path(dag_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let dag = sqlx::query_as::<_, DAG>("SELECT id, name FROM dags WHERE id = $1")
        .bind(dag_id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("DAG with id {} not found", dag_id)))?;

    Ok(Json(dag))
}

pub async fn get_dag_with_details(
    Extension(pool): Extension<PgPool>,
    Path(dag_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let dag = sqlx::query_as::<_, DAG>("SELECT id, name FROM dags WHERE id = $1")
        .bind(dag_id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("DAG with id {} not found", dag_id)))?;

    let nodes = sqlx::query_as::<_, Node>("SELECT id, dag_id, label FROM nodes WHERE dag_id = $1")
        .bind(dag_id)
        .fetch_all(&pool)
        .await?;

    let edges =
        sqlx::query_as::<_, Edge>("SELECT id, source, target, dag_id FROM edges WHERE dag_id = $1")
            .bind(dag_id)
            .fetch_all(&pool)
            .await?;

    let details = DAGDetails { dag, nodes, edges };

    Ok(Json(details))
}

pub async fn update_dag(
    Extension(pool): Extension<PgPool>,
    Path(dag_id): Path<Uuid>,
    Json(payload): Json<UpdateDAGPayload>,
) -> Result<impl IntoResponse> {
    validate_name(&payload.name)?;

    let result = sqlx::query("UPDATE dags SET name = $1 WHERE id = $2")
        .bind(&payload.name)
        .bind(dag_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "DAG with id {} not found",
            dag_id
        )));
    }

    let dag = DAG {
        id: dag_id,
        name: payload.name,
    };

    Ok(Json(dag))
}

pub async fn delete_dag(
    Extension(pool): Extension<PgPool>,
    Path(dag_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Delete associated edges first
    sqlx::query("DELETE FROM edges WHERE dag_id = $1")
        .bind(dag_id)
        .execute(&pool)
        .await?;

    // Delete associated nodes
    sqlx::query("DELETE FROM nodes WHERE dag_id = $1")
        .bind(dag_id)
        .execute(&pool)
        .await?;

    // Delete the DAG
    let result = sqlx::query("DELETE FROM dags WHERE id = $1")
        .bind(dag_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "DAG with id {} not found",
            dag_id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
