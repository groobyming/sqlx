use super::fake_bk_sqlx as bk_sqlx;

// f32 is not included below as REAL represents a floating point value
// stored as an 8-byte IEEE floating point number
// For more info see: https://www.sqlite.org/datatype3.html#storage_classes_and_datatypes
impl_database_ext! {
    bk_sqlx::sqlite::Sqlite {
        bool,
        i32,
        i64,
        f64,
        String,
        Vec<u8>,

        #[cfg(feature = "chrono")]
        bk_sqlx::types::chrono::NaiveDate,

        #[cfg(feature = "chrono")]
        bk_sqlx::types::chrono::NaiveDateTime,

        #[cfg(feature = "chrono")]
        bk_sqlx::types::chrono::DateTime<bk_sqlx::types::chrono::Utc> | bk_sqlx::types::chrono::DateTime<_>,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::OffsetDateTime,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::Date,

        #[cfg(feature = "uuid")]
        bk_sqlx::types::Uuid,
    },
    ParamChecking::Weak,
    feature-types: _info => None,
    row: bk_sqlx::sqlite::SqliteRow,
    // Since proc-macros don't benefit from async, we can make a describe call directly
    // which also ensures that the database is closed afterwards, regardless of errors.
    describe-blocking: bk_sqlx_sqlite::describe_blocking,
}
