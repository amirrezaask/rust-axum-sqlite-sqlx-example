use axum::{
    routing::{delete, get, post, put},
    Router,
};
mod db;
mod notes;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        // notes
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
