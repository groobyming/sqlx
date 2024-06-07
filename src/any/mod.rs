//! **SEE DOCUMENTATION BEFORE USE**. Runtime-generic database driver.
#![doc = include_str!("install_drivers_note.md")]

use std::sync::Once;

pub use bk_sqlx_core::any::driver::install_drivers;

pub use bk_sqlx_core::any::{
    Any, AnyArguments, AnyConnectOptions, AnyExecutor, AnyKind, AnyPoolOptions, AnyQueryResult,
    AnyRow, AnyStatement, AnyTransactionManager, AnyTypeInfo, AnyValue, AnyValueRef,
};

pub(crate) mod reexports {
    /// **SEE DOCUMENTATION BEFORE USE**. Type alias for `Pool<Any>`.
    #[doc = include_str!("install_drivers_note.md")]
    pub use bk_sqlx_core::any::AnyPool;

    /// **SEE DOCUMENTATION BEFORE USE**. Runtime-generic database connection.
    #[doc = include_str!("install_drivers_note.md")]
    pub use bk_sqlx_core::any::AnyConnection;
}

/// Install all currently compiled-in drivers for [`AnyConnection`] to use.
///
/// May be called multiple times; only the first call will install drivers, subsequent calls
/// will have no effect.
///
/// ### Panics
/// If [`install_drivers`] has already been called *not* through this function.
pub fn install_default_drivers() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        install_drivers(&[
            #[cfg(feature = "mysql")]
            bk_sqlx_mysql::any::DRIVER,
            #[cfg(feature = "postgres")]
            bk_sqlx_postgres::any::DRIVER,
            #[cfg(feature = "sqlite")]
            bk_sqlx_sqlite::any::DRIVER,
        ])
        .expect("non-default drivers already installed")
    });
}
