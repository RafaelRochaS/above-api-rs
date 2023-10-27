use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub name: String,
}
