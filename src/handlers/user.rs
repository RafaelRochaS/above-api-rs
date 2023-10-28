use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::users;

pub async fn create_user(Json(payload): Json<users::CreateUser>) -> impl IntoResponse {
    tracing::debug!("Received 'create user' request");

    let user = users::User {
        id: 1337,
        name: payload.first_name + &payload.last_name,
        address: payload.address,
        email: payload.email,
        age: payload.age,
    };

    tracing::debug!("Created user with id: {}", user.id);

    (StatusCode::CREATED, Json(user))
}
