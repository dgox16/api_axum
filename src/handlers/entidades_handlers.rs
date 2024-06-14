use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    models::entidades_models::{
        BancoModelo, ClasificacionCuenta, CuentaModelo, EmpresaModelo, FinalidadCuenta,
        FrecuenciaEmpresaModelo, GrupoCuenta, NaturalezaCuenta, OperacionProveedor,
        ProveedorModelo, SucursalModelo, TipoProveedor,
    },
    schemas::entidades_schemas::{
        CrearBancoSchema, CrearCuentaSchema, CrearEmpresaSchema, CrearFrecuenciaEmpresaSchema,
        CrearProveedorSchema, CrearSucursalSchema,
    },
    validators::entidades_validators::{
        validar_nueva_cuenta, validar_nueva_empresa, validar_nueva_frecuencia_empresa,
        validar_nueva_sucursal, validar_nuevo_banco, validar_nuevo_proveedor,
    },
    AppState,
};

pub async fn crear_nueva_sucursal_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearSucursalSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_sucursal(&body)?;
    let nueva_sucursal = sqlx::query_as!(
        SucursalModelo,
        "INSERT INTO sucursales (nombre, encargado, domicilio)
        VALUES ($1,$2,$3) RETURNING *",
        body.nombre,
        body.encargado,
        body.domicilio
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nueva_sucursal
    });
    Ok(Json(respuesta))
}

pub async fn crear_nuevo_proveedor_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearProveedorSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nuevo_proveedor(&body)?;
    let tipo = body.tipo.unwrap_or(TipoProveedor::Nacional);
    let operacion = body
        .operacion
        .unwrap_or(OperacionProveedor::ServiciosProfesionales);
    let pais_residencia = body.pais_residencia.unwrap_or(1);
    let pais_nacimiento = body.pais_nacimiento.unwrap_or(1);

    let nuevo_proveedor = sqlx::query_as!(
        ProveedorModelo,
        r#"INSERT INTO proveedores
        (nombre, domicilio, rfc, curp, telefono, tipo, operacion, regimen,
        nombre_extranjero, pais_residencia, pais_nacimiento, banco, cuenta_clabe)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
        RETURNING id_proveedor, nombre, domicilio, rfc, curp, telefono,
        tipo AS "tipo: TipoProveedor", operacion AS "operacion: OperacionProveedor",
        regimen, nombre_extranjero, pais_residencia, pais_nacimiento, banco, cuenta_clabe"#,
        body.nombre,
        body.domicilio,
        body.rfc,
        body.curp,
        body.telefono,
        tipo as TipoProveedor,
        operacion as OperacionProveedor,
        body.regimen,
        body.nombre_extranjero,
        pais_residencia,
        pais_nacimiento,
        body.banco,
        body.cuenta_clabe
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_proveedor
    });
    Ok(Json(respuesta))
}

pub async fn crear_nuevo_banco_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearBancoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nuevo_banco(&body)?;
    let nuevo_banco = sqlx::query_as!(
        BancoModelo,
        "INSERT INTO bancos (nombre) VALUES ($1) RETURNING *",
        body.nombre.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_banco
    });
    Ok(Json(respuesta))
}

pub async fn crear_nueva_cuenta_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearCuentaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_cuenta(&body)?;
    let clasificacion = body.clasificacion.unwrap_or(ClasificacionCuenta::Capitulo);
    let grupo = body.grupo.unwrap_or(GrupoCuenta::Activo);
    let finalidad = body.finalidad.unwrap_or(FinalidadCuenta::Otros);
    let naturaleza = body.naturaleza.unwrap_or(NaturalezaCuenta::Deudora);
    let afectable = body.afectable.unwrap_or(false);
    let en_balance = body.en_balance.unwrap_or(false);
    let subcuenta_siti = body.subcuenta_siti.unwrap_or(false);
    let prorrateo = body.prorrateo.unwrap_or(false);

    let nueva_cuenta = sqlx::query_as!(
        CuentaModelo,
        r#"INSERT INTO cuentas
        (cuenta,cuenta_siti,nombre,clasificacion,grupo,finalidad,naturaleza,afectable
        ,padre,nivel,en_balance,en_catalogo_minimo,nombre_balance,nombre_siti,
        cuenta_padre_siti,cuenta_agrupar,orden_siti,subcuenta_siti,prorrateo)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)
        RETURNING id_cuenta, cuenta,cuenta_siti,nombre,
        clasificacion AS "clasificacion: ClasificacionCuenta",
        grupo AS "grupo: GrupoCuenta",finalidad AS "finalidad: FinalidadCuenta",
        naturaleza AS "naturaleza: NaturalezaCuenta",afectable,padre,nivel,en_balance,
        en_catalogo_minimo,nombre_balance,nombre_siti,cuenta_padre_siti,cuenta_agrupar,
        orden_siti,subcuenta_siti,prorrateo"#,
        body.cuenta,
        body.cuenta_siti,
        body.nombre,
        clasificacion as ClasificacionCuenta,
        grupo as GrupoCuenta,
        finalidad as FinalidadCuenta,
        naturaleza as NaturalezaCuenta,
        afectable,
        body.padre,
        body.nivel,
        en_balance,
        body.en_catalogo_minimo,
        body.nombre_balance,
        body.nombre_siti,
        body.cuenta_padre_siti,
        body.cuenta_agrupar,
        body.orden_siti,
        subcuenta_siti,
        prorrateo
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nueva_cuenta
    });
    Ok(Json(respuesta))
}

pub async fn crear_nueva_frecuencia_empresa_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearFrecuenciaEmpresaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_frecuencia_empresa(&body)?;
    let nueva_frecuencia_empresa = sqlx::query_as!(
        FrecuenciaEmpresaModelo,
        "INSERT INTO frecuencias_empresa (nombre, dias, meses)
        VALUES ($1,$2,$3) RETURNING id_frecuencias_empresa, nombre, dias, meses",
        body.nombre,
        body.dias,
        body.meses
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nueva_frecuencia_empresa
    });
    Ok(Json(respuesta))
}

pub async fn crear_nueva_empresa_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearEmpresaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_empresa(&body)?;
    let nueva_empresa = sqlx::query_as!(
        EmpresaModelo,
        "INSERT INTO empresas (nombre,domicilio,telefono,empleos_fijos, frecuencia)
        VALUES ($1,$2,$3,$4,$5) RETURNING *",
        body.nombre,
        body.domicilio,
        body.telefono,
        body.empleos_fijos,
        body.frecuencia
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": true,
        "datos": nueva_empresa
    });
    Ok(Json(respuesta))
}
