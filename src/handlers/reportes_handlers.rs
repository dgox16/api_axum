use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde_json::json;

use crate::{
    models::{
        entidades_models::{
            ClasificacionCuenta, CuentaModelo, FinalidadCuenta, GrupoCuenta, NaturalezaCuenta,
        },
        poliza_models::DetallePolizaFormateadoModelo,
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
    let detalles_rango =
        buscar_detalles_polizas_rango_fechas(&data, query.fecha_inicial, query.fecha_final).await?;
    let fecha_cero = NaiveDate::from_ymd_opt(1, 1, 1).unwrap();
    let detalles_anteriores =
        buscar_detalles_polizas_rango_fechas(&data, fecha_cero, query.fecha_inicial).await?;

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

    let total: f32 = detalles_rango
        .iter()
        .map(|detalle| detalle.abono - detalle.cargo)
        .sum();

    let balanza: Vec<BalanzaComprobacionModelo> = cuentas_encontradas
        .into_iter()
        .map(|cuenta| {
            let detalles_anteriores_cuenta: Vec<&DetallePolizaFormateadoModelo> =
                detalles_anteriores
                    .iter()
                    .filter(|detalle| detalle.cuenta == cuenta.cuenta)
                    .collect();

            let detalles_rango_cuenta: Vec<&DetallePolizaFormateadoModelo> = detalles_rango
                .iter()
                .filter(|detalle| detalle.cuenta == cuenta.cuenta)
                .collect();

            let (deudora_anterior, acreedora_anterior) =
                if ["1", "5"].contains(&&cuenta.cuenta[..1]) {
                    let saldo_anterior: f32 = detalles_anteriores_cuenta
                        .iter()
                        .map(|detalle| detalle.cargo - detalle.abono)
                        .sum();
                    (saldo_anterior, 0.0)
                } else {
                    let saldo_anterior: f32 = detalles_anteriores_cuenta
                        .iter()
                        .map(|detalle| detalle.abono - detalle.cargo)
                        .sum();
                    (0.0, saldo_anterior)
                };

            // Calcular cargo y abono
            let cargo: f32 = detalles_rango_cuenta
                .iter()
                .map(|detalle| detalle.cargo)
                .sum();
            let abono: f32 = detalles_rango_cuenta
                .iter()
                .map(|detalle| detalle.abono)
                .sum();

            // Calcular saldo deudor y saldo acreedor
            let (saldo_deudor, saldo_acreedor) = if ["1", "5"].contains(&&cuenta.cuenta[..1]) {
                let saldo_final: f32 = detalles_rango_cuenta
                    .iter()
                    .map(|detalle| detalle.cargo - detalle.abono)
                    .sum();
                (saldo_final, 0.0)
            } else {
                let saldo_final: f32 = detalles_rango_cuenta
                    .iter()
                    .map(|detalle| detalle.abono - detalle.cargo)
                    .sum();
                (0.0, saldo_final)
            };

            BalanzaComprobacionModelo {
                cuenta: cuenta.cuenta,
                nombre: cuenta.nombre,
                deudora_anterior,
                acreedora_anterior,
                cargo,
                abono,
                saldo_deudor,
                saldo_acreedor,
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
