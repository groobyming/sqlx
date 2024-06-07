use bk_sqlx::{error::ErrorKind, postgres::Postgres, Connection};
use bk_sqlx_test::new;

#[bk_sqlx_macros::test]
async fn it_fails_with_unique_violation() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut tx = conn.begin().await?;

    bk_sqlx::query("INSERT INTO tweet(id, text, owner_id) VALUES (1, 'Foo', 1);")
        .execute(&mut *tx)
        .await?;

    let res: Result<_, bk_sqlx::Error> = bk_sqlx::query("INSERT INTO tweet VALUES (1, NOW(), 'Foo', 1);")
        .execute(&mut *tx)
        .await;
    let err = res.unwrap_err();

    let err = err.into_database_error().unwrap();

    assert_eq!(err.kind(), ErrorKind::UniqueViolation);

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_fails_with_foreign_key_violation() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut tx = conn.begin().await?;

    let res: Result<_, bk_sqlx::Error> =
        bk_sqlx::query("INSERT INTO tweet_reply (tweet_id, text) VALUES (1, 'Reply!');")
            .execute(&mut *tx)
            .await;
    let err = res.unwrap_err();

    let err = err.into_database_error().unwrap();

    assert_eq!(err.kind(), ErrorKind::ForeignKeyViolation);

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_fails_with_not_null_violation() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut tx = conn.begin().await?;

    let res: Result<_, bk_sqlx::Error> = bk_sqlx::query("INSERT INTO tweet (text) VALUES (null);")
        .execute(&mut *tx)
        .await;
    let err = res.unwrap_err();

    let err = err.into_database_error().unwrap();

    assert_eq!(err.kind(), ErrorKind::NotNullViolation);

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_fails_with_check_violation() -> anyhow::Result<()> {
    let mut conn = new::<Postgres>().await?;
    let mut tx = conn.begin().await?;

    let res: Result<_, bk_sqlx::Error> =
        bk_sqlx::query("INSERT INTO products VALUES (1, 'Product 1', 0);")
            .execute(&mut *tx)
            .await;
    let err = res.unwrap_err();

    let err = err.into_database_error().unwrap();

    assert_eq!(err.kind(), ErrorKind::CheckViolation);

    Ok(())
}
