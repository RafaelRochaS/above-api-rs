use axum::{routing::post, Router};

use crate::handlers::user::create_user;

pub fn initialize_routes() -> Router {
    let user_routes_v1 = Router::new().route("/user", post(create_user));

    let v1_group = Router::new().nest("/users", user_routes_v1);

    Router::new().nest("/api/v1", v1_group)
}
