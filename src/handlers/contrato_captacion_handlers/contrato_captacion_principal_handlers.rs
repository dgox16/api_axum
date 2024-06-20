use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

use crate::{
    models::{
        contrato_captacion_models::{ContratoCaptacionModelo, TipoContratoCaptacion},
        fichas_models::{
            detalle_ficha_models::{DetalleFichaModelo, DetalleFichaTemporalModelo},
            ficha_principal_models::FichaModelo,
        },
        user_models::UsuarioModelo,
    },
    responses::error_responses::error_base_datos,
    schemas::contratos_captacion_schemas::{
        AbonoCargoContratoCaptacionSchema, CargoAbonoEnum, CrearContratoCaptacionSchema,
        DepositoRetiroContratoCaptacionSchema, ListarContratosCaptacionQuery,
        ObtenerSaldoContratosCaptacionQuery, TipoPagoFicha, TipoSaldoContratoCaptacion,
    },
    validators::contrato_captacion_validators::validar_nueva_contrato_captacion,
    AppState,
};

use super::contrato_captacion_auxiliares::{
    calcular_totales_captacion, eliminar_temporales_captacion, formatear_contratos_captacion,
    obtener_temporales_captacion,
};

pub async fn crear_contrato_captacion_handler(
    State(data): State<Arc<AppState>>,
    Extension(usuario): Extension<UsuarioModelo>,
    Json(body): Json<CrearContratoCaptacionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_contrato_captacion(&data, &body).await?;
    let nuevo_contrato_captacion = sqlx::query_as!(
        ContratoCaptacionModelo,
        r#"INSERT INTO contratos_captacion 
        (id_persona,cuenta,servicio,fecha,no_dejar_retirar_antes_de,fecha_contrato,
        monto_autorizado,numero_sesion,tipo_sesion,nombre,tipo,contrato_migrado,
        fecha_desbloqueo,usuario_desbloqueo,fecha_libera_garantia,
        usuario_libera_garantia,monto_libera_garantia,tercero_autorizado,tasa_pactada,
        bloqueada, monto_bloqueado_adicional,usuario,fecha_interes,
        autoriza_cancelacion, usuario_autoriza_cancelacion) VALUES 
        ($1, $2, $3, $4, $5, $6,$7,$8,$9,$10,$11,$12,$13,
        $14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25) 
        RETURNING id_contrato_captacion,id_persona,cuenta,servicio,fecha,
        no_dejar_retirar_antes_de,fecha_contrato,monto_autorizado,numero_sesion,
        tipo_sesion,nombre,tipo AS "tipo:TipoContratoCaptacion",
        contrato_migrado,fecha_desbloqueo,usuario_desbloqueo,fecha_libera_garantia,
        usuario_libera_garantia,monto_libera_garantia,tercero_autorizado,tasa_pactada,
        bloqueada, monto_bloqueado_adicional,usuario,fecha_interes,
        autoriza_cancelacion, usuario_autoriza_cancelacion"#,
        body.id_persona,
        body.cuenta,
        body.servicio,
        body.fecha,
        body.no_dejar_retirar_antes_de,
        body.fecha_contrato,
        body.monto_autorizado,
        body.numero_sesion,
        body.tipo_sesion,
        body.nombre,
        body.tipo as TipoContratoCaptacion,
        body.contrato_migrado,
        body.fecha_desbloqueo,
        body.usuario_desbloqueo,
        body.fecha_libera_garantia,
        body.usuario_libera_garantia,
        body.monto_libera_garantia,
        body.tercero_autorizado,
        body.tasa_pactada,
        body.bloqueada,
        body.monto_bloqueado_adicional,
        usuario.id,
        body.fecha_interes,
        body.autoriza_cancelacion,
        body.usuario_autoriza_cancelacion
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_contrato_captacion
    });

    Ok(Json(respuesta))
}

pub async fn listar_contratos_captacion_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<ListarContratosCaptacionQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let contratos_encontrados = sqlx::query_as!(
        ContratoCaptacionModelo,
        r#"SELECT id_contrato_captacion,id_persona,cuenta,servicio,fecha,
        no_dejar_retirar_antes_de,fecha_contrato,monto_autorizado,numero_sesion,
        tipo_sesion,nombre,tipo AS "tipo:TipoContratoCaptacion",
        contrato_migrado,fecha_desbloqueo,usuario_desbloqueo,fecha_libera_garantia,
        usuario_libera_garantia,monto_libera_garantia,tercero_autorizado,tasa_pactada,
        bloqueada, monto_bloqueado_adicional,usuario,fecha_interes,
        autoriza_cancelacion, usuario_autoriza_cancelacion
        FROM contratos_captacion 
        WHERE id_persona=$1"#,
        query.persona
    )
    .fetch_all(&data.db)
    .await
    .map_err(error_base_datos)?;

    let contratos_formateados = formatear_contratos_captacion(contratos_encontrados, data).await?;

    let respuesta = json!({
        "estado": true,
        "datos": contratos_formateados
    });

    Ok(Json(respuesta))
}

pub async fn abono_cargo_contrato_captacion_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AbonoCargoContratoCaptacionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let abono = match body.abono_cargo {
        CargoAbonoEnum::Cargo => 0.0,
        CargoAbonoEnum::Abono => match body.abono {
            Some(valor) => valor,
            None => {
                let respuesta_error = json!({
                    "estado": false,
                    "mensaje": "El valor del abono no puede estar vacío cuando se selecciona 'Abono'",
                });
                return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
            }
        },
    };

    let cargo = match body.abono_cargo {
        CargoAbonoEnum::Abono => 0.0,
        CargoAbonoEnum::Cargo => match body.cargo {
            Some(valor) => valor,
            None => {
                let respuesta_error = json!({
                    "estado": false,
                    "mensaje": "El valor del cargo no puede estar vacío cuando se selecciona 'Abono'",
                });
                return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
            }
        },
    };

    let nuevo_detalle_temporal = sqlx::query_as!(
        DetalleFichaTemporalModelo,
        "INSERT INTO detalles_ficha_temporal 
        (persona, captacion, abono, cargo) VALUES 
        ($1, $2, $3, $4) 
        RETURNING *",
        body.persona,
        body.captacion,
        abono,
        cargo
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_detalle_temporal
    });

    Ok(Json(respuesta))
}

pub async fn obtener_saldo_contrato_captacion_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<ObtenerSaldoContratosCaptacionQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let saldo = calcular_totales_captacion(data, query.persona, query.tipo).await?;

    let respuesta = json!({
        "estado": true,
        "datos": saldo
    });

    Ok(Json(respuesta))
}

pub async fn obtener_temporales_contrato_captacion_handler(
    State(data): State<Arc<AppState>>,
    Query(query): Query<ObtenerSaldoContratosCaptacionQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let temporales = obtener_temporales_captacion(data, query.persona, &query.tipo).await?;

    let respuesta = json!({
        "estado": true,
        "datos": temporales
    });

    Ok(Json(respuesta))
}

pub async fn deposito_contrato_captacion_handler(
    State(data): State<Arc<AppState>>,
    Extension(usuario): Extension<UsuarioModelo>,
    Json(body): Json<DepositoRetiroContratoCaptacionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let abonos_temporales = obtener_temporales_captacion(
        data.clone(),
        body.persona,
        &TipoSaldoContratoCaptacion::Abonos,
    )
    .await?;

    if abonos_temporales.is_empty() {
        let respuesta_error = serde_json::json!({
            "estado": false,
            "mensaje": "No hay abonos temporales",
        });
        return Err((StatusCode::BAD_REQUEST, Json(respuesta_error)));
    }

    let mut efectivo = 0.0;
    let mut cheques = 0.0;
    let mut transferencia = 0.0;
    let mut tarjeta = 0.0;

    let total_abonos = calcular_totales_captacion(
        data.clone(),
        body.persona,
        TipoSaldoContratoCaptacion::Abonos,
    )
    .await?;

    match body.tipo_pago {
        Some(TipoPagoFicha::Efectivo) => efectivo = total_abonos,
        Some(TipoPagoFicha::Cheques) => cheques = total_abonos,
        Some(TipoPagoFicha::Transferencia) => transferencia = total_abonos,
        Some(TipoPagoFicha::Tarjeta) => tarjeta = total_abonos,
        None => efectivo = total_abonos,
    }

    let referencia = body
        .referencia
        .unwrap_or("Referencia por defecto".to_owned());

    let instrumento = body.instrumento.unwrap_or("--".to_owned());

    let nueva_ficha = sqlx::query_as!(
        FichaModelo,
        "INSERT INTO fichas
        (folio, persona, usuario, sucursal, poliza, operacion_fuente, efectivo,
        cheques, transferencia, tarjeta, cancelada, referencia, factura,
        pagada, instrumento) VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *",
        body.folio_ficha,
        body.persona,
        usuario.id,
        body.sucursal,
        body.poliza,
        body.operacion_fuente,
        efectivo,
        cheques,
        transferencia,
        tarjeta,
        false,
        referencia,
        body.factura,
        false,
        instrumento
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    let mut detalles_creados = Vec::new();

    let observacion = body.observacion.unwrap_or("No hay observacion".to_owned());

    for abono_temporal in abonos_temporales {
        let nuevo_detalle_ficha = sqlx::query_as!(
            DetalleFichaModelo,
            "INSERT INTO detalles_ficha 
            (ficha, captacion, cargo, abono, operacion_fuente, 
            subficha, observacion, procesado) VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            nueva_ficha.id_ficha,
            abono_temporal.captacion,
            abono_temporal.cargo,
            abono_temporal.abono,
            body.operacion_fuente,
            body.subficha,
            observacion,
            body.procesado
        )
        .fetch_one(&data.db)
        .await
        .map_err(error_base_datos)?;

        detalles_creados.push(nuevo_detalle_ficha)
    }

    let temporales_eliminados = eliminar_temporales_captacion(
        data.clone(),
        body.persona,
        &TipoSaldoContratoCaptacion::Abonos,
    )
    .await?;

    let respuesta = json!({
        "estado": true,
        "datos": {
            "ficha": nueva_ficha,
            "detalles_ficha": detalles_creados,
            "temporales_eliminados": temporales_eliminados
        }
    });
    Ok(Json(respuesta))
}
