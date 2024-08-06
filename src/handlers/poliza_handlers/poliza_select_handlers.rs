use crate::{
    models::{
        entidades_models::{
            BalanzaComprobacion, ClasificacionCuenta, CuentaModelo, FinalidadCuenta, GrupoCuenta,
            NaturalezaCuenta,
        },
        poliza_models::{
            AplicacionPoliza, DetallePolizaFormateadoModelo, DetallePolizaModelo, FuentePoliza,
            IvaDetallePoliza, PolizaEgresoModelo, PolizaModelo, TipoPoliza,
        },
    },
    responses::error_responses::error_base_datos,
    schemas::poliza_schemas::{
        BuscarPolizaQuery, ObtenerBalanzaComprobacionQuery, ObtenerPolizaParams,
    },
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDate, TimeZone, Utc};
use serde_json::json;
use std::sync::Arc;

pub async fn obtener_poliza_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPolizaParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let poliza_encontrada = sqlx::query_as!(
        PolizaModelo,
        r#"SELECT id_poliza,tipo AS "tipo: TipoPoliza",numero,
        sucursal,fecha_poliza,fecha_registro_poliza,concepto,
        usuario_autoriza,usuario_elabora,aplicacion AS "aplicacion: AplicacionPoliza",
        fuente AS "fuente: FuentePoliza",automatico
        FROM polizas WHERE id_poliza = $1"#,
        params.id_poliza
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    let detalles_encontrados = sqlx::query_as!(
        DetallePolizaModelo,
        r#"SELECT id_detalle_poliza, poliza,cuenta,sucursal,
                    cargo,abono,proveedor,concepto,iva AS "iva: IvaDetallePoliza"
                    FROM detalles_poliza WHERE poliza = $1"#,
        poliza_encontrada.id_poliza
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    let mut respuesta = json!({
        "estado": true,
        "datos": {
            "poliza": poliza_encontrada,
        }
    });

    if poliza_encontrada.tipo == TipoPoliza::Egreso {
        let poliza_egreso_encontrada = sqlx::query_as!(
            PolizaEgresoModelo,
            r#"SELECT id_poliza_egreso, poliza, beneficiario, banco, cheque
            FROM polizas_egreso WHERE poliza = $1"#,
            poliza_encontrada.id_poliza
        )
        .fetch_one(&data.db)
        .await
        .map_err(error_base_datos)?;

        respuesta["datos"]["poliza_egreso"] = json!(poliza_egreso_encontrada);
    }

    if !detalles_encontrados.is_empty() {
        respuesta["datos"]["detalles_poliza"] = json!(detalles_encontrados);
    }

    Ok(Json(respuesta))
}

pub async fn buscar_polizas_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<BuscarPolizaQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let limite = query.limite.unwrap_or(20);
    let concepto = query.concepto.unwrap_or(String::from("%"));

    let polizas_encontradas = sqlx::query_as!(
        PolizaModelo,
        r#"SELECT id_poliza, tipo AS "tipo: TipoPoliza", numero, sucursal, fecha_poliza,
        fecha_registro_poliza, concepto, usuario_autoriza, usuario_elabora, 
        aplicacion AS "aplicacion: AplicacionPoliza",fuente AS "fuente: FuentePoliza", automatico
        FROM polizas WHERE concepto ILIKE '%' || $1 || '%' LIMIT $2"#,
        concepto,
        limite
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    let respuesta = json!({
        "estado": true,
        "datos": polizas_encontradas
    });
    Ok(Json(respuesta))
}

async fn buscar_detalles_polizas_rango_fechas(
    data: &Arc<AppState>,
    fecha: NaiveDate,
) -> Result<Vec<DetallePolizaFormateadoModelo>, (StatusCode, Json<serde_json::Value>)> {
    let fecha_formateada = match fecha.and_hms_opt(23, 59, 59) {
        Some(f) => f,
        None => {
            let respuesta_error = serde_json::json!({
                "estado": false,
                "mensaje": "La fecha tiene un formato incorrecto",
            });
            return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
        }
    };
    let fecha_utc = Utc.from_utc_datetime(&fecha_formateada);
    let detalles_encontrados = sqlx::query_as!(
        DetallePolizaFormateadoModelo,
        r#"
        SELECT dp.id_detalle_poliza, dp.poliza, c.cuenta AS cuenta, dp.sucursal,
               dp.cargo, dp.abono, dp.proveedor, dp.concepto, dp.iva AS "iva: IvaDetallePoliza"
        FROM detalles_poliza dp 
        INNER JOIN polizas p ON dp.poliza = p.id_poliza
        INNER JOIN cuentas c ON dp.cuenta = c.id_cuenta
        WHERE p.fecha_poliza AT TIME ZONE 'America/Mexico_City' AT TIME ZONE 'UTC' <= $1
        ORDER BY c.cuenta
        "#,
        fecha_utc
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(detalles_encontrados)
}

async fn buscar_detalles_polizas_dia_especifico(
    data: &Arc<AppState>,
    fecha: NaiveDate,
) -> Result<Vec<DetallePolizaFormateadoModelo>, (StatusCode, Json<serde_json::Value>)> {
    let fecha_inicial = match fecha.and_hms_opt(0, 1, 1) {
        Some(f) => f,
        None => {
            let respuesta_error = serde_json::json!({
                "estado": false,
                "mensaje": "La fecha tiene un formato incorrecto",
            });
            return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
        }
    };
    let fecha_utc_inicial = Utc.from_utc_datetime(&fecha_inicial);

    let fecha_final = match fecha.and_hms_opt(23, 59, 59) {
        Some(f) => f,
        None => {
            let respuesta_error = serde_json::json!({
                "estado": false,
                "mensaje": "La fecha tiene un formato incorrecto",
            });
            return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
        }
    };
    let fecha_utc_final = Utc.from_utc_datetime(&fecha_final);

    let detalles_encontrados = sqlx::query_as!(
        DetallePolizaFormateadoModelo,
        r#"
        SELECT dp.id_detalle_poliza, dp.poliza, c.cuenta AS cuenta, dp.sucursal,
               dp.cargo, dp.abono, dp.proveedor, dp.concepto, dp.iva AS "iva: IvaDetallePoliza"
        FROM detalles_poliza dp
        INNER JOIN polizas p ON dp.poliza = p.id_poliza
        INNER JOIN cuentas c ON dp.cuenta = c.id_cuenta
        WHERE (p.fecha_poliza AT TIME ZONE 'America/Mexico_City' AT TIME ZONE 'UTC' >= $1
               AND p.fecha_poliza AT TIME ZONE 'America/Mexico_City' AT TIME ZONE 'UTC' < $2)
        ORDER BY c.cuenta
        "#,
        fecha_utc_inicial,
        fecha_utc_final
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(detalles_encontrados)
}

pub async fn buscar_detalles_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<ObtenerBalanzaComprobacionQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let detalles = match query.dia_especifico {
        Some(dia_especifico) => {
            if dia_especifico {
                buscar_detalles_polizas_dia_especifico(&data, query.fecha).await?
            } else {
                buscar_detalles_polizas_rango_fechas(&data, query.fecha).await?
            }
        }
        None => buscar_detalles_polizas_rango_fechas(&data, query.fecha).await?,
    };

    let respuesta = json!({
        "estado" : true,
        "datos": detalles,
    });
    Ok(Json(respuesta))
}

pub async fn obtener_balanza_comprobacion_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<ObtenerBalanzaComprobacionQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let detalles = match query.dia_especifico {
        Some(dia_especifico) => {
            if dia_especifico {
                buscar_detalles_polizas_dia_especifico(&data, query.fecha).await?
            } else {
                buscar_detalles_polizas_rango_fechas(&data, query.fecha).await?
            }
        }
        None => buscar_detalles_polizas_rango_fechas(&data, query.fecha).await?,
    };
    let cuentas_encontradas = sqlx::query_as!(
        CuentaModelo,
        r#"SELECT id_cuenta, cuenta,cuenta_siti,nombre,
        clasificacion AS "clasificacion: ClasificacionCuenta",
        grupo AS "grupo: GrupoCuenta",finalidad AS "finalidad: FinalidadCuenta",
        naturaleza AS "naturaleza: NaturalezaCuenta",afectable,padre,nivel,en_balance,
        en_catalogo_minimo,nombre_balance,nombre_siti,cuenta_padre_siti,cuenta_agrupar,
        orden_siti,subcuenta_siti,prorrateo FROM cuentas"#,
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    let total: f32 = detalles
        .iter()
        .map(|detalle| detalle.abono - detalle.cargo)
        .sum();

    let balanza: Vec<BalanzaComprobacion> = cuentas_encontradas
        .into_iter()
        .map(|cuenta| {
            let (total_cargo, total_abono) = detalles
                .iter()
                .filter(|detalle| detalle.cuenta == cuenta.cuenta)
                .fold((0.0, 0.0), |(acum_cargo, acum_abono), detalle| {
                    (acum_cargo + detalle.cargo, acum_abono + detalle.abono)
                });

            BalanzaComprobacion {
                cuenta: cuenta.cuenta,
                nombre: cuenta.nombre,
                total_cargo,
                total_abono,
                total: total_abono - total_cargo,
            }
        })
        .collect();

    let respuesta = json!({
        "estado" : true,
        "datos":{
            "balanza": balanza,
            "total": total,
        }
    });
    Ok(Json(respuesta))
}
