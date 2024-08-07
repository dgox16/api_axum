use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handlers::reportes_handlers::obtener_balanza_comprobacion_handler, AppState};

pub fn reportes_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/balanza_comprobacion",
            get(obtener_balanza_comprobacion_handler),
        )
        .with_state(app_state)
}
