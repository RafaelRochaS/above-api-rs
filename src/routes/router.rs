use axum::{routing::post, Router};

use crate::handlers::user::create_user;

pub fn initialize_routes() -> Router {
    Router::new().route("/:version/user", post(create_user))
}
