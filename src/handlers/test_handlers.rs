use axum::{response::IntoResponse, Json};

// Funcion solo para comprobar que el servidor sirve
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "The API works perfectly";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
