use axum::{http::StatusCode, Json};

use crate::schemas::persona_schemas::persona_aval_schemas::CrearPersonaAvalSchema;

pub fn validar_nueva_persona_aval(
    body: &CrearPersonaAvalSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.socio_migrado.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El socio migrado no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.lugar_nacimiento.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El lugar de nacimiento no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    Ok(())
}
