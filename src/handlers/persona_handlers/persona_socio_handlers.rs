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
        persona_socio_models::SocioPersonaModelo,
        persona_types::{
            ClasificacionPersona, EsPropietarioPersona, PeriodoPersona, RegimenConyugalPersona,
        },
    },
    schemas::persona_schemas::{
        persona_principal_schemas::ObtenerPersonaParams,
        persona_socio_schemas::CrearPersonaSocioSchema,
    },
    validators::persona_validators::persona_socio_validators::validar_nueva_persona_socio,
    AppState,
};

pub async fn crear_nueva_persona_socio_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPersonaParams>,
    Json(body): Json<CrearPersonaSocioSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_persona_socio(&body)?;
    let nuevo_socio = sqlx::query_as!(
        SocioPersonaModelo,
        r#"INSERT INTO socios_persona
        (id_persona, clasificacion, ocupacion_pld, especificacion_pld, actividad_pld, 
        antiguedad, periodo, frecuencia_captacion, operacion_maxima_captacion,
        perfil_frecuencia_prestamo, operacion_maxima_prestamo,ingresos_mensual, egresos_mensual,
        grado_afectacion, afectacion,proveedor_recursos, parentesco, persona_recomendo,
        manera_enterarse, lengua, empresa, puesto, fecha_trabajo, ingreso_ordinario,
        otros_ingresos, es_propietario,entre_calle, y_calle, fecha_residencia, lugar_nacimiento,
        estado_nacimiento,regimen_conyugal,profesion,escolaridad,
        autorizo_compartir_informacion_ifai, autorizo_publicidad)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,
        $19,$20,$21,$22,$23, $24,$25,$26,$27,$28,$29,$30,$31,$32,$33,$34,$35,$36)
        RETURNING
        id_persona_socio, id_persona, clasificacion AS "clasificacion: ClasificacionPersona",
        ocupacion_pld, especificacion_pld, antiguedad, actividad_pld,
        periodo AS "periodo: PeriodoPersona", frecuencia_captacion, operacion_maxima_captacion,
        perfil_frecuencia_prestamo, operacion_maxima_prestamo,
        ingresos_mensual, egresos_mensual, grado_afectacion, afectacion,
        proveedor_recursos, parentesco, persona_recomendo, manera_enterarse, lengua, empresa,
        puesto, fecha_trabajo, ingreso_ordinario, otros_ingresos, 
        es_propietario AS "es_propietario: EsPropietarioPersona",entre_calle, y_calle, 
        fecha_residencia, lugar_nacimiento, estado_nacimiento,
        regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona",profesion,escolaridad, 
        autorizo_compartir_informacion_ifai,autorizo_publicidad"#,
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
        body.proveedor_recursos,
        body.parentesco,
        body.persona_recomendo,
        body.manera_enterarse,
        body.lengua,
        body.empresa,
        body.puesto,
        body.fecha_trabajo,
        body.ingreso_ordinario,
        body.otros_ingresos,
        body.es_propietario as EsPropietarioPersona,
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
        "datos": nuevo_socio
    });

    Ok(Json(respuesta))
}

pub async fn obtener_persona_socio_handler(
    data: &Arc<AppState>,
    id_persona: i32,
) -> Result<SocioPersonaModelo, (StatusCode, Json<serde_json::Value>)> {
    let socio_encontrado = sqlx::query_as!(
        SocioPersonaModelo,
        r#"SELECT id_persona_socio, id_persona, 
        clasificacion AS "clasificacion: ClasificacionPersona",ocupacion_pld, especificacion_pld,
        antiguedad, actividad_pld,periodo AS "periodo: PeriodoPersona", frecuencia_captacion,
        operacion_maxima_captacion,perfil_frecuencia_prestamo, operacion_maxima_prestamo,
        ingresos_mensual, egresos_mensual, grado_afectacion, afectacion,
        proveedor_recursos, parentesco, persona_recomendo, manera_enterarse, lengua, empresa,
        puesto, fecha_trabajo, ingreso_ordinario, otros_ingresos, 
        es_propietario AS "es_propietario: EsPropietarioPersona",entre_calle, y_calle, 
        fecha_residencia, lugar_nacimiento, estado_nacimiento,
        regimen_conyugal AS "regimen_conyugal: RegimenConyugalPersona",profesion,escolaridad, 
        autorizo_compartir_informacion_ifai,autorizo_publicidad 
        FROM socios_persona 
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

    Ok(socio_encontrado)
}
