use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers::sucursal_handlers::crear_nueva_sucursal_handler, AppState};

pub fn sucursal_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/sucursal/nueva", post(crear_nueva_sucursal_handler))
        .with_state(app_state)
}
