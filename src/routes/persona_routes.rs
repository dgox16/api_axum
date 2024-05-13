use std::sync::Arc;

use axum::{middleware, routing::post, Router};

use crate::{
    handlers::persona_handlers::crear_nueva_persona_handler,
    middlewares::jwt_middlewares::auth_required, AppState,
};

pub fn persona_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/persona/nueva",
            post(crear_nueva_persona_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .with_state(app_state)
}
