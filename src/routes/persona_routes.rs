use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers::persona_handlers::crear_nueva_persona_handler, AppState};

pub fn persona_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/persona/nueva", post(crear_nueva_persona_handler))
        .with_state(app_state)
}
