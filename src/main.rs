use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

mod notes;

async fn migrate(pool: &Pool<Sqlite>) -> Result<()> {
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS notes (id integer primary key, content text)")
        .execute(pool).await?;


    Ok(())
}

#[tokio::main]
async fn main() {
    let conn = SqlitePoolOptions::new()
        .connect("backpack.sqlite")
        .await
        .unwrap();


    migrate(&conn).await.unwrap();

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
