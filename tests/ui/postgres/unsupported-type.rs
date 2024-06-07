fn main() {
    // we're probably not going to get around to the geometric types anytime soon
    let _ = bk_sqlx::query!("select null::circle");
    let _ = bk_sqlx::query!("select $1::circle", panic!());
}
