use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::tutores_de_persona_models::{
        QuienEsTutor, TipoTutor, TutorDePersonaModelo,
    },
    schemas::persona_schemas::{
        persona_principal_schemas::ObtenerPersonaParams,
        tutores_de_persona_schemas::CrearTutorDePersonaSchema,
    },
    validators::persona_validators::tutores_de_persona_validators::validar_nuevo_tutor_de_persona,
    AppState,
};

pub async fn crear_nuevo_tutor_de_persona_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<Vec<CrearTutorDePersonaSchema>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut tutores_creados = Vec::new();
    validar_nuevo_tutor_de_persona(&data, params.id_persona, &body).await?;
    for tutor in body {
        let nuevo_tutor = sqlx::query_as!(
            TutorDePersonaModelo,
            r#"INSERT INTO tutores_de_persona
            (id_persona, tutor, tutor_migrado, quien_es_tutor, documento_legal, documento, tipo)
            VALUES ($1, $2, $3, $4, $5, $6, $7 )
            RETURNING 
            id_tutor_de_persona, id_persona, tutor, tutor_migrado,
            quien_es_tutor AS "quien_es_tutor: QuienEsTutor", documento_legal, documento,
            tipo AS "tipo: TipoTutor""#,
            params.id_persona,
            tutor.tutor,
            tutor.tutor_migrado,
            tutor.quien_es_tutor as QuienEsTutor,
            tutor.documento_legal,
            tutor.documento,
            tutor.tipo as TipoTutor,
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
        tutores_creados.push(nuevo_tutor);
    }

    let respuesta = json!({
        "estado": true,
        "datos": tutores_creados
    });

    Ok(Json(respuesta))
}

pub async fn obtener_tutores_de_persona_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<Vec<TutorDePersonaModelo>, (StatusCode, Json<serde_json::Value>)> {
    let tutores_encontrados = sqlx::query_as!(
        TutorDePersonaModelo,
        r#"SELECT 
        id_tutor_de_persona, id_persona, tutor, tutor_migrado,
        quien_es_tutor AS "quien_es_tutor: QuienEsTutor", documento_legal, documento,
        tipo AS "tipo: TipoTutor"
        FROM tutores_de_persona WHERE id_persona=$1
        "#,
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

    Ok(tutores_encontrados)
}
