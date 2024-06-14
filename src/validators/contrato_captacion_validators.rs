use std::sync::Arc;

use axum::{http::StatusCode, Json};

use crate::{schemas::contratos_captacion_schemas::CrearContratoCaptacionSchema, AppState};

pub async fn validar_nueva_contrato_captacion(
    data: &Arc<AppState>,
    body: &CrearContratoCaptacionSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let persona_existente = sqlx::query_scalar!(
        "SELECT tipo FROM personas WHERE id_persona = $1",
        body.id_persona
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    if let Some(tipo) = persona_existente {
        if tipo != 2 {
            let respuesta_error = serde_json::json!({
                "estado": false,
                "mensaje": "Solo los socios pueden tener contratos de captacion",
            });
            return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
        }
    } else {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El ID de la persona no existe",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cuenta.trim().is_empty() || body.cuenta.trim().len() != 16 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "Formato incorrecto en la cuenta ingresada",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.monto_autorizado < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El monto autorizado no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.numero_sesion.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de numero de sesion no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.tipo_sesion.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de tipo de sesion no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo del nombre no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.contrato_migrado < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El contrato migrado no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.monto_libera_garantia < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El monto para liberar garantia no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.tasa_pactada < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La tasa pactada no puede ser negativa",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.monto_bloqueado_adicional < 0.0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El monto bloqueado adicional no puede ser negativo",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    Ok(())
}
