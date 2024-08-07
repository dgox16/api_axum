use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    models::{
        entidades_models::{
            ClasificacionCuenta, CuentaModelo, FinalidadCuenta, GrupoCuenta, NaturalezaCuenta,
        },
        reportes_models::BalanzaComprobacionModelo,
    },
    responses::error_responses::error_base_datos,
    schemas::reportes_schemas::ObtenerBalanzaComprobacionQuery,
    AppState,
};

use super::poliza_handlers::poliza_select_handlers::buscar_detalles_polizas_rango_fechas;

pub async fn obtener_balanza_comprobacion_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<ObtenerBalanzaComprobacionQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let detalles =
        buscar_detalles_polizas_rango_fechas(&data, query.fecha_inicial, query.fecha_final).await?;

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

    let balanza: Vec<BalanzaComprobacionModelo> = cuentas_encontradas
        .into_iter()
        .map(|cuenta| {
            let (total_cargo, total_abono) = detalles
                .iter()
                .filter(|detalle| detalle.cuenta == cuenta.cuenta)
                .fold((0.0, 0.0), |(acum_cargo, acum_abono), detalle| {
                    (acum_cargo + detalle.cargo, acum_abono + detalle.abono)
                });

            BalanzaComprobacionModelo {
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
