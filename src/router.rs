use std::sync::Arc;

use axum::Router;

use crate::{
    routes::{auth_routes::auth_router, test_routes::test_router},
    AppState,
};

// Creamos un enrutador en un archivo aparte
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(auth_router(app_state))
        .merge(test_router())
}
