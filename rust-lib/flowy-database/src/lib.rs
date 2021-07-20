pub mod schema;

#[macro_use]
pub mod macros;

#[macro_use]
extern crate diesel;
pub use diesel::*;

#[macro_use]
extern crate diesel_derives;
pub use diesel_derives::*;

#[macro_use]
extern crate diesel_migrations;

pub use flowy_sqlite::{DBConnection, Database};

use diesel_migrations::*;
use flowy_sqlite::{Error, PoolConfig};
use std::{fmt::Debug, io, path::Path};

pub mod prelude {
    pub use super::UserDatabaseConnection;
    pub use diesel::{query_dsl::*, BelongingToDsl, ExpressionMethods, RunQueryDsl};
}

embed_migrations!("../flowy-database/migrations/");
pub const DB_NAME: &str = "flowy-database.db";

pub fn init(storage_path: &str) -> Result<Database, io::Error> {
    if !Path::new(storage_path).exists() {
        std::fs::create_dir_all(storage_path)?;
    }
    let pool_config = PoolConfig::default();
    let database = Database::new(storage_path, DB_NAME, pool_config).map_err(as_io_error)?;
    let conn = database.get_connection().map_err(as_io_error)?;
    let _ = embedded_migrations::run(&*conn).map_err(as_io_error)?;
    Ok(database)
}

fn as_io_error<E>(e: E) -> io::Error
where
    E: Into<Error> + Debug,
{
    let msg = format!("{:?}", e);
    io::Error::new(io::ErrorKind::NotConnected, msg)
}

pub trait UserDatabaseConnection: Send + Sync {
    fn get_connection(&self) -> Result<DBConnection, String>;
}
