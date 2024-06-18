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
        persona_sucursal_models::SucursalPersonaModelo,
        persona_types::{ClasificacionPersona, RegimenConyugalPersona},
    },
    responses::error_responses::error_base_datos,
    schemas::persona_schemas::{
        persona_principal_schemas::ObtenerPersonaParams,
        persona_sucursal_schemas::CrearPersonaSucursalSchema,
    },
    validators::persona_validators::persona_sucursal_validators::validar_nueva_persona_sucursal,
    AppState,
};

pub async fn crear_nueva_persona_sucursal_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaSucursalSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_sucursal(&data, params.id_persona, &body).await?;
    let nueva_sucursal = sqlx::query_as!(
        SucursalPersonaModelo,
        r#"INSERT INTO sucursales_persona
        (id_persona, clasificacion, entre_calle, y_calle,
        fecha_residencia, lugar_nacimiento,estado_nacimiento,regimen_conyugal,profesion,
        escolaridad,autorizo_compartir_informacion_ifai, autorizo_publicidad)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12)
        RETURNING
        id_persona_sucursal, id_persona, clasificacion AS "clasificacion: ClasificacionPersona",
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
    .map_err(error_base_datos)?;

    let respuesta = json!({
        "estado": true,
        "datos": nueva_sucursal
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_sucursal_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<SucursalPersonaModelo, (StatusCode, Json<serde_json::Value>)> {
    let sucursal_encontrada = sqlx::query_as!(
        SucursalPersonaModelo,
        r#"SELECT id_persona_sucursal, id_persona,
        clasificacion AS "clasificacion: ClasificacionPersona",entre_calle, y_calle, 
        fecha_residencia, lugar_nacimiento, estado_nacimiento,
        regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona",profesion,escolaridad, 
        autorizo_compartir_informacion_ifai,autorizo_publicidad 
        FROM sucursales_persona 
        WHERE id_persona=$1"#,
        id_persona,
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(sucursal_encontrada)
}
