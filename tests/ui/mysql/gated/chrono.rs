fn main() {
    let _ = bk_sqlx::query!("select CONVERT(now(), DATE) date");

    let _ = bk_sqlx::query!("select CONVERT(now(), TIME) time");

    let _ = bk_sqlx::query!("select CONVERT(now(), DATETIME) datetime");
}
