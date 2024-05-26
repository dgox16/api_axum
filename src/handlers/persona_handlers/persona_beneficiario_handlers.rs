use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::persona_beneficiario_models::BeneficiarioPersonaModelo,
    schemas::persona_schemas::{
        persona_beneficiario_schemas::CrearPersonaBeneficiarioSchema,
        persona_principal_schemas::ObtenerPersonaParams,
    },
    validators::persona_validators::persona_beneficiario_validators::validar_nueva_persona_beneficiario,
    AppState,
};

pub async fn crear_nueva_persona_beneficiario_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaBeneficiarioSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_beneficiario(&data, params.id_persona).await?;
    let nuevo_beneficiario = sqlx::query_as!(
        BeneficiarioPersonaModelo,
        r#"INSERT INTO beneficiarios_persona
        (id_persona, entre_calle, y_calle)
        VALUES ($1, $2, $3)
        RETURNING
        id_persona_beneficiario, id_persona,entre_calle, y_calle"#,
        params.id_persona,
        body.entre_calle,
        body.y_calle,
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
        "datos": nuevo_beneficiario
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_beneficiario_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<BeneficiarioPersonaModelo, (StatusCode, Json<serde_json::Value>)> {
    let beneficiario_encontrado = sqlx::query_as!(
        BeneficiarioPersonaModelo,
        r#"SELECT id_persona_beneficiario, id_persona,entre_calle, y_calle
        FROM beneficiarios_persona 
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

    Ok(beneficiario_encontrado)
}
