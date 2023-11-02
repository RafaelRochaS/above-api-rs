use std::collections::HashMap;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

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

    (StatusCode::CREATED, Json(user.id))
}

pub async fn get_user(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let id = params.get("id").expect("no id found!");

    tracing::debug!("Request for 'get user' id: {}", id);

    let user = users::User {
        id: 1337,
        name: "Joe da quebrada".to_string(),
        address: "address".to_string(),
        email: "email".to_string(),
        age: 200,
    };

    (StatusCode::OK, Json(user))
}
