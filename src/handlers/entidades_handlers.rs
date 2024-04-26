use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    models::entidades_models::{BancoModelo, SucursalModelo},
    schemas::entidades_schemas::{CrearBancoSchema, CrearSucursalSchema},
    validators::entidades_validators::{validar_nueva_sucursal, validar_nuevo_banco},
    AppState,
};

pub async fn crear_nueva_sucursal_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearSucursalSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_sucursal(&body)?;
    let nueva_sucursal = sqlx::query_as!(
        SucursalModelo,
        "INSERT INTO sucursales (nombre, encargado, domicilio) VALUES ($1,$2,$3) RETURNING *",
        body.nombre,
        body.encargado,
        body.domicilio
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": "exitoso",
        "data": nueva_sucursal
    });
    Ok(Json(respuesta))
}
pub async fn crear_nuevo_banco_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearBancoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nuevo_banco(&body)?;
    let nuevo_banco = sqlx::query_as!(
        BancoModelo,
        "INSERT INTO bancos (nombre) VALUES ($1) RETURNING *",
        body.nombre.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": "exitoso",
        "data": nuevo_banco
    });
    Ok(Json(respuesta))
}
