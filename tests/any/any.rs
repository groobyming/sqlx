use bk_sqlx::any::AnyRow;
use bk_sqlx::{Any, Connection, Executor, Row};
use bk_sqlx_test::new;

#[bk_sqlx_macros::test]
async fn it_connects() -> anyhow::Result<()> {
    bk_sqlx::any::install_default_drivers();

    let mut conn = new::<Any>().await?;

    let value = bk_sqlx::query("select 1 + 5")
        .try_map(|row: AnyRow| row.try_get::<i32, _>(0))
        .fetch_one(&mut conn)
        .await?;

    assert_eq!(6i32, value);

    conn.close().await?;

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_pings() -> anyhow::Result<()> {
    bk_sqlx::any::install_default_drivers();

    let mut conn = new::<Any>().await?;

    conn.ping().await?;

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_executes_with_pool() -> anyhow::Result<()> {
    bk_sqlx::any::install_default_drivers();

    let pool = bk_sqlx_test::pool::<Any>().await?;

    let rows = pool.fetch_all("SELECT 1; SElECT 2").await?;

    assert_eq!(rows.len(), 2);

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_does_not_stop_stream_after_decoding_error() -> anyhow::Result<()> {
    use futures::stream::StreamExt;

    bk_sqlx::any::install_default_drivers();

    // see https://github.com/launchbadge/bk_sqlx/issues/1884
    let pool = bk_sqlx_test::pool::<Any>().await?;

    #[derive(Debug, PartialEq)]
    struct MyType;
    impl<'a> bk_sqlx::FromRow<'a, AnyRow> for MyType {
        fn from_row(row: &'a AnyRow) -> bk_sqlx::Result<Self> {
            let n = row.try_get::<i32, _>(0)?;
            if n == 1 {
                Err(bk_sqlx::Error::RowNotFound)
            } else {
                Ok(MyType)
            }
        }
    }

    let rows = bk_sqlx::query_as("SELECT 0 UNION ALL SELECT 1 UNION ALL SELECT 2")
        .fetch(&pool)
        .map(|r| r.ok())
        .collect::<Vec<_>>()
        .await;

    assert_eq!(rows, vec![Some(MyType), None, Some(MyType)]);
    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_gets_by_name() -> anyhow::Result<()> {
    bk_sqlx::any::install_default_drivers();

    let mut conn = new::<Any>().await?;

    let row = conn.fetch_one("SELECT 1 as _1").await?;
    let val: i32 = row.get("_1");

    assert_eq!(val, 1);

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_can_fail_and_recover() -> anyhow::Result<()> {
    bk_sqlx::any::install_default_drivers();

    let mut conn = new::<Any>().await?;

    for i in 0..10 {
        // make a query that will fail
        let res = conn
            .execute("INSERT INTO not_found (column) VALUES (10)")
            .await;

        assert!(res.is_err());

        // now try and use the connection
        let val: i32 = conn
            .fetch_one(&*format!("SELECT {i}"))
            .await?
            .get_unchecked(0);

        assert_eq!(val, i);
    }

    Ok(())
}

#[bk_sqlx_macros::test]
async fn it_can_fail_and_recover_with_pool() -> anyhow::Result<()> {
    bk_sqlx::any::install_default_drivers();

    let pool = bk_sqlx_test::pool::<Any>().await?;

    for i in 0..10 {
        // make a query that will fail
        let res = pool
            .execute("INSERT INTO not_found (column) VALUES (10)")
            .await;

        assert!(res.is_err());

        // now try and use the connection
        let val: i32 = pool
            .fetch_one(&*format!("SELECT {i}"))
            .await?
            .get_unchecked(0);

        assert_eq!(val, i);
    }

    Ok(())
}
