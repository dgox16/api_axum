use crate::{
    models::{
        poliza_models::{AplicacionPoliza, FuentePoliza, PolizaModelo, TipoPoliza},
        user_models::UsuarioModelo,
    },
    schemas::poliza_schema::CrearPolizaSchema,
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
    let nueva_poliza = sqlx::query_as!(
        PolizaModelo, r#"INSERT INTO polizas (tipo, sucursal, concepto, usuario_elabora) VALUES ($1, $2, $3, $4) RETURNING id_poliza,tipo AS "tipo: TipoPoliza",numero,sucursal,fecha_poliza,fecha_registro_poliza,concepto,usuario_autoriza,usuario_elabora,aplicacion AS "aplicacion: AplicacionPoliza",fuente AS "fuente: FuentePoliza",automatico"#,
        body.tipo as TipoPoliza,
        body.sucursal,
        body.concepto.to_string(),
        usuario.id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let respuesta_error = serde_json::json!({
            "estado": "error",
            "mensaje": format!("Error en la base de datos: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(respuesta_error))
    })?;

    let respuesta = json!({
        "estado": "exitoso",
        "data": nueva_poliza
    });

    Ok(Json(respuesta))
}
