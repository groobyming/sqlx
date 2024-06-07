fn main() {
    let query = bk_sqlx::query!("select 1 as \"'1\"");
}
