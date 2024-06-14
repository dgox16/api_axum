use axum::{http::StatusCode, Json};

use crate::schemas::trabajo_schemas::CrearPuestoTrabajoSchema;

pub fn validar_nuevo_puesto_trabajo(
    body: &CrearPuestoTrabajoSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre del puesto de trabajo no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}
