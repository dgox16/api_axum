use crate::{
    models::{
        poliza_models::{
            AplicacionPoliza, DetallePolizaModelo, FuentePoliza, IvaDetallePoliza,
            PolizaEgresoModelo, PolizaModelo, TipoPoliza,
        },
        user_models::UsuarioModelo,
    },
    responses::error_responses::error_base_datos,
    schemas::poliza_schemas::{CrearDetallePolizaSchema, CrearPolizaSchema, ObtenerPolizaParams},
    validators::poliza_validators::{
        validar_nueva_poliza, validar_nueva_poliza_egreso, validar_nuevo_detalle_poliza,
    },
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn crear_detalle_poliza_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<ObtenerPolizaParams>,
    Json(body): Json<CrearDetallePolizaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("{}", params.id_poliza);
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

    let iva = body.iva.unwrap_or(IvaDetallePoliza::NoAplica);
    let nuevo_detalle = sqlx::query_as!(
        DetallePolizaModelo,
        r#"INSERT INTO detalles_poliza
            (poliza,cuenta,sucursal,cargo,abono,proveedor,concepto,iva) 
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
            RETURNING id_detalle_poliza, poliza,cuenta,sucursal,cargo,
            abono,proveedor,concepto,iva AS "iva: IvaDetallePoliza""#,
        poliza_encontrada.id_poliza,
        body.cuenta,
        poliza_encontrada.sucursal,
        body.cargo,
        body.abono,
        body.proveedor,
        body.concepto,
        iva as IvaDetallePoliza
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    let respuesta = json!({
        "estado": true,
        "datos": nuevo_detalle
    });

    Ok(Json(respuesta))
}

pub async fn crear_nueva_poliza_handler(
    State(data): State<Arc<AppState>>,
    Extension(usuario): Extension<UsuarioModelo>,
    Json(body): Json<CrearPolizaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match &body.detalles_poliza {
        Some(detalles_poliza) => {
            for detalle in detalles_poliza {
                validar_nuevo_detalle_poliza(detalle)?;
            }
        }
        None => {}
    }

    let (nueva_poliza, nueva_poliza_egreso) = match body.tipo {
        TipoPoliza::Egreso => insertar_poliza_con_egreso(&data, &usuario, &body).await?,
        _ => insertar_poliza_solamente(&data, &usuario, &body).await?,
    };

    let detalles_creados = if let Some(detalles_poliza) = body.detalles_poliza {
        Some(
            crear_detalles_poliza(
                &data,
                nueva_poliza.id_poliza,
                body.sucursal,
                detalles_poliza,
            )
            .await?,
        )
    } else {
        None
    };

    let mut respuesta = json!({
        "estado": true,
        "datos": {
            "poliza": nueva_poliza,
        }
    });

    if let Some(poliza_egreso) = nueva_poliza_egreso {
        respuesta["datos"]["poliza_egreso"] = json!(poliza_egreso);
    }

    if let Some(detalles) = detalles_creados {
        respuesta["datos"]["detalles_poliza"] = json!(detalles);
    }

    Ok(Json(respuesta))
}

async fn crear_detalles_poliza(
    data: &Arc<AppState>,
    nueva_poliza_id: i32,
    sucursal: i32,
    detalles_poliza: Vec<CrearDetallePolizaSchema>,
) -> Result<Vec<DetallePolizaModelo>, (StatusCode, Json<serde_json::Value>)> {
    let mut detalles_creados = Vec::new();
    for detalle in detalles_poliza {
        let iva = detalle.iva.unwrap_or(IvaDetallePoliza::NoAplica);
        let nuevo_detalle = sqlx::query_as!(
            DetallePolizaModelo,
            r#"INSERT INTO detalles_poliza
            (poliza,cuenta,sucursal,cargo,abono,proveedor,concepto,iva) 
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
            RETURNING id_detalle_poliza, poliza,cuenta,sucursal,cargo,
            abono,proveedor,concepto,iva AS "iva: IvaDetallePoliza""#,
            nueva_poliza_id,
            detalle.cuenta,
            sucursal,
            detalle.cargo,
            detalle.abono,
            detalle.proveedor,
            detalle.concepto,
            iva as IvaDetallePoliza
        )
        .fetch_one(&data.db)
        .await
        .map_err(error_base_datos)?;
        detalles_creados.push(nuevo_detalle);
    }
    Ok(detalles_creados)
}

async fn insertar_poliza_con_egreso(
    data: &Arc<AppState>,
    usuario: &UsuarioModelo,
    body: &CrearPolizaSchema,
) -> Result<(PolizaModelo, Option<PolizaEgresoModelo>), (StatusCode, Json<serde_json::Value>)> {
    let body_poliza_egreso = match &body.poliza_egreso {
        Some(poliza_egreso) => {
            validar_nueva_poliza_egreso(data, poliza_egreso).await?;
            poliza_egreso
        }
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "estado": false,
                    "mensaje": "No se ha proporcionado una póliza de egreso",
                })),
            ));
        }
    };

    let nueva_poliza = crear_poliza(data, usuario.id, body).await?;
    let nueva_poliza_egreso = sqlx::query_as!(
        PolizaEgresoModelo,
        "INSERT INTO polizas_egreso (poliza,beneficiario, banco, cheque)
        VALUES ($1,$2,$3,$4) RETURNING *",
        nueva_poliza.id_poliza,
        body_poliza_egreso.beneficiario.to_string(),
        body_poliza_egreso.banco,
        body_poliza_egreso.cheque.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok((nueva_poliza, Some(nueva_poliza_egreso)))
}

async fn insertar_poliza_solamente(
    data: &Arc<AppState>,
    usuario: &UsuarioModelo,
    body: &CrearPolizaSchema,
) -> Result<(PolizaModelo, Option<PolizaEgresoModelo>), (StatusCode, Json<serde_json::Value>)> {
    let nueva_poliza = crear_poliza(data, usuario.id, body).await?;
    Ok((nueva_poliza, None))
}

async fn crear_poliza(
    data: &Arc<AppState>,
    usuario_id: i32,
    body: &CrearPolizaSchema,
) -> Result<PolizaModelo, (StatusCode, Json<serde_json::Value>)> {
    validar_nueva_poliza(body)?;
    let numero = body.numero.unwrap_or(1);
    let aplicacion = body.aplicacion.clone().unwrap_or(AplicacionPoliza::Normal);
    let fuente = body.fuente.clone().unwrap_or(FuentePoliza::Operacion);
    let poliza_resultado = sqlx::query_as!(
        PolizaModelo,
        r#"INSERT INTO polizas 
        (tipo, numero, sucursal, concepto, usuario_autoriza, usuario_elabora, aplicacion,fuente)
        VALUES ($1, $2, $3, $4, $5, $6,$7,$8)
        RETURNING id_poliza,tipo AS "tipo: TipoPoliza",numero,sucursal,
        fecha_poliza,fecha_registro_poliza,concepto,
        usuario_autoriza,usuario_elabora,aplicacion AS "aplicacion: AplicacionPoliza",
        fuente AS "fuente: FuentePoliza",automatico"#,
        body.tipo.clone() as TipoPoliza,
        numero,
        body.sucursal,
        body.concepto.to_string(),
        usuario_id,
        usuario_id,
        aplicacion as AplicacionPoliza,
        fuente as FuentePoliza
    )
    .fetch_one(&data.db)
    .await;

    match poliza_resultado {
        Ok(poliza) => Ok(poliza),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": format!("Error en la base de datos: {}", e),
            })),
        )),
    }
}
