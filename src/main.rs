use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Human {
    name: String,
}

async fn index() -> impl IntoResponse {
    Json(Human {
        name: "Hellow".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(index));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
