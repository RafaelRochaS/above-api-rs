use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::users;

pub async fn create_user(Json(payload): Json<users::CreateUser>) -> impl IntoResponse {
    let user = users::User {
        id: 1337,
        username: payload.username,
        name: payload.first_name + &payload.last_name,
    };

    (StatusCode::CREATED, Json(user))
}
