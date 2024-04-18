use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::domicilio_models::{CalleModel, TipoCalle},
    responses::domicilio_responses::NewCalleResponse,
    schemas::domicilio_schemas::CreateNewCalleSchema,
    AppState,
};

pub fn new_calle_response(calle: &CalleModel) -> Result<NewCalleResponse, sqlx::Error> {
    let tipo_calle = match calle.tipo {
        TipoCalle::CA => "CA",
        TipoCalle::AV => "AV",
        TipoCalle::PR => "PR",
        TipoCalle::CE => "CE",
        TipoCalle::PV => "PV",
        TipoCalle::CZ => "CZ",
    };

    Ok(NewCalleResponse {
        id_calle: calle.id_calle.to_string(),
        nombre: calle.nombre.clone(),
        tipo: tipo_calle.to_string(),
    })
}
// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
pub struct PalabraClaveQuery {
    palabra: Option<String>,
}

pub async fn search_calle(
    State(data): State<Arc<AppState>>,
    Query(query): Query<PalabraClaveQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match query.palabra {
        Some(palabra) => {
            let calles = sqlx::query_as!(
        CalleModel,
        r#"SELECT id_calle,nombre,tipo AS "tipo: TipoCalle" FROM calles WHERE nombre ILIKE '%' || $1 || '%' LIMIT 20"#,
        palabra
    )
    .fetch_all(&data.db)
    .await
.map_err(|e| {
            let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

            let new_calles: Vec<NewCalleResponse> = calles
                .into_iter()
                .map(|calle| new_calle_response(&calle))
                .collect::<Result<_, _>>()
                .map_err(|e| {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("Database error: {}", e),
                    });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
                })?;

            let response = json!({
                "status": "success",
                "data": new_calles
            });
            Ok(Json(response))
        }
        _ => {
            let calles = sqlx::query_as!(
                CalleModel,
                r#"SELECT id_calle,nombre,tipo AS "tipo: TipoCalle" FROM calles LIMIT 20"#,
            )
            .fetch_all(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

            let new_calles: Vec<NewCalleResponse> = calles
                .into_iter()
                .map(|calle| new_calle_response(&calle))
                .collect::<Result<_, _>>()
                .map_err(|e| {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("Database error: {}", e),
                    });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
                })?;

            let response = json!({
                "status": "success",
                "data": new_calles
            });
            Ok(Json(response))
        }
    }
}

pub async fn create_new_calle(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNewCalleSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let tipo_calle_str = match body.tipo.as_str() {
        "CA" => TipoCalle::CA,
        "AV" => TipoCalle::AV,
        "PR" => TipoCalle::PR,
        "CE" => TipoCalle::CE,
        "PV" => TipoCalle::PV,
        "CZ" => TipoCalle::CZ,
        _ => TipoCalle::CA,
    };

    let new_calle = sqlx::query_as!(
        CalleModel,
        r#"INSERT INTO calles (nombre, tipo) VALUES ($1, $2) RETURNING id_calle,nombre,tipo AS "tipo: TipoCalle""#,
        body.nombre.to_string(),
        tipo_calle_str as TipoCalle
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let response_calle = new_calle_response(&new_calle);
    match response_calle {
        Ok(calle) => {
            let response = json!({
                "status": "success",
                "data":calle
            });
            Ok(Json(response))
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}
