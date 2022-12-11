// use std::sync::Arc;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;

mod db;
mod notes;

#[tokio::main]
async fn main() {
    let conn = SqlitePoolOptions::new().connect("sqlite://backpack.sqlite").await.unwrap();
    // build our application with a single route
    let app = Router::new()
        // notes
        .with_state(conn)
        .route("/notes", get(notes::handlers::index))
        .route("/notes", post(notes::handlers::new))
        .route("/notes", put(notes::handlers::update))
        .route("/notes", delete(notes::handlers::delete));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
