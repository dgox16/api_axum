use std::sync::Arc;

use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::{
    models::{
        contrato_captacion_models::ContratoCaptacionModelo,
        fichas_models::detalle_ficha_models::DetalleFichaTemporalModelo,
    },
    responses::contrato_captacion_responses::ListarContratoCaptacionRespuesta,
    schemas::contratos_captacion_schemas::TipoSaldoContratoCaptacion,
    AppState,
};

async fn calcular_saldo_contrato_captacion(
    contrato: i32,
    data: &Arc<AppState>,
) -> Result<f32, (StatusCode, Json<serde_json::Value>)> {
    let detalles_fichas = sqlx::query!(
        "SELECT cargo, abono FROM detalles_ficha WHERE captacion = $1",
        contrato
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )
    })?;

    let saldo: f32 = detalles_fichas
        .iter()
        .map(|detalle| {
            let cargo = detalle.cargo;
            let abono = detalle.abono;
            abono - cargo
        })
        .sum();

    Ok((saldo * 1000.0).round() / 1000.0)
}

pub async fn formatear_contratos_captacion(
    contratos_encontrados: Vec<ContratoCaptacionModelo>,
    data: Arc<AppState>,
) -> Result<Vec<ListarContratoCaptacionRespuesta>, (StatusCode, Json<serde_json::Value>)> {
    let mut contratos_formateados = Vec::new();

    for contrato in contratos_encontrados {
        let saldo =
            match calcular_saldo_contrato_captacion(contrato.id_contrato_captacion, &data).await {
                Ok(saldo) => saldo,
                Err((status, response)) => {
                    return Err((status, response));
                }
            };

        let contrato_respuesta = ListarContratoCaptacionRespuesta {
            id_contrato_captacion: contrato.id_contrato_captacion,
            id_persona: contrato.id_persona,
            cuenta: contrato.cuenta,
            servicio: contrato.servicio,
            fecha: contrato.fecha,
            no_dejar_retirar_antes_de: contrato.no_dejar_retirar_antes_de,
            fecha_contrato: contrato.fecha_contrato,
            monto_autorizado: contrato.monto_autorizado,
            numero_sesion: contrato.numero_sesion,
            tipo_sesion: contrato.tipo_sesion,
            nombre: contrato.nombre,
            tipo: contrato.tipo,
            contrato_migrado: contrato.contrato_migrado,
            fecha_desbloqueo: contrato.fecha_desbloqueo,
            usuario_desbloqueo: contrato.usuario_desbloqueo,
            fecha_libera_garantia: contrato.fecha_libera_garantia,
            usuario_libera_garantia: contrato.usuario_libera_garantia,
            monto_libera_garantia: contrato.monto_libera_garantia,
            tercero_autorizado: contrato.tercero_autorizado,
            tasa_pactada: contrato.tasa_pactada,
            bloqueada: contrato.bloqueada,
            monto_bloqueado_adicional: contrato.monto_bloqueado_adicional,
            usuario: contrato.usuario,
            fecha_interes: contrato.fecha_interes,
            autoriza_cancelacion: contrato.autoriza_cancelacion,
            usuario_autoriza_cancelacion: contrato.usuario_autoriza_cancelacion,
            saldo,
        };

        contratos_formateados.push(contrato_respuesta);
    }

    Ok(contratos_formateados)
}

pub async fn obtener_temporales_captacion(
    data: Arc<AppState>,
    persona: i32,
    tipo: &TipoSaldoContratoCaptacion,
) -> Result<Vec<DetalleFichaTemporalModelo>, (StatusCode, Json<serde_json::Value>)> {
    let peticion_detalles_temporales = match tipo {
        TipoSaldoContratoCaptacion::Abonos => {
            sqlx::query_as!(
                DetalleFichaTemporalModelo,
                "SELECT * FROM detalles_ficha_temporal
                WHERE persona=$1 AND abono > 0 AND cargo = 0",
                persona
            )
            .fetch_all(&data.db)
            .await
        }
        TipoSaldoContratoCaptacion::Cargos => {
            sqlx::query_as!(
                DetalleFichaTemporalModelo,
                "SELECT * FROM detalles_ficha_temporal
                WHERE persona=$1 AND cargo > 0 AND abono = 0",
                persona
            )
            .fetch_all(&data.db)
            .await
        }
        TipoSaldoContratoCaptacion::Todos => {
            sqlx::query_as!(
                DetalleFichaTemporalModelo,
                "SELECT * FROM detalles_ficha_temporal
                WHERE persona=$1",
                persona
            )
            .fetch_all(&data.db)
            .await
        }
    };

    peticion_detalles_temporales.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )
    })
}

pub async fn calcular_totales_captacion(
    data: Arc<AppState>,
    persona: i32,
    tipo: TipoSaldoContratoCaptacion,
) -> Result<f32, (StatusCode, Json<serde_json::Value>)> {
    let detalles_temporales = obtener_temporales_captacion(data, persona, &tipo).await?;
    let total: f32 = detalles_temporales
        .iter()
        .map(|detalle| match tipo {
            TipoSaldoContratoCaptacion::Abonos => detalle.abono,
            TipoSaldoContratoCaptacion::Cargos => detalle.cargo,
            TipoSaldoContratoCaptacion::Todos => detalle.abono - detalle.cargo,
        })
        .sum();

    Ok((total * 1000.0).round() / 1000.0)
}
