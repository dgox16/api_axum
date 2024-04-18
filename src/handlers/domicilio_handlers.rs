use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::domicilio_models::{CalleModelo, TipoCalle},
    responses::domicilio_responses::CalleRespuesta,
    schemas::domicilio_schemas::{BuscarCalleQuery, CrearNuevaCalleSchema},
    AppState,
};

pub fn formatear_calle(calle: &CalleModelo) -> Result<CalleRespuesta, sqlx::Error> {
    let tipo_calle = match calle.tipo {
        TipoCalle::CA => "CA",
        TipoCalle::AV => "AV",
        TipoCalle::PR => "PR",
        TipoCalle::CE => "CE",
        TipoCalle::PV => "PV",
        TipoCalle::CZ => "CZ",
    };

    Ok(CalleRespuesta {
        id_calle: calle.id_calle.to_string(),
        nombre: calle.nombre.clone(),
        tipo: tipo_calle.to_string(),
    })
}

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

            let calles_formateadas: Vec<CalleRespuesta> = calles_encontradas
                .into_iter()
                .map(|calle| formatear_calle(&calle))
                .collect::<Result<_, _>>()
                .map_err(|e| {
                    let respuesta_error = serde_json::json!({
                        "estado": "error",
                        "mensaje": format!("Fallo con las calles encontradas: {}", e),
                    });
                    (StatusCode::CONFLICT, Json(respuesta_error))
                })?;

            let respuesta = json!({
                "status": "exitoso",
                "data": calles_formateadas
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

            let calles_formateadas: Vec<CalleRespuesta> = calles_encontradas
                .into_iter()
                .map(|calle| formatear_calle(&calle))
                .collect::<Result<_, _>>()
                .map_err(|e| {
                    let respuesta_error = serde_json::json!({
                        "estado": "error",
                        "mensaje": format!("Fallo con las calles encontradas: {}", e),
                    });
                    (StatusCode::CONFLICT, Json(respuesta_error))
                })?;

            let response = json!({
                "status": "success",
                "data": calles_formateadas
            });
            Ok(Json(response))
        }
    }
}

pub async fn crear_nueva_calle_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearNuevaCalleSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let tipo_calle = match body.tipo.as_str() {
        "CA" => TipoCalle::CA,
        "AV" => TipoCalle::AV,
        "PR" => TipoCalle::PR,
        "CE" => TipoCalle::CE,
        "PV" => TipoCalle::PV,
        "CZ" => TipoCalle::CZ,
        _ => TipoCalle::CA,
    };

    let nueva_calle = sqlx::query_as!(
        CalleModelo,
        r#"INSERT INTO calles (nombre, tipo) VALUES ($1, $2) RETURNING id_calle,nombre,tipo AS "tipo: TipoCalle""#,
        body.nombre.to_string(),
        tipo_calle as TipoCalle
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

    let calle_formateada = formatear_calle(&nueva_calle);
    match calle_formateada {
        Ok(calle) => {
            let respuesta = json!({
                "estado": "exitoso",
                "data":calle
            });
            Ok(Json(respuesta))
        }
        Err(e) => {
            let respuesta_error = serde_json::json!({
                "estado": "error",
                "mensaje": format!("Fallo con la nueva calle: {}", e),
            });
            Err((StatusCode::CONFLICT, Json(respuesta_error)))
        }
    }
}
