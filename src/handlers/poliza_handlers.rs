use crate::{
    models::{
        poliza_models::{
            AplicacionPoliza, FuentePoliza, PolizaEgresoModelo, PolizaModelo, TipoPoliza,
        },
        user_models::UsuarioModelo,
    },
    schemas::poliza_schema::{CrearPolizaSchema, PolizaEgresoSchema},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;
use std::sync::Arc;

pub async fn crear_nueva_poliza_handler(
    State(data): State<Arc<AppState>>,
    Extension(usuario): Extension<UsuarioModelo>,
    Json(body): Json<CrearPolizaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let resultado = if body.tipo == TipoPoliza::Egreso {
        insertar_poliza_con_egreso(&data, &usuario, &body).await
    } else {
        insertar_poliza_solamente(&data, &usuario, &body).await
    };

    match resultado {
        Ok((nueva_poliza, Some(nueva_poliza_egreso))) => {
            let respuesta = json!({
                "estado": "exitoso",
                "data": {
                    "poliza": nueva_poliza,
                    "poliza_egreso": nueva_poliza_egreso,
                }
            });
            Ok(Json(respuesta))
        }
        Ok((nueva_poliza, None)) => {
            let respuesta = json!({
                "estado": "exitoso",
                "data": nueva_poliza,
            });
            Ok(Json(respuesta))
        }
        Err((status, error)) => Err((status, Json(error))),
    }
}

async fn insertar_poliza_con_egreso(
    data: &Arc<AppState>,
    usuario: &UsuarioModelo,
    body: &CrearPolizaSchema,
) -> Result<(PolizaModelo, Option<PolizaEgresoModelo>), (StatusCode, serde_json::Value)> {
    let body_poliza_egreso = match &body.poliza_egreso {
        Some(poliza_egreso) => match validar_datos_poliza_egreso(&data, poliza_egreso).await {
            Ok(()) => poliza_egreso,
            Err(err) => return Err((StatusCode::BAD_REQUEST, err)),
        },
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                serde_json::json!({
                    "estado": "error",
                    "mensaje": "No se ha proporcionado una póliza de egreso",
                }),
            ));
        }
    };

    let nueva_poliza = insertar_poliza(&data, usuario.id, body).await?;
    let nueva_poliza_egreso = sqlx::query_as!(PolizaEgresoModelo,
        "INSERT INTO polizas_egreso (poliza,beneficiario, banco, cheque) VALUES ($1,$2,$3,$4) RETURNING *",
        nueva_poliza.id_poliza,
        body_poliza_egreso.beneficiario.to_string(),
        body_poliza_egreso.banco,
        body_poliza_egreso.cheque.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            serde_json::json!({
                "estado": "error",
                "mensaje": format!("Error en la base de datos: {}", e),
            }),
        )
    })?;

    Ok((nueva_poliza, Some(nueva_poliza_egreso)))
}

async fn insertar_poliza_solamente(
    data: &Arc<AppState>,
    usuario: &UsuarioModelo,
    body: &CrearPolizaSchema,
) -> Result<(PolizaModelo, Option<PolizaEgresoModelo>), (StatusCode, serde_json::Value)> {
    let nueva_poliza = insertar_poliza(&data, usuario.id, body).await?;
    Ok((nueva_poliza, None))
}

async fn validar_datos_poliza_egreso(
    data: &Arc<AppState>,
    poliza_egreso: &PolizaEgresoSchema,
) -> Result<(), serde_json::Value> {
    // Verificar que el campo de beneficiario no esté vacío
    if poliza_egreso.beneficiario.is_empty() {
        return Err(serde_json::json!({
            "estado": "error",
            "mensaje": "El campo de beneficiario no puede estar vacío",
        }));
    }

    // Verificar que el ID del banco exista en la base de datos
    let banco_existente: Option<i32> = sqlx::query_scalar!(
        "SELECT id_banco FROM bancos WHERE id_banco = $1",
        poliza_egreso.banco
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|_| {
        serde_json::json!({
            "estado": "error",
            "mensaje": "Error al verificar la existencia del banco",
        })
    })?;

    if banco_existente.is_none() {
        return Err(serde_json::json!({
            "estado": "error",
            "mensaje": "El ID del banco proporcionado no existe",
        }));
    }

    if poliza_egreso.cheque.is_empty() {
        return Err(serde_json::json!({
            "estado": "error",
            "mensaje": "El campo de cheque no puede estar vacío",
        }));
    }

    Ok(())
}
async fn insertar_poliza(
    data: &Arc<AppState>,
    usuario_id: i32,
    body: &CrearPolizaSchema,
) -> Result<PolizaModelo, (StatusCode, serde_json::Value)> {
    let tipo = body.tipo.clone();
    let poliza_result = sqlx::query_as!(
        PolizaModelo, r#"INSERT INTO polizas (tipo, sucursal, concepto, usuario_elabora) VALUES ($1, $2, $3, $4) RETURNING id_poliza,tipo AS "tipo: TipoPoliza",numero,sucursal,fecha_poliza,fecha_registro_poliza,concepto,usuario_autoriza,usuario_elabora,aplicacion AS "aplicacion: AplicacionPoliza",fuente AS "fuente: FuentePoliza",automatico"#,
        tipo as TipoPoliza,
        body.sucursal,
        body.concepto.to_string(),
        usuario_id
    )
    .fetch_one(&data.db)
    .await;

    match poliza_result {
        Ok(poliza) => Ok(poliza),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            serde_json::json!({
                "estado": "error",
                "mensaje": format!("Error en la base de datos: {}", e),
            }),
        )),
    }
}
