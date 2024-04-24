use axum::{http::StatusCode, Json};

use crate::schemas::sucursal_schemas::NuevoBancoSchema;

pub fn validar_nuevo_banco_schema(
    body: &NuevoBancoSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El nombre del banco no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}
