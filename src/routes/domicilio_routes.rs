use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::domicilio_handlers::{create_new_calle, search_calle},
    AppState,
};

pub fn domicilio_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/calle/nueva", post(create_new_calle))
        .route("/api/calle/buscar/", get(search_calle))
        .with_state(app_state)
}
