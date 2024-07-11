use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;

use super::user::User;

#[derive(Serialize)]
pub struct ReadUserResponse {
    id: String,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
}

impl From<User> for ReadUserResponse {
    fn from(user: User) -> Self {
        ReadUserResponse {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        }
    }
}

pub async fn read_user(Path(id): Path<String>) -> (StatusCode, Json<ReadUserResponse>) {
    // TODO: fetch from db
    let user = User {
        id,
        username: "username".to_owned(),
        first_name: "first-name".to_owned(),
        last_name: "last-name".to_owned(),
        email: "email".to_owned(),
        password: "password".to_owned(),
    };

    (StatusCode::OK, Json(user.into()))
}
