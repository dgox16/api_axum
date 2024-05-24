use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::{
        persona_conyuge_models::ConyugePersonaModelo, persona_types::RegimenConyugalPersona,
    },
    schemas::persona_schemas::{
        persona_conyuge_schemas::CrearPersonaConyugeSchema,
        persona_principal_schemas::ObtenerPersonaParams,
    },
    validators::persona_validators::persona_conyuge_validators::validar_nueva_persona_conyuge,
    AppState,
};

pub async fn crear_nueva_persona_conyuge_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaConyugeSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_conyuge(&data, params.id_persona, &body).await?;
    let nuevo_conyuge = sqlx::query_as!(
        ConyugePersonaModelo,
        r#"INSERT INTO conyuges_persona
        (id_persona, fecha_residencia, lugar_nacimiento,
        estado_nacimiento,regimen_conyugal)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
        id_persona_conyuge, id_persona,fecha_residencia, lugar_nacimiento, estado_nacimiento,
        regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona""#,
        params.id_persona,
        body.fecha_residencia,
        body.lugar_nacimiento,
        body.estado_nacimiento,
        body.regimen_conyugal as RegimenConyugalPersona,
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

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_conyuge
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_conyuge_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<ConyugePersonaModelo, (StatusCode, Json<serde_json::Value>)> {
    let conyuge_encontrado = sqlx::query_as!(
        ConyugePersonaModelo,
        r#"SELECT id_persona_conyuge, id_persona,fecha_residencia, lugar_nacimiento,
        estado_nacimiento,regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona"
        FROM conyuges_persona 
        WHERE id_persona=$1"#,
        id_persona,
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

    Ok(conyuge_encontrado)
}
