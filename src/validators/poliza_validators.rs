use std::sync::Arc;

use axum::{http::StatusCode, Json};

use crate::{
    responses::error_responses::error_base_datos,
    schemas::poliza_schema::{
        CrearDetallePolizaSchema, CrearPolizaEgresoSchema, CrearPolizaSchema,
    },
    AppState,
};

pub async fn validar_nueva_poliza_egreso(
    data: &Arc<AppState>,
    poliza_egreso: &CrearPolizaEgresoSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    // Verificar que el campo de beneficiario no esté vacío
    if poliza_egreso.beneficiario.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
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
    .map_err(error_base_datos)?;

    if banco_existente.is_none() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El ID del banco proporcionado no existe",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    if poliza_egreso.cheque.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de cheque no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    Ok(())
}

pub fn validar_nueva_poliza(
    body: &CrearPolizaSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.concepto.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El concepto de la poliza no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.numero.is_some() && body.numero.unwrap() < 1 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El numero de la poliza debe ser mayor o igual a 1",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nuevo_detalle_poliza(
    body: &CrearDetallePolizaSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.concepto.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El concepto del detalle de poliza no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cargo < 0.00 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El cargo no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.abono < 0.00 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El abono no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}
