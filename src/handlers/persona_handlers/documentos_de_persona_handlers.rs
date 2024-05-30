use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::documentos_de_persona_models::DocumentoDePersonaModelo,
    schemas::persona_schemas::{
        documentos_de_persona_schemas::CrearDocumentoDePersonaSchema,
        persona_principal_schemas::ObtenerPersonaParams,
    },
    validators::persona_validators::documentos_de_persona_validators::validar_nuevo_documento_de_persona,
    AppState,
};

pub async fn crear_nuevo_documento_de_persona_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<Vec<CrearDocumentoDePersonaSchema>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut documentos_creados = Vec::new();
    validar_nuevo_documento_de_persona(&body)?;
    for documento in body {
        let nuevo_documento = sqlx::query_as!(
            DocumentoDePersonaModelo,
            r#"INSERT INTO documentos_de_persona
            (id_persona, documento, numero_identificacion, es_identificacion, vigencia)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING 
            id_documento_de_persona, id_persona, documento, numero_identificacion,
            es_identificacion, vigencia"#,
            params.id_persona,
            documento.documento,
            documento.numero_identificacion,
            documento.es_identificacion,
            documento.vigencia
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "estado": false,
                    "mensaje": format!("Error en la base de datos: {}", e),
                })),
            )
        })?;
        documentos_creados.push(nuevo_documento);
    }

    let respuesta = json!({
        "estado": true,
        "datos": documentos_creados
    });

    Ok(Json(respuesta))
}

pub async fn obtener_documentos_de_persona_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<Vec<DocumentoDePersonaModelo>, (StatusCode, Json<serde_json::Value>)> {
    let documentos_encontrados = sqlx::query_as!(
        DocumentoDePersonaModelo,
        r#"SELECT 
        id_documento_de_persona, id_persona, documento, numero_identificacion,
        es_identificacion, vigencia
        FROM documentos_de_persona WHERE id_persona=$1"#,
        id_persona,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )
    })?;

    Ok(documentos_encontrados)
}
