use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::ubicacion_models::{CalleModelo, DomicilioModel, TipoCalle},
    schemas::ubicacion_schemas::{BuscarCalleQuery, CrearCalleSchema, CrearDomicilioSchema},
    AppState,
};

pub async fn buscar_calle_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarCalleQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let limite = query.limite.unwrap_or(20);
    match query.palabra {
        Some(palabra) => {
            let calles_encontradas = sqlx::query_as!(
                CalleModelo,
                r#"SELECT id_calle,nombre,tipo AS "tipo: TipoCalle" FROM calles WHERE nombre ILIKE '%' || $1 || '%' LIMIT $2"#,
                palabra,
                limite
            )
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                let respuesta_error = serde_json::json!({
                    "estado": "error",
                    "mensaje": format!("Error en la base de datos: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
            })?;

            let respuesta = json!({
                "status": "exitoso",
                "data": calles_encontradas
            });
            Ok(Json(respuesta))
        }
        _ => {
            let calles_encontradas = sqlx::query_as!(
                CalleModelo,
                r#"SELECT id_calle,nombre,tipo AS "tipo: TipoCalle" FROM calles LIMIT $1"#,
                limite
            )
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                let respuesta_error = serde_json::json!({
                    "estado": "error",
                    "mensaje": format!("Error en la base de datos: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
            })?;

            let respuesta = json!({
                "status": "success",
                "data": calles_encontradas
            });
            Ok(Json(respuesta))
        }
    }
}

pub async fn crear_nueva_calle_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearCalleSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nueva_calle = sqlx::query_as!(
        CalleModelo,
        r#"INSERT INTO calles (nombre, tipo) VALUES ($1, $2) RETURNING id_calle,nombre,tipo AS "tipo: TipoCalle""#,
        body.nombre.to_string(),
        body.tipo as TipoCalle
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
        "data":nueva_calle
    });
    Ok(Json(respuesta))
}

pub async fn crear_nuevo_domicilio_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearDomicilioSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nuevo_domicilio = sqlx::query_as!(
        DomicilioModel,
        "INSERT INTO domicilios
        (cp,colonia,calle_id,entre_calle_id,y_calle_id,numero_exterior,numero_interior,geolocalizacion)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *",
        body.cp.to_string(),
        body.colonia.to_string(),
        body.calle_id,
        body.entre_calle_id,
        body.y_calle_id,
        body.numero_exterior.to_string(),
        body.numero_interior,
        body.geolocalizacion
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
        "data": nuevo_domicilio
    });
    Ok(Json(respuesta))
}
