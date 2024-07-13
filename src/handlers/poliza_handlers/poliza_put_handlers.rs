use crate::{
    models::{
        poliza_models::{
            AplicacionPoliza, DetallePolizaModelo, FuentePoliza, IvaDetallePoliza,
            PolizaEgresoModelo, PolizaModelo, TipoPoliza,
        },
        user_models::UsuarioModelo,
    },
    responses::error_responses::error_base_datos,
    schemas::poliza_schemas::{
        CrearDetallePolizaSchema, CrearPolizaSchema, EditarPolizaSchema, ObtenerPolizaParams,
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

async fn actualizar_poliza(
    data: &Arc<AppState>,
    id_usuario: i32,
    id_poliza: i32,
    body: &EditarPolizaSchema,
) -> Result<PolizaModelo, (StatusCode, Json<serde_json::Value>)> {
    let poliza_actualizada = sqlx::query_as!(
        PolizaModelo,
        r#"
        UPDATE polizas
        SET tipo = $1, numero = $2, sucursal = $3, concepto = $4, 
            usuario_autoriza = $5, usuario_elabora = $6, aplicacion = $7, fuente = $8
        WHERE id_poliza = $9
        RETURNING id_poliza, tipo AS "tipo: TipoPoliza", numero, sucursal,
                  fecha_poliza, fecha_registro_poliza, concepto,
                  usuario_autoriza, usuario_elabora, aplicacion AS "aplicacion: AplicacionPoliza",
                  fuente AS "fuente: FuentePoliza", automatico
        "#,
        body.tipo.clone() as TipoPoliza,
        body.numero,
        body.sucursal,
        body.concepto,
        id_usuario,
        id_usuario,
        body.aplicacion.clone() as AplicacionPoliza,
        body.fuente.clone() as FuentePoliza,
        id_poliza
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(poliza_actualizada)
}

async fn actualizar_poliza_egreso(
    data: &Arc<AppState>,
    id_poliza: i32,
    body: EditarPolizaSchema,
) -> Result<PolizaEgresoModelo, (StatusCode, Json<serde_json::Value>)> {
    let body_poliza_egreso = match body.poliza_egreso {
        Some(poliza_egreso) => poliza_egreso,
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
    let nueva_poliza_egreso = sqlx::query_as!(
        PolizaEgresoModelo,
        "UPDATE polizas_egreso
            SET beneficiario = $1, banco = $2, cheque = $3 
            WHERE poliza = $4
            RETURNING *",
        body_poliza_egreso.beneficiario,
        body_poliza_egreso.banco,
        body_poliza_egreso.cheque,
        id_poliza
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(nueva_poliza_egreso)
}

async fn eliminar_poliza_egreso(
    data: &Arc<AppState>,
    id_poliza: i32,
) -> Result<bool, (StatusCode, Json<serde_json::Value>)> {
    let resultado = sqlx::query!("DELETE FROM polizas_egreso WHERE poliza=$1", id_poliza)
        .execute(&data.db)
        .await
        .map_err(error_base_datos)?;

    if resultado.rows_affected() == 0 {
        Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "estado": false,
                "mensaje": "No se ha proporcionado una póliza de egreso",
            })),
        ))
    } else {
        Ok(true)
    }
}

async fn crear_poliza_egreso(
    data: &Arc<AppState>,
    id_poliza: i32,
    body: EditarPolizaSchema,
) -> Result<PolizaEgresoModelo, (StatusCode, Json<serde_json::Value>)> {
    let body_poliza_egreso = match body.poliza_egreso {
        Some(poliza_egreso) => poliza_egreso,
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
    let nueva_poliza_egreso = sqlx::query_as!(
        PolizaEgresoModelo,
        "INSERT INTO polizas_egreso (poliza,beneficiario, banco, cheque)
            VALUES ($1,$2,$3,$4) RETURNING *",
        id_poliza,
        body_poliza_egreso.beneficiario.to_string(),
        body_poliza_egreso.banco,
        body_poliza_egreso.cheque.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_base_datos)?;

    Ok(nueva_poliza_egreso)
}

pub async fn editar_poliza_handler(
    State(data): State<Arc<AppState>>,
    Extension(usuario): Extension<UsuarioModelo>,
    Path(params): Path<ObtenerPolizaParams>,
    Json(body): Json<EditarPolizaSchema>,
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

    let poliza_actualizada = actualizar_poliza(&data, usuario.id, params.id_poliza, &body).await?;

    if (poliza_encontrada.tipo == TipoPoliza::Egreso) & (body.tipo == TipoPoliza::Egreso) {
        let poliza_egreso_actualizada =
            actualizar_poliza_egreso(&data, params.id_poliza, body).await?;

        let respuesta = json!({
            "estado": true,
            "datos": {
                "poliza": poliza_actualizada,
                "poliza_egreso": poliza_egreso_actualizada
            }
        });

        Ok(Json(respuesta))
    } else if (poliza_encontrada.tipo == TipoPoliza::Egreso) & (body.tipo != TipoPoliza::Egreso) {
        eliminar_poliza_egreso(&data, params.id_poliza).await?;

        let respuesta = json!({
            "estado": true,
            "datos": {
                "poliza": poliza_actualizada,
            }
        });

        Ok(Json(respuesta))
    } else if (poliza_encontrada.tipo != TipoPoliza::Egreso) & (body.tipo == TipoPoliza::Egreso) {
        let nueva_poliza_egreso = crear_poliza_egreso(&data, params.id_poliza, body).await?;

        let respuesta = json!({
            "estado": true,
            "datos": {
                "poliza": poliza_actualizada,
                "poliza_egreso": nueva_poliza_egreso,
            }
        });

        Ok(Json(respuesta))
    } else {
        let respuesta = json!({
            "estado": true,
            "datos": {
                "poliza": poliza_actualizada,
            }
        });

        Ok(Json(respuesta))
    }
}
