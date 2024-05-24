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
        persona_aval_models::AvalPersonaModelo,
        persona_types::{ClasificacionPersona, RegimenConyugalPersona},
    },
    schemas::persona_schemas::{
        persona_aval_schemas::CrearPersonaAvalSchema,
        persona_principal_schemas::ObtenerPersonaParams,
    },
    AppState,
};

pub async fn crear_nueva_persona_aval_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaAvalSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nuevo_aval = sqlx::query_as!(
        AvalPersonaModelo,
        r#"INSERT INTO avales_persona
        (id_persona, clasificacion, socio_migrado,entre_calle, y_calle,fecha_residencia,
        lugar_nacimiento,estado_nacimiento,regimen_conyugal,profesion,escolaridad,
        autorizo_compartir_informacion_ifai, autorizo_publicidad,fecha_bloqueo,usuario_bloqueo)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
        RETURNING
        id_persona_aval, id_persona, clasificacion AS "clasificacion: ClasificacionPersona",
        socio_migrado,entre_calle, y_calle,fecha_residencia, lugar_nacimiento,
        estado_nacimiento,regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona",
        profesion,escolaridad, autorizo_compartir_informacion_ifai,autorizo_publicidad,
        fecha_bloqueo,usuario_bloqueo"#,
        params.id_persona,
        body.clasificacion as ClasificacionPersona,
        body.socio_migrado,
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
        body.fecha_bloqueo,
        body.usuario_bloqueo
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
        "datos": nuevo_aval
    });

    Ok(Json(respuesta))
}
