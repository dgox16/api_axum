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
        persona_aspirante_models::AspirantePersonaModelo,
        persona_types::{ClasificacionPersona, PeriodoPersona},
    },
    schemas::persona_schemas::{
        persona_aspirante_schemas::CrearPersonaAspiranteSchema,
        persona_principal_schemas::ObtenerPersonaParams,
    },
    validators::persona_validators::persona_aspirante_validators::validar_nueva_persona_aspirante,
    AppState,
};

pub async fn crear_nueva_persona_aspirante_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaAspiranteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_aspirante(&body)?;
    let nuevo_aspirante = sqlx::query_as!(
        AspirantePersonaModelo,
        r#"INSERT INTO aspirantes_persona
        (id_persona, clasificacion, ocupacion_pld, especificacion_pld, actividad_pld, 
        antiguedad, periodo, frecuencia_captacion, operacion_maxima_captacion,
        perfil_frecuencia_prestamo, operacion_maxima_prestamo,
        ingresos_mensual, egresos_mensual, grado_afectacion, afectacion,
        entre_calle, y_calle, fecha_residencia, lugar_nacimiento, estado_nacimiento, profesion,
        escolaridad, autorizo_compartir_informacion_ifai, autorizo_publicidad)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12,$13,
        $14,$15,$16,$17,$18,$19,$20,$21,$22,$23, $24) RETURNING
        id_persona_aspirante, id_persona, clasificacion AS "clasificacion: ClasificacionPersona",
        ocupacion_pld, especificacion_pld, antiguedad, actividad_pld,
        periodo AS "periodo: PeriodoPersona", frecuencia_captacion, operacion_maxima_captacion,
        perfil_frecuencia_prestamo, operacion_maxima_prestamo,
        ingresos_mensual, egresos_mensual, grado_afectacion, afectacion,
        entre_calle, y_calle, fecha_residencia, lugar_nacimiento, estado_nacimiento, profesion,
        escolaridad, autorizo_compartir_informacion_ifai, autorizo_publicidad"#,
        params.id_persona,
        body.clasificacion as ClasificacionPersona,
        body.ocupacion_pld,
        body.especificacion_pld,
        body.actividad_pld,
        body.antiguedad,
        body.periodo as PeriodoPersona,
        body.frecuencia_captacion,
        body.operacion_maxima_captacion,
        body.perfil_frecuencia_prestamo,
        body.operacion_maxima_prestamo,
        body.ingresos_mensual,
        body.egresos_mensual,
        body.grado_afectacion,
        body.afectacion,
        body.entre_calle,
        body.y_calle,
        body.fecha_residencia,
        body.lugar_nacimiento,
        body.estado_nacimiento,
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
        "datos": nuevo_aspirante
    });

    Ok(Json(respuesta))
}
