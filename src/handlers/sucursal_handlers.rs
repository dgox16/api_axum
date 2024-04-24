use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    models::sucursal_models::{BancoModel, SucursalModel},
    schemas::sucursal_schemas::{NuevaSucursalSchema, NuevoBancoSchema},
    validators::sucursal_validators::validar_nuevo_banco_schema,
    AppState,
};

pub async fn crear_nueva_sucursal_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<NuevaSucursalSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nueva_sucursal = sqlx::query_as!(
        SucursalModel,
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
    Json(body): Json<NuevoBancoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nuevo_banco_schema(&body)?;
    let nuevo_banco = sqlx::query_as!(
        BancoModel,
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
