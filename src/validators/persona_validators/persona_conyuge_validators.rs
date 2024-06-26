use std::sync::Arc;

use axum::{http::StatusCode, Json};

use crate::{
    responses::error_responses::error_base_datos,
    schemas::persona_schemas::persona_conyuge_schemas::CrearPersonaConyugeSchema, AppState,
};

pub async fn validar_nueva_persona_conyuge(
    data: &Arc<AppState>,
    id_persona: i32,
    body: &CrearPersonaConyugeSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let persona_existente = sqlx::query_scalar!(
        "SELECT tipo FROM personas WHERE id_persona = $1",
        id_persona
    )
    .fetch_optional(&data.db)
    .await
    .map_err(error_base_datos)?;

    if let Some(tipo) = persona_existente {
        if tipo != 5 {
            let respuesta_error = serde_json::json!({
                "estado": false,
                "mensaje": "El tipo de persona no coincide",
            });
            return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
        }
    } else {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El ID de la persona no existe",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.lugar_nacimiento.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El lugar de nacimiento no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    Ok(())
}
