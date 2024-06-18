use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    models::trabajo_models::PuestoTrabajoModelo, responses::error_responses::error_base_datos,
    schemas::trabajo_schemas::CrearPuestoTrabajoSchema,
    validators::trabajo_validators::validar_nuevo_puesto_trabajo, AppState,
};

pub async fn crear_puesto_trabajo_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearPuestoTrabajoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nuevo_puesto_trabajo(&body)?;
    let nuevo_puesto = sqlx::query_as!(
        PuestoTrabajoModelo,
        "INSERT INTO puestos_trabajo
        (nombre) VALUES ($1) 
        RETURNING id_puesto_trabajo, nombre",
        body.nombre
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_puesto
    });

    Ok(Json(respuesta))
}
