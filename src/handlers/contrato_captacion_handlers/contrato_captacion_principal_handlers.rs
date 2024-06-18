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
        fichas_models::detalle_ficha_models::DetalleFichaTemporalModelo,
        user_models::UsuarioModelo,
    },
    schemas::contratos_captacion_schemas::{
        AbonoCargoContratoCaptacionSchema, CargoAbonoEnum, CrearContratoCaptacionSchema,
        ListarContratosCaptacionQuery, ObtenerSaldoContratosCaptacionQuery,
    },
    validators::contrato_captacion_validators::validar_nueva_contrato_captacion,
    AppState,
};

use super::contrato_captacion_auxiliares::{
    calcular_totales_captacion, formatear_contratos_captacion,
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
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )
    })?;

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
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )
    })?;

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
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )
    })?;

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

// pub async fn deposito_contrato_captacion_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<DepositoRetiroContratoCaptacionSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let nueva_ficha = sqlx::query_as!(
//         Ficha,
//         "INSERT INTO detalles_ficha_temporal
//         (persona, captacion, abono, cargo) VALUES
//         ($1, $2, $3, $4)
//         RETURNING *",
//         body.persona,
//         body.captacion,
//         body.abono,
//         0.0
//     )
//     .fetch_one(&data.db)
//     .await
//     .map_err(|e| {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(serde_json::json!({
//                 "estado": false,
//                 "mensaje": format!("Error en la base de datos: {}", e),
//             })),
//         )
//     })?;
// }
