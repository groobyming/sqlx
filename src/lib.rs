#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("lib.md")]

pub use bk_sqlx_core::acquire::Acquire;
pub use bk_sqlx_core::arguments::{Arguments, IntoArguments};
pub use bk_sqlx_core::column::Column;
pub use bk_sqlx_core::column::ColumnIndex;
pub use bk_sqlx_core::connection::{ConnectOptions, Connection};
pub use bk_sqlx_core::database::{self, Database};
pub use bk_sqlx_core::describe::Describe;
pub use bk_sqlx_core::executor::{Execute, Executor};
pub use bk_sqlx_core::from_row::FromRow;
pub use bk_sqlx_core::pool::{self, Pool};
pub use bk_sqlx_core::query::{query, query_with};
pub use bk_sqlx_core::query_as::{query_as, query_as_with};
pub use bk_sqlx_core::query_builder::{self, QueryBuilder};
pub use bk_sqlx_core::query_scalar::{query_scalar, query_scalar_with};
pub use bk_sqlx_core::raw_sql::{raw_sql, RawSql};
pub use bk_sqlx_core::row::Row;
pub use bk_sqlx_core::statement::Statement;
pub use bk_sqlx_core::transaction::{Transaction, TransactionManager};
pub use bk_sqlx_core::type_info::TypeInfo;
pub use bk_sqlx_core::types::Type;
pub use bk_sqlx_core::value::{Value, ValueRef};
pub use bk_sqlx_core::Either;

#[doc(inline)]
pub use bk_sqlx_core::error::{self, Error, Result};

#[cfg(feature = "migrate")]
pub use bk_sqlx_core::migrate;

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
#[doc(inline)]
pub use bk_sqlx_mysql::{self as mysql, MySql, MySqlConnection, MySqlExecutor, MySqlPool};

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
#[doc(inline)]
pub use bk_sqlx_postgres::{self as postgres, PgConnection, PgExecutor, PgPool, Postgres};

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
#[doc(inline)]
pub use bk_sqlx_sqlite::{self as sqlite, Sqlite, SqliteConnection, SqliteExecutor, SqlitePool};

#[cfg(feature = "any")]
#[cfg_attr(docsrs, doc(cfg(feature = "any")))]
pub use crate::any::{reexports::*, Any, AnyExecutor};

#[cfg(feature = "macros")]
#[doc(hidden)]
pub extern crate bk_sqlx_macros;

// derives
#[cfg(feature = "macros")]
#[doc(hidden)]
pub use bk_sqlx_macros::{FromRow, Type};

// We can't do our normal facade approach with an attribute, but thankfully we can now
// have docs out-of-line quite easily.
#[doc = include_str!("macros/test.md")]
#[cfg(feature = "macros")]
pub use bk_sqlx_macros::test;

#[doc(hidden)]
#[cfg(feature = "migrate")]
pub use bk_sqlx_core::testing;

#[doc(hidden)]
pub use bk_sqlx_core::rt::test_block_on;

#[cfg(feature = "any")]
pub mod any;

#[cfg(feature = "macros")]
mod macros;

// macro support
#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod ty_match;

#[doc(hidden)]
pub use bk_sqlx_core::rt as __rt;

/// Conversions between Rust and SQL types.
///
/// To see how each SQL type maps to a Rust type, see the corresponding `types` module for each
/// database:
///
///  * Postgres: [postgres::types]
///  * MySQL: [mysql::types]
///  * SQLite: [sqlite::types]
///  * MSSQL: [mssql::types]
///
/// Any external types that have had [`Type`] implemented for, are re-exported in this module
/// for convenience as downstream users need to use a compatible version of the external crate
/// to take advantage of the implementation.
///
/// [`Type`]: types::Type
pub mod types {
    pub use bk_sqlx_core::types::*;

    #[cfg(feature = "macros")]
    #[doc(hidden)]
    pub use bk_sqlx_macros::Type;
}

/// Provides [`Encode`](encode::Encode) for encoding values for the database.
pub mod encode {
    pub use bk_sqlx_core::encode::{Encode, IsNull};

    #[cfg(feature = "macros")]
    #[doc(hidden)]
    pub use bk_sqlx_macros::Encode;
}

pub use self::encode::Encode;

/// Provides [`Decode`](decode::Decode) for decoding values from the database.
pub mod decode {
    pub use bk_sqlx_core::decode::Decode;

    #[cfg(feature = "macros")]
    #[doc(hidden)]
    pub use bk_sqlx_macros::Decode;
}

pub use self::decode::Decode;

/// Types and traits for the `query` family of functions and macros.
pub mod query {
    pub use bk_sqlx_core::query::{Map, Query};
    pub use bk_sqlx_core::query_as::QueryAs;
    pub use bk_sqlx_core::query_scalar::QueryScalar;
}

/// Convenience re-export of common traits.
pub mod prelude {
    pub use super::Acquire;
    pub use super::ConnectOptions;
    pub use super::Connection;
    pub use super::Decode;
    pub use super::Encode;
    pub use super::Executor;
    pub use super::FromRow;
    pub use super::IntoArguments;
    pub use super::Row;
    pub use super::Statement;
    pub use super::Type;
}
