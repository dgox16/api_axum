use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::{
    models::{
        persona_models::{EstadoCivilPersona, PersonaModelo, SexoPersona, ViviendaPersona},
        user_models::UsuarioModelo,
    },
    schemas::persona_schemas::CrearPersonaSchema,
    AppState,
};

pub async fn crear_nueva_persona_handler(
    State(data): State<Arc<AppState>>,
    Extension(usuario): Extension<UsuarioModelo>,
    Json(body): Json<CrearPersonaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let nueva_persona = sqlx::query_as!(
        PersonaModelo,
        r#"INSERT INTO personas 
        (nombre,apellido_paterno,apellido_materno,tipo,sexo,
        usuario_actualizo,cp,barrio,ciudad,calle,numero_exterior,
        numero_interior,vivienda,geolocalizacion,
        observaciones_geolocalizacion,fecha_nacimiento,
        pais_nacimiento,estado_civil,persona_conyuge,
        es_embargo_precautorio,bloqueado_autoridad,tercero_autorizado)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12,$13,
        $14,$15,$16,$17,$18,$19,$20,$21,$22) RETURNING
        id_persona,nombre,apellido_paterno,apellido_materno,tipo,
        sexo AS "sexo: SexoPersona",fecha_actualizacion,
        usuario_actualizo,cp,barrio,ciudad,calle,numero_exterior,
        numero_interior,vivienda AS "vivienda: ViviendaPersona",
        geolocalizacion,observaciones_geolocalizacion,
        fecha_nacimiento,pais_nacimiento,
        estado_civil AS "estado_civil: EstadoCivilPersona",
        persona_conyuge,es_embargo_precautorio,bloqueado_autoridad,tercero_autorizado"#,
        body.nombre,
        body.apellido_paterno,
        body.apellido_materno,
        body.tipo,
        body.sexo as SexoPersona,
        usuario.id,
        body.cp,
        body.barrio,
        body.ciudad,
        body.calle,
        body.numero_exterior,
        body.numero_interior,
        body.vivienda as ViviendaPersona,
        body.geolocalizacion,
        body.observaciones_geolocalizacion,
        body.fecha_nacimiento,
        body.pais_nacimiento,
        body.estado_civil as EstadoCivilPersona,
        body.persona_conyuge,
        body.es_embargo_precautorio,
        body.bloqueado_autoridad,
        body.tercero_autorizado
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
        "datos": nueva_persona
    });

    Ok(Json(respuesta))
}
