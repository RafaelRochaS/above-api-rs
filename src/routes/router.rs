use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::user::{create_user, get_user};

pub fn initialize_routes() -> Router {
    let user_routes_v1 = Router::new()
        .route("/user", post(create_user))
        .route("/user", get(get_user));

    let v1_group = Router::new().nest("/users", user_routes_v1);

    Router::new().nest("/api/v1", v1_group)
}
