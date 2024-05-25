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
        persona_tercero_autorizado_models::TerceroAutorizadoPersonaModelo,
        persona_types::{ClasificacionPersona, RegimenConyugalPersona},
    },
    schemas::persona_schemas::{
        persona_principal_schemas::ObtenerPersonaParams,
        persona_tercero_autorizado_schemas::CrearPersonaTerceroAutorizadoSchema,
    },
    validators::persona_validators::persona_tercero_autorizado_validators::validar_nueva_persona_tercero_autorizado,
    AppState,
};

pub async fn crear_nueva_persona_tercero_autorizado_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaTerceroAutorizadoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_tercero_autorizado(&data, params.id_persona, &body).await?;
    let nuevo_tercero_autorizado = sqlx::query_as!(
        TerceroAutorizadoPersonaModelo,
        r#"INSERT INTO tercero_autorizados_persona
        (id_persona, clasificacion,entre_calle, y_calle,
        fecha_residencia, lugar_nacimiento,estado_nacimiento,regimen_conyugal,profesion,
        escolaridad,autorizo_compartir_informacion_ifai, autorizo_publicidad)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12)
        RETURNING
        id_persona_tercero_autorizado, id_persona,
        clasificacion AS "clasificacion: ClasificacionPersona",entre_calle, y_calle, 
        fecha_residencia, lugar_nacimiento, estado_nacimiento,
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
        "datos": nuevo_tercero_autorizado
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_tercero_autorizado_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<TerceroAutorizadoPersonaModelo, (StatusCode, Json<serde_json::Value>)> {
    let tercero_autorizado_encontrado = sqlx::query_as!(
        TerceroAutorizadoPersonaModelo,
        r#"SELECT id_persona_tercero_autorizado, id_persona,
        clasificacion AS "clasificacion: ClasificacionPersona",entre_calle, y_calle, 
        fecha_residencia, lugar_nacimiento, estado_nacimiento,
        regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona",profesion,escolaridad, 
        autorizo_compartir_informacion_ifai,autorizo_publicidad 
        FROM tercero_autorizados_persona 
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

    Ok(tercero_autorizado_encontrado)
}
