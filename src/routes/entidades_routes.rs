use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
    handlers::entidades_handlers::{crear_nueva_sucursal_handler, crear_nuevo_banco_handler},
    AppState,
};

pub fn entidades_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/sucursal/nueva", post(crear_nueva_sucursal_handler))
        .route("/api/banco/nuevo", post(crear_nuevo_banco_handler))
        .with_state(app_state)
}
