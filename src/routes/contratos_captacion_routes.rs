// use std::sync::Arc;
//
// use axum::{routing::post, Router};
//
// use crate::AppState;
//
// pub fn persona_router(app_state: Arc<AppState>) -> Router {
//     Router::new()
//         .route(
//             "/api/persona/tutor_persona/nuevo/:id_persona",
//             post(crear_nuevo_tutor_de_persona_handler),
//         )
//         .with_state(app_state)
// }
