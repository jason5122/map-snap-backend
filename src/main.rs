use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/photos", get(photos))
        .route("/users", post(create_user));

    let address = if cfg!(debug_assertions) {
        "localhost"
    } else {
        "0.0.0.0"
    };
    let port = "8000";
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", address, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn photos() -> Json<Value> {
    Json(json!({
      "photos ": [
        { "id": 0, "lat": 10, "long": 10 },
        { "id": 1, "lat": 10, "long": 10 }
      ]
    }))
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
