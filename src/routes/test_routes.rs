use axum::{routing::get, Router};

use crate::handlers::test_handlers::health_checker_handler;

pub fn test_router() -> Router {
    Router::new().route("/api", get(health_checker_handler))
}
