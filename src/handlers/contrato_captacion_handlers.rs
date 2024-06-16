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
        user_models::UsuarioModelo,
    },
    responses::contrato_captacion_responses::ListarContratoCaptacionRespuesta,
    schemas::contratos_captacion_schemas::{
        CrearContratoCaptacionSchema, ListarContratosCaptacionQuery,
    },
    validators::contrato_captacion_validators::validar_nueva_contrato_captacion,
    AppState,
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

async fn calcular_saldo(
    contrato: &ContratoCaptacionModelo,
    data: &Arc<AppState>,
) -> Result<f32, (StatusCode, Json<serde_json::Value>)> {
    let detalles_fichas = sqlx::query!(
        "SELECT cargo, abono FROM detalles_ficha WHERE captacion = $1",
        contrato.id_contrato_captacion
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

async fn formatear_contratos_captacion(
    contratos_encontrados: Vec<ContratoCaptacionModelo>,
    data: Arc<AppState>,
) -> Result<Vec<ListarContratoCaptacionRespuesta>, (StatusCode, Json<serde_json::Value>)> {
    let mut contratos_formateados = Vec::new();

    for contrato in contratos_encontrados {
        let saldo = match calcular_saldo(&contrato, &data).await {
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
