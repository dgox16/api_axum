use crate::{
    models::poliza_models::{
        AplicacionPoliza, DetallePolizaModelo, FuentePoliza, IvaDetallePoliza, PolizaEgresoModelo,
        PolizaModelo, TipoPoliza,
    },
    responses::error_responses::error_base_datos,
    schemas::poliza_schemas::{BuscarPolizaQuery, ObtenerPolizaParams},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
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

pub async fn buscar_polizas_concepto_handler(
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
