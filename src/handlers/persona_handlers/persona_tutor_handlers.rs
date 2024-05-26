use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::{
        persona_tutor_models::TutorPersonaModelo,
        persona_types::{ClasificacionPersona, RegimenConyugalPersona},
    },
    schemas::persona_schemas::{
        persona_principal_schemas::ObtenerPersonaParams,
        persona_tutor_schemas::CrearPersonaTutorSchema,
    },
    validators::persona_validators::persona_tutor_validators::validar_nueva_persona_tutor,
    AppState,
};

pub async fn crear_nueva_persona_tutor_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaTutorSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_tutor(&data, params.id_persona, &body).await?;
    let nuevo_tutor = sqlx::query_as!(
        TutorPersonaModelo,
        r#"INSERT INTO tutores_persona
        (id_persona, clasificacion, entre_calle, y_calle,
        fecha_residencia, lugar_nacimiento,estado_nacimiento,regimen_conyugal,profesion,
        escolaridad,autorizo_compartir_informacion_ifai, autorizo_publicidad)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12)
        RETURNING
        id_persona_tutor, id_persona, clasificacion AS "clasificacion: ClasificacionPersona",
        entre_calle, y_calle, fecha_residencia, lugar_nacimiento, estado_nacimiento,
        regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona",profesion,escolaridad, 
        autorizo_compartir_informacion_ifai,autorizo_publicidad"#,
        params.id_persona,
        body.clasificacion as ClasificacionPersona,
        body.entre_calle,
        body.y_calle,
        body.fecha_residencia,
        body.lugar_nacimiento,
        body.estado_nacimiento,
        body.regimen_conyugal as RegimenConyugalPersona,
        body.profesion,
        body.escolaridad,
        body.autorizo_compartir_informacion_ifai,
        body.autorizo_publicidad,
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

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_tutor
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_tutor_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<TutorPersonaModelo, (StatusCode, Json<serde_json::Value>)> {
    let tutor_encontrado = sqlx::query_as!(
        TutorPersonaModelo,
        r#"SELECT id_persona_tutor, id_persona,
        clasificacion AS "clasificacion: ClasificacionPersona",entre_calle, y_calle, 
        fecha_residencia, lugar_nacimiento, estado_nacimiento,
        regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona",profesion,escolaridad, 
        autorizo_compartir_informacion_ifai,autorizo_publicidad 
        FROM tutores_persona 
        WHERE id_persona=$1"#,
        id_persona,
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

    Ok(tutor_encontrado)
}
