use axum::{http::StatusCode, Json};
use sqlx::Error;

pub fn error_base_datos(error: Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
            "estado": false,
            "mensaje": format!("Error en la base de datos: {}", error),
        })),
    )
}
