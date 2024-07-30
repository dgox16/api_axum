use crate::{
    models::poliza_models::{
        AplicacionPoliza, DetallePolizaModelo, FuentePoliza, IvaDetallePoliza, PolizaEgresoModelo,
        PolizaModelo, TipoPoliza,
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
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
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

async fn buscar_detalles_polizas(
    data: &Arc<AppState>,
    fecha: DateTime<Utc>,
) -> Result<Vec<DetallePolizaModelo>, (StatusCode, Json<serde_json::Value>)> {
    let detalles_encontrados = sqlx::query_as!(
        DetallePolizaModelo,
        r#"
        SELECT dp.id_detalle_poliza, dp.poliza, dp.cuenta, dp.sucursal,
               dp.cargo, dp.abono, dp.proveedor, dp.concepto, dp.iva AS "iva: IvaDetallePoliza"
        FROM detalles_poliza dp
        INNER JOIN polizas p ON dp.poliza = p.id_poliza
        WHERE p.fecha_poliza <= $1
        "#,
        fecha
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(detalles_encontrados)
}

pub async fn obtener_balanza_comprobacion_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<ObtenerBalanzaComprobacionQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let fecha_formateada = query.fecha.and_hms_opt(23, 59, 59);
    match fecha_formateada {
        Some(f) => {
            let f_utc = Utc.from_utc_datetime(&f);
            let detalles = buscar_detalles_polizas(&data, f_utc).await?;
            let total: f32 = detalles
                .iter()
                .map(|detalle| detalle.abono - detalle.cargo)
                .sum();

            let respuesta = json!({
                "estado" : true,
                "datos": {
                    "detalles": detalles,
                    "total": total
                }
            });
            Ok(Json(respuesta))
        }
        None => {
            let respuesta_error = serde_json::json!({
                "estado": false,
                "mensaje": "Fecha mal formateada",
            });
            Err((StatusCode::BAD_REQUEST, Json(respuesta_error)))
        }
    }
}
