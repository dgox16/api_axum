use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handlers::auth_handlers::{
        get_me_handler, login_user_handler, logout_handler, register_user_handler,
    },
    middlewares::jwt_middlewares::auth_required,
    AppState,
};

pub fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route(
            "/api/auth/logout",
            get(logout_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .route(
            "/api/users/me",
            get(get_me_handler).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_required,
            )),
        )
        .with_state(app_state)
}
