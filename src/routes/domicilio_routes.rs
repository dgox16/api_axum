use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::domicilio_handlers::{buscar_calle_handler, crear_nueva_calle_handler},
    AppState,
};

pub fn domicilio_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/calle/nueva", post(crear_nueva_calle_handler))
        .route("/api/calle/buscar/", get(buscar_calle_handler))
        .with_state(app_state)
}
