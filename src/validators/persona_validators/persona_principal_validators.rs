use axum::{http::StatusCode, Json};

use crate::schemas::persona_schemas::persona_principal_schemas::CrearPersonaSchema;

pub fn validar_nueva_persona(
    body: &CrearPersonaSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de nombre no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.apellido_paterno.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de apellido paterno no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.apellido_materno.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de apellido materno no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cp.trim().is_empty() || body.cp.trim().len() != 5 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El codigo postal no es valido",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.numero_exterior.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo del numero exterior no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.geolocalizacion.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de la geolocalizacion no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.observaciones_geolocalizacion.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de observaciones de geolocalizacion no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    Ok(())
}
