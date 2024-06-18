use std::sync::Arc;

use axum::{http::StatusCode, Json};

use crate::{responses::error_responses::error_base_datos, AppState};

pub async fn validar_nueva_relacion_persona(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let persona_existente = sqlx::query_scalar!(
        "SELECT tipo FROM personas WHERE id_persona = $1",
        id_persona
    )
    .fetch_optional(&data.db)
    .await
    .map_err(error_base_datos)?;

    if let Some(tipo) = persona_existente {
        if tipo != 2 {
            let respuesta_error = serde_json::json!({
                "estado": false,
                "mensaje": "Solo los socios pueden tener relaciones",
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

    Ok(())
}
