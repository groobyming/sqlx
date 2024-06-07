// The no-arg variant is covered by other tests already.

use bk_sqlx::PgPool;

const MIGRATOR: bk_sqlx::migrate::Migrator = bk_sqlx::migrate!("tests/postgres/migrations");

#[bk_sqlx::test]
async fn it_gets_a_pool(pool: PgPool) -> bk_sqlx::Result<()> {
    let mut conn = pool.acquire().await?;

    let db_name: String = bk_sqlx::query_scalar("SELECT current_database()")
        .fetch_one(&mut *conn)
        .await?;

    assert!(db_name.starts_with("_bk_sqlx_test"), "dbname: {db_name:?}");

    Ok(())
}

// This should apply migrations and then `fixtures/users.sql`
#[bk_sqlx::test(migrations = "tests/postgres/migrations", fixtures("users"))]
async fn it_gets_users(pool: PgPool) -> bk_sqlx::Result<()> {
    let usernames: Vec<String> =
        bk_sqlx::query_scalar(r#"SELECT username FROM "user" ORDER BY username"#)
            .fetch_all(&pool)
            .await?;

    assert_eq!(usernames, ["alice", "bob"]);

    let post_exists: bool = bk_sqlx::query_scalar("SELECT exists(SELECT 1 FROM post)")
        .fetch_one(&pool)
        .await?;

    assert!(!post_exists);

    let comment_exists: bool = bk_sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then fixtures `fixtures/users.sql` and `fixtures/posts.sql`
#[bk_sqlx::test(migrations = "tests/postgres/migrations", fixtures("users", "posts"))]
async fn it_gets_posts(pool: PgPool) -> bk_sqlx::Result<()> {
    let post_contents: Vec<String> =
        bk_sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = bk_sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then `../fixtures/postgres/users.sql` and `../fixtures/postgres/posts.sql`
#[bk_sqlx::test(
    migrations = "tests/postgres/migrations",
    fixtures("../fixtures/postgres/users.sql", "../fixtures/postgres/posts.sql")
)]
async fn it_gets_posts_explicit_fixtures_path(pool: PgPool) -> bk_sqlx::Result<()> {
    let post_contents: Vec<String> =
        bk_sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = bk_sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then `../fixtures/postgres/users.sql` and `fixtures/posts.sql`
#[bk_sqlx::test(
    migrations = "tests/postgres/migrations",
    fixtures("../fixtures/postgres/users.sql"),
    fixtures("posts")
)]
async fn it_gets_posts_mixed_fixtures_path(pool: PgPool) -> bk_sqlx::Result<()> {
    let post_contents: Vec<String> =
        bk_sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = bk_sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// This should apply migrations and then `../fixtures/postgres/users.sql` and `../fixtures/postgres/posts.sql`
#[bk_sqlx::test(
    migrations = "tests/postgres/migrations",
    fixtures(path = "../fixtures/postgres", scripts("users.sql", "posts"))
)]
async fn it_gets_posts_custom_relative_fixtures_path(pool: PgPool) -> bk_sqlx::Result<()> {
    let post_contents: Vec<String> =
        bk_sqlx::query_scalar("SELECT content FROM post ORDER BY created_at")
            .fetch_all(&pool)
            .await?;

    assert_eq!(
        post_contents,
        [
            "This new computer is lightning-fast!",
            "@alice is a haxxor :("
        ]
    );

    let comment_exists: bool = bk_sqlx::query_scalar("SELECT exists(SELECT 1 FROM comment)")
        .fetch_one(&pool)
        .await?;

    assert!(!comment_exists);

    Ok(())
}

// Try `migrator`
#[bk_sqlx::test(migrator = "MIGRATOR", fixtures("users", "posts", "comments"))]
async fn it_gets_comments(pool: PgPool) -> bk_sqlx::Result<()> {
    let post_1_comments: Vec<String> = bk_sqlx::query_scalar(
        "SELECT content FROM comment WHERE post_id = $1::uuid ORDER BY created_at",
    )
    .bind(&"252c1d98-a9b0-4f18-8298-e59058bdfe16")
    .fetch_all(&pool)
    .await?;

    assert_eq!(
        post_1_comments,
        ["lol bet ur still bad, 1v1 me", "you're on!"]
    );

    let post_2_comments: Vec<String> = bk_sqlx::query_scalar(
        "SELECT content FROM comment WHERE post_id = $1::uuid ORDER BY created_at",
    )
    .bind(&"844265f7-2472-4689-9a2e-b21f40dbf401")
    .fetch_all(&pool)
    .await?;

    assert_eq!(post_2_comments, ["lol you're just mad you lost :P"]);

    Ok(())
}
