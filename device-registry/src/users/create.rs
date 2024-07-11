use axum::{http::StatusCode, Json};
use cuid::cuid2;
use serde::Deserialize;

use super::{read::ReadUserResponse, user::User};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<ReadUserResponse>) {
    let user = User {
        id: cuid2(),
        username: payload.username,
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        password: payload.password, // TODO: encrypt
    };

    // TODO: save to db

    (StatusCode::CREATED, Json(user.into()))
}
