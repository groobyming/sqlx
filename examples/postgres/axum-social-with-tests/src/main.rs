use anyhow::Context;
use bk_sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL")
        // The error from `var()` doesn't mention the environment variable.
        .context("DATABASE_URL must be set")?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("failed to connect to DATABASE_URL")?;

    bk_sqlx::migrate!().run(&db).await?;

    bk_sqlx_example_postgres_axum_social::http::serve(db).await
}
