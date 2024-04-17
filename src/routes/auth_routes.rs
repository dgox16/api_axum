use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers::auth_handlers::register_user_handler, AppState};

pub fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register_user_handler))
        .with_state(app_state)
}
