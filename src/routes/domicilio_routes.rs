use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers::domicilio_handlers::create_new_calle, AppState};

pub fn domicilio_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/calle/nueva", post(create_new_calle))
        .with_state(app_state)
}
