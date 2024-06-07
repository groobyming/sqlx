fn main() {
    let _ = bk_sqlx::query!("select now()::date");

    let _ = bk_sqlx::query!("select now()::time");

    let _ = bk_sqlx::query!("select now()::timestamp");

    let _ = bk_sqlx::query!("select now()::timestamptz");

    let _ = bk_sqlx::query!("select $1::date", ());

    let _ = bk_sqlx::query!("select $1::time", ());

    let _ = bk_sqlx::query!("select $1::timestamp", ());

    let _ = bk_sqlx::query!("select $1::timestamptz", ());
}
