use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers::trabajo_handlers::crear_puesto_trabajo_handler, AppState};

pub fn trabajo_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/puesto_trabajo/nuevo/",
            post(crear_puesto_trabajo_handler),
        )
        .with_state(app_state)
}
