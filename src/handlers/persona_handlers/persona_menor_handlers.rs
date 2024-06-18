use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::persona_models::persona_menor_models::MenorPersonaModelo,
    responses::error_responses::error_base_datos,
    schemas::persona_schemas::{
        persona_menor_schema::CrearPersonaMenorSchema,
        persona_principal_schemas::ObtenerPersonaParams,
    },
    validators::persona_validators::persona_menor_validators::validar_nueva_persona_menor,
    AppState,
};

pub async fn crear_nueva_persona_menor_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaMenorSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_menor(&data, params.id_persona, &body).await?;
    let nuevo_menor = sqlx::query_as!(
        MenorPersonaModelo,
        r#"INSERT INTO menores_persona
        (id_persona,entre_calle, y_calle,fecha_residencia,lugar_nacimiento,estado_nacimiento,
        escolaridad,autorizo_compartir_informacion_ifai, autorizo_publicidad)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9)
        RETURNING
        id_persona_menor, id_persona,entre_calle, y_calle,fecha_residencia, lugar_nacimiento,
        estado_nacimiento,escolaridad,autorizo_compartir_informacion_ifai,autorizo_publicidad"#,
        params.id_persona,
        body.entre_calle,
        body.y_calle,
        body.fecha_residencia,
        body.lugar_nacimiento,
        body.estado_nacimiento,
        body.escolaridad,
        body.autorizo_compartir_informacion_ifai,
        body.autorizo_publicidad,
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_menor
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_menor_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<MenorPersonaModelo, (StatusCode, Json<serde_json::Value>)> {
    let menor_encontrado = sqlx::query_as!(
        MenorPersonaModelo,
        r#"SELECT 
        id_persona_menor, id_persona,entre_calle, y_calle,fecha_residencia, lugar_nacimiento,
        estado_nacimiento,escolaridad,autorizo_compartir_informacion_ifai,autorizo_publicidad
        FROM menores_persona 
        WHERE id_persona=$1"#,
        id_persona,
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(menor_encontrado)
}
