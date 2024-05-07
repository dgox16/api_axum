use axum::{response::IntoResponse, Json};

// Funcion solo para comprobar que el servidor sirve
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "La API funciona correctamente";

    let json_response = serde_json::json!({
        "estado": true,
        "mensaje": MESSAGE
    });

    Json(json_response)
}
