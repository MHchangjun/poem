use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection
};

pub mod poem;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;