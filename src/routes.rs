use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn photos() -> Json<Value> {
    Json(json!({
      "photos ": [
        { "id": 0, "lat": 10, "long": 10 },
        { "id": 1, "lat": 10, "long": 10 }
      ]
    }))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}
