use crate::{
    models::poliza_models::{AplicacionPoliza, FuentePoliza, PolizaModelo, TipoPoliza},
    responses::error_responses::error_base_datos,
    schemas::poliza_schemas::ObtenerPolizaParams,
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn eliminar_poliza_handler(
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

    if poliza_encontrada.tipo == TipoPoliza::Egreso {
        sqlx::query!(
            r#"DELETE FROM polizas_egreso WHERE poliza = $1"#,
            params.id_poliza,
        )
        .execute(&data.db)
        .await
        .map_err(error_base_datos)?;
    }

    sqlx::query!(
        r#"DELETE FROM detalles_poliza WHERE poliza = $1"#,
        params.id_poliza,
    )
    .execute(&data.db)
    .await
    .map_err(error_base_datos)?;

    let resultado_poliza = sqlx::query!(
        r#"DELETE FROM polizas WHERE id_poliza = $1"#,
        params.id_poliza,
    )
    .execute(&data.db)
    .await
    .map_err(error_base_datos)?;

    let respuesta = if resultado_poliza.rows_affected() > 0 {
        json!({
            "estado": true,
            "mensaje": "Poliza eliminada correctamente"
        })
    } else {
        json!({
            "estado": false,
            "mensaje": "No se encontró una póliza con ese id"
        })
    };

    Ok(Json(respuesta))
}
