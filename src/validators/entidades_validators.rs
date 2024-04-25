use axum::{http::StatusCode, Json};

use crate::schemas::entidades_schemas::{CrearBancoSchema, CrearSucursalSchema};

pub fn validar_nuevo_banco(
    body: &CrearBancoSchema,
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

pub fn validar_nueva_sucursal(
    body: &CrearSucursalSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El nombre de la sucursal no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}
