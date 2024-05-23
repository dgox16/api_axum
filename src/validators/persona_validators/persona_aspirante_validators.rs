use axum::{http::StatusCode, Json};

use crate::schemas::persona_schemas::persona_aspirante_schemas::CrearPersonaAspiranteSchema;

pub fn validar_nueva_persona_aspirante(
    body: &CrearPersonaAspiranteSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.frecuencia_captacion < 0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "La frecuencia de la captacion no puede ser negativa",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.operacion_maxima_captacion < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "La operacion maxima de captacion no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.perfil_frecuencia_prestamo < 0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El perfil de frecuencia de prestamo no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.operacion_maxima_prestamo < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "La operacion maxima de prestamo no puede ser negativa",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.ingresos_mensual < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "Los ingresos mensuales no pueden ser negativos",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.egresos_mensual < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "Los egresos mensuales no pueden ser negativos",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.grado_afectacion < 0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El grado de afectacion no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.afectacion < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "La afectacion no puede ser negativa",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.lugar_nacimiento.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El lugar de nacimiento no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    Ok(())
}
