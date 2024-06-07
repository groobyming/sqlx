use super::fake_bk_sqlx as bk_sqlx;

impl_database_ext! {
    bk_sqlx::mysql::MySql {
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,

        // ordering is important here as otherwise we might infer strings to be binary
        // CHAR, VAR_CHAR, TEXT
        String,

        // BINARY, VAR_BINARY, BLOB
        Vec<u8>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        bk_sqlx::types::chrono::NaiveTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        bk_sqlx::types::chrono::NaiveDate,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        bk_sqlx::types::chrono::NaiveDateTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        bk_sqlx::types::chrono::DateTime<bk_sqlx::types::chrono::Utc>,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::Time,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::Date,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::OffsetDateTime,

        #[cfg(feature = "bigdecimal")]
        bk_sqlx::types::BigDecimal,

        #[cfg(feature = "rust_decimal")]
        bk_sqlx::types::Decimal,

        #[cfg(feature = "json")]
        bk_sqlx::types::JsonValue,
    },
    ParamChecking::Weak,
    feature-types: info => info.__type_feature_gate(),
    row: bk_sqlx::mysql::MySqlRow,
}
