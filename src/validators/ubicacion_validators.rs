use axum::{http::StatusCode, Json};

use crate::schemas::ubicacion_schemas::{
    CrearCalleSchema, CrearCiudadSchema, CrearDomicilioSchema, CrearMunicipioSchema,
};

pub fn validar_nueva_calle(
    body: &CrearCalleSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre de la calle no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nueva_domicilio(
    body: &CrearDomicilioSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.cp.trim().is_empty() || body.cp.trim().len() != 5 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El codigo postal no puede estar vacio ni tener una longitud diferente a 5",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.colonia.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo de la colonia no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.numero_exterior.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El numero exterior debe ser indicado",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nuevo_municipio(
    body: &CrearMunicipioSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre del municipio no puede estar vacio"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.estado < 1 || body.estado > 32 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El estado no existe"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.factor_riesgo < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El factor de riesgo no puede ser negativo"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nueva_ciudad(
    body: &CrearCiudadSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.clave_localidad < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La clave de la localidad no puede ser negativo"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.estado < 1 || body.estado > 32 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El estado no existe"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre de la ciudad no puede estar vacio"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.numero_habitantes < 1 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El numero de habitantes no puede ser 0"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.orden < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El orden de la ciudad no puede ser negativo"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cp.trim().is_empty() || body.cp.trim().len() != 5 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El codigo postal no puede estar vacio ni tener una longitud diferente a 5",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}
