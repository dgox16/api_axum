use std::sync::Arc;

use axum::{http::StatusCode, Json};

use crate::{schemas::poliza_schema::CrearPolizaEgresoSchema, AppState};

pub async fn validar_nueva_poliza_egreso(
    data: &Arc<AppState>,
    poliza_egreso: &CrearPolizaEgresoSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    // Verificar que el campo de beneficiario no esté vacío
    if poliza_egreso.beneficiario.is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El campo de beneficiario no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    // Verificar que el ID del banco exista en la base de datos
    let banco_existente: Option<i32> = sqlx::query_scalar!(
        "SELECT id_banco FROM bancos WHERE id_banco = $1",
        poliza_egreso.banco
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    if banco_existente.is_none() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El ID del banco proporcionado no existe",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    if poliza_egreso.cheque.is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El campo de cheque no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    Ok(())
}
