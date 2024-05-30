use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::relaciones_de_persona_models::RelacionDePersonaModelo,
    schemas::persona_schemas::{
        persona_principal_schemas::ObtenerPersonaParams,
        relaciones_de_persona_schemas::CrearRelacionDePersonaSchema,
    },
    validators::persona_validators::relaciones_de_persona_validators::validar_nueva_relacion_persona,
    AppState,
};

pub async fn crear_nueva_relacion_de_persona_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<Vec<CrearRelacionDePersonaSchema>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut relaciones_creadas = Vec::new();
    validar_nueva_relacion_persona(&data, params.id_persona).await?;
    for relacion in body {
        let nueva_relacion = sqlx::query_as!(
            RelacionDePersonaModelo,
            r#"INSERT INTO relaciones_de_persona
            (id_persona, persona_relacionada, parentesco)
            VALUES ($1, $2, $3)
            RETURNING 
            id_relacion_de_persona, id_persona, persona_relacionada, parentesco "#,
            params.id_persona,
            relacion.persona_relacionada,
            relacion.parentesco
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "estado": false,
                    "mensaje": format!("Error en la base de datos: {}", e),
                })),
            )
        })?;
        relaciones_creadas.push(nueva_relacion);
    }

    let respuesta = json!({
        "estado": true,
        "datos": relaciones_creadas
    });

    Ok(Json(respuesta))
}

pub async fn obtener_relaciones_de_persona_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<Vec<RelacionDePersonaModelo>, (StatusCode, Json<serde_json::Value>)> {
    let relaciones_encontradas = sqlx::query_as!(
        RelacionDePersonaModelo,
        r#"SELECT 
        id_relacion_de_persona, id_persona, persona_relacionada, parentesco 
        FROM relaciones_de_persona WHERE id_persona=$1
        "#,
        id_persona,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )
    })?;

    Ok(relaciones_encontradas)
}
