use super::fake_bk_sqlx as bk_sqlx;

impl_database_ext! {
    bk_sqlx::postgres::Postgres {
        (),
        bool,
        String | &str,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        Vec<u8> | &[u8],

        bk_sqlx::postgres::types::Oid,

        bk_sqlx::postgres::types::PgInterval,

        bk_sqlx::postgres::types::PgMoney,

        bk_sqlx::postgres::types::PgLTree,

        bk_sqlx::postgres::types::PgLQuery,

        #[cfg(feature = "uuid")]
        bk_sqlx::types::Uuid,

        #[cfg(feature = "chrono")]
        bk_sqlx::types::chrono::NaiveTime,

        #[cfg(feature = "chrono")]
        bk_sqlx::types::chrono::NaiveDate,

        #[cfg(feature = "chrono")]
        bk_sqlx::types::chrono::NaiveDateTime,

        #[cfg(feature = "chrono")]
        bk_sqlx::types::chrono::DateTime<bk_sqlx::types::chrono::Utc> | bk_sqlx::types::chrono::DateTime<_>,

        #[cfg(feature = "chrono")]
        bk_sqlx::postgres::types::PgTimeTz<bk_sqlx::types::chrono::NaiveTime, bk_sqlx::types::chrono::FixedOffset>,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::Time,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::Date,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        bk_sqlx::types::time::OffsetDateTime,

        #[cfg(feature = "time")]
        bk_sqlx::postgres::types::PgTimeTz<bk_sqlx::types::time::Time, bk_sqlx::types::time::UtcOffset>,

        #[cfg(feature = "bigdecimal")]
        bk_sqlx::types::BigDecimal,

        #[cfg(feature = "rust_decimal")]
        bk_sqlx::types::Decimal,

        #[cfg(feature = "ipnetwork")]
        bk_sqlx::types::ipnetwork::IpNetwork,

        #[cfg(feature = "mac_address")]
        bk_sqlx::types::mac_address::MacAddress,

        #[cfg(feature = "json")]
        bk_sqlx::types::JsonValue,

        #[cfg(feature = "bit-vec")]
        bk_sqlx::types::BitVec,

        // Arrays

        Vec<bool> | &[bool],
        Vec<String> | &[String],
        Vec<Vec<u8>> | &[Vec<u8>],
        Vec<i8> | &[i8],
        Vec<i16> | &[i16],
        Vec<i32> | &[i32],
        Vec<i64> | &[i64],
        Vec<f32> | &[f32],
        Vec<f64> | &[f64],
        Vec<bk_sqlx::postgres::types::Oid> | &[bk_sqlx::postgres::types::Oid],
        Vec<bk_sqlx::postgres::types::PgMoney> | &[bk_sqlx::postgres::types::PgMoney],

        #[cfg(feature = "uuid")]
        Vec<bk_sqlx::types::Uuid> | &[bk_sqlx::types::Uuid],

        #[cfg(feature = "chrono")]
        Vec<bk_sqlx::types::chrono::NaiveTime> | &[bk_sqlx::types::chrono::NaiveTime],

        #[cfg(feature = "chrono")]
        Vec<bk_sqlx::types::chrono::NaiveDate> | &[bk_sqlx::types::chrono::NaiveDate],

        #[cfg(feature = "chrono")]
        Vec<bk_sqlx::types::chrono::NaiveDateTime> | &[bk_sqlx::types::chrono::NaiveDateTime],

        #[cfg(feature = "chrono")]
        Vec<bk_sqlx::types::chrono::DateTime<bk_sqlx::types::chrono::Utc>> | &[bk_sqlx::types::chrono::DateTime<_>],

        #[cfg(feature = "time")]
        Vec<bk_sqlx::types::time::Time> | &[bk_sqlx::types::time::Time],

        #[cfg(feature = "time")]
        Vec<bk_sqlx::types::time::Date> | &[bk_sqlx::types::time::Date],

        #[cfg(feature = "time")]
        Vec<bk_sqlx::types::time::PrimitiveDateTime> | &[bk_sqlx::types::time::PrimitiveDateTime],

        #[cfg(feature = "time")]
        Vec<bk_sqlx::types::time::OffsetDateTime> | &[bk_sqlx::types::time::OffsetDateTime],

        #[cfg(feature = "bigdecimal")]
        Vec<bk_sqlx::types::BigDecimal> | &[bk_sqlx::types::BigDecimal],

        #[cfg(feature = "rust_decimal")]
        Vec<bk_sqlx::types::Decimal> | &[bk_sqlx::types::Decimal],

        #[cfg(feature = "ipnetwork")]
        Vec<bk_sqlx::types::ipnetwork::IpNetwork> | &[bk_sqlx::types::ipnetwork::IpNetwork],

        #[cfg(feature = "mac_address")]
        Vec<bk_sqlx::types::mac_address::MacAddress> | &[bk_sqlx::types::mac_address::MacAddress],

        #[cfg(feature = "json")]
        Vec<bk_sqlx::types::JsonValue> | &[bk_sqlx::types::JsonValue],

        // Ranges

        bk_sqlx::postgres::types::PgRange<i32>,
        bk_sqlx::postgres::types::PgRange<i64>,

        #[cfg(feature = "bigdecimal")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::BigDecimal>,

        #[cfg(feature = "rust_decimal")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::Decimal>,

        #[cfg(feature = "chrono")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::NaiveDate>,

        #[cfg(feature = "chrono")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::NaiveDateTime>,

        #[cfg(feature = "chrono")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::DateTime<bk_sqlx::types::chrono::Utc>> |
            bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::DateTime<_>>,

        #[cfg(feature = "time")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::Date>,

        #[cfg(feature = "time")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::PrimitiveDateTime>,

        #[cfg(feature = "time")]
        bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::OffsetDateTime>,

        // Range arrays

        Vec<bk_sqlx::postgres::types::PgRange<i32>> | &[bk_sqlx::postgres::types::PgRange<i32>],
        Vec<bk_sqlx::postgres::types::PgRange<i64>> | &[bk_sqlx::postgres::types::PgRange<i64>],

        #[cfg(feature = "bigdecimal")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::BigDecimal>> |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::BigDecimal>],

        #[cfg(feature = "rust_decimal")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::Decimal>> |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::Decimal>],

        #[cfg(feature = "chrono")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::NaiveDate>> |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::NaiveDate>],

        #[cfg(feature = "chrono")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::NaiveDateTime>> |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::NaiveDateTime>],

        #[cfg(feature = "chrono")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::DateTime<bk_sqlx::types::chrono::Utc>>> |
            Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::DateTime<_>>>,

        #[cfg(feature = "chrono")]
        &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::DateTime<bk_sqlx::types::chrono::Utc>>] |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::chrono::DateTime<_>>],

        #[cfg(feature = "time")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::Date>> |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::Date>],

        #[cfg(feature = "time")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::PrimitiveDateTime>> |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::PrimitiveDateTime>],

        #[cfg(feature = "time")]
        Vec<bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::OffsetDateTime>> |
            &[bk_sqlx::postgres::types::PgRange<bk_sqlx::types::time::OffsetDateTime>],
    },
    ParamChecking::Strong,
    feature-types: info => info.__type_feature_gate(),
    row: bk_sqlx::postgres::PgRow,
}
