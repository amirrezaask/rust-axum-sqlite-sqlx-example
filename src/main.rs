use axum::{routing::get, Router};
use sqlx::sqlite::SqlitePoolOptions;

mod db;
mod notes;

#[tokio::main]
async fn main() {
    let conn = SqlitePoolOptions::new()
        .connect("sqlite://backpack.sqlite")
        .await
        .unwrap();

    let app = Router::new()
        .route(
            "/notes",
            get(notes::handlers::index)
                .post(notes::handlers::new)
                .put(notes::handlers::update)
                .delete(notes::handlers::delete),
        )
        .with_state(conn);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
