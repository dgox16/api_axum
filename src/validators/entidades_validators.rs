use axum::{http::StatusCode, Json};

use crate::schemas::entidades_schemas::{
    CrearBancoSchema, CrearCuentaSchema, CrearEmpresaSchema, CrearFrecuenciaEmpresaSchema,
    CrearProveedorSchema, CrearSucursalSchema,
};

pub fn validar_nuevo_banco(
    body: &CrearBancoSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre del banco no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nueva_sucursal(
    body: &CrearSucursalSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre de la sucursal no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nuevo_proveedor(
    body: &CrearProveedorSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre de la sucursal no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.rfc.trim().is_empty() || body.rfc.trim().len() < 13 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El RFC no puede estar vacío y debe tener al menos 13 caracteres",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.curp.trim().is_empty() || body.curp.trim().len() < 18 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La CURP no puede estar vacía y debe tener al menos 18 caracteres",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.telefono.trim().is_empty() || body.telefono.trim().len() < 10 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El telefono no puede estar vacío y debe tener al menos 10 caracteres",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.regimen.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El regimen no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cuenta_clabe.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La cuenta CLABE no puede estar vacia",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nueva_cuenta(
    body: &CrearCuentaSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.cuenta.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La cuenta no puede estar vacia",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cuenta_siti.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La cuenta SITI no puede estar vacia",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    if body.padre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El campo del padre no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.nivel < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nivel no puede ser menos de 0",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.nombre_balance.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre del balance no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.nombre_siti.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre SITI no puede estar vacio",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cuenta_padre_siti.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La cuenta del padre SITI no puede estar vacia",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.cuenta_agrupar.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La cuenta a agrupar no puede estar vacia",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.orden_siti < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "La order SITI no puede ser menos de 0",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nueva_empresa(
    body: &CrearEmpresaSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre de la empresa no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.telefono.trim().is_empty() || body.telefono.trim().len() != 10 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El telefono no puede estar vacio o tener una extension diferente a 10",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.empleos_fijos < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "No puede haber empleos fijos negativos"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}

pub fn validar_nueva_frecuencia_empresa(
    body: &CrearFrecuenciaEmpresaSchema,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if body.nombre.trim().is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "El nombre de la frecuencia de empresa no puede estar vacío",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.dias < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "No puede haber dias negativos"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    if body.meses < 0 {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "No puede haber dias negativos"
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }
    Ok(())
}
