use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    models::entidades_models::{
        BancoModelo, ClasificacionCuenta, CuentaModelo, FinalidadCuenta, GrupoCuenta,
        NaturalezaCuenta, SucursalModelo,
    },
    schemas::entidades_schemas::{CrearBancoSchema, CrearCuentaSchema, CrearSucursalSchema},
    validators::entidades_validators::{validar_nueva_sucursal, validar_nuevo_banco},
    AppState,
};

pub async fn crear_nueva_sucursal_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearSucursalSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_sucursal(&body)?;
    let nueva_sucursal = sqlx::query_as!(
        SucursalModelo,
        "INSERT INTO sucursales (nombre, encargado, domicilio) VALUES ($1,$2,$3) RETURNING *",
        body.nombre,
        body.encargado,
        body.domicilio
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": "exitoso",
        "data": nueva_sucursal
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
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": "exitoso",
        "data": nuevo_banco
    });
    Ok(Json(respuesta))
}

pub async fn crear_nueva_cuenta_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CrearCuentaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
        r#"INSERT INTO cuentas (cuenta,cuenta_siti,nombre,clasificacion,grupo,finalidad,naturaleza,afectable,padre,nivel,en_balance,en_catalogo_minimo,nombre_balance,nombre_siti,cuenta_padre_siti,cuenta_agrupar,orden_siti,subcuenta_siti,prorrateo) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19) RETURNING id_cuenta, cuenta,cuenta_siti,nombre,clasificacion AS "clasificacion: ClasificacionCuenta",grupo AS "grupo: GrupoCuenta",finalidad AS "finalidad: FinalidadCuenta",naturaleza AS "naturaleza: NaturalezaCuenta",afectable,padre,nivel,en_balance,en_catalogo_minimo,nombre_balance,nombre_siti,cuenta_padre_siti,cuenta_agrupar,orden_siti,subcuenta_siti,prorrateo"#,
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
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": "exitoso",
        "data": nueva_cuenta
    });
    Ok(Json(respuesta))
}
