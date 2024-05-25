use std::sync::Arc;

use axum::{http::StatusCode, Json};

use crate::{
    schemas::persona_schemas::persona_propietario_real_schemas::CrearPersonaPropietarioRealSchema,
    AppState,
};

pub async fn validar_nueva_persona_propietario_real(
    data: &Arc<AppState>,
    id_persona: i32,
    body: &CrearPersonaPropietarioRealSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let persona_existente = sqlx::query_scalar!(
        "SELECT tipo FROM personas WHERE id_persona = $1",
        id_persona
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    if let Some(tipo) = persona_existente {
        if tipo != 9 {
            let respuesta_error = serde_json::json!({
                "estado": "error",
                "mensaje": "El tipo de persona no coincide",
            });
            return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
        }
    } else {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": "El ID de la persona no existe",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
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
