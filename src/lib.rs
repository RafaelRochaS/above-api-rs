use axum::Router;

mod consts;
mod handlers;
mod models;
mod routes;

pub fn create_app() -> Router {
    return routes::router::initialize_routes();
}
