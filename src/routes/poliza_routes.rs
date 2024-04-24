use std::sync::Arc;

use axum::{middleware, routing::post, Router};

use crate::{
    handlers::poliza_handlers::crear_nueva_poliza_handler,
    middlewares::jwt_middlewares::auth_required, AppState,
};

pub fn poliza_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/poliza/nueva",
            post(crear_nueva_poliza_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .with_state(app_state)
}
