use axum::{http::StatusCode, Json};

use crate::schemas::ubicacion_schemas::CrearCalleSchema;

pub fn validar_nueva_calle(
    body: &CrearCalleSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre de la calle no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}
