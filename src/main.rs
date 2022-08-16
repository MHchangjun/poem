mod subject;
mod db;
mod api;

#[macro_use]
extern crate diesel;

use std::env;
use actix_web::{App, HttpServer, web};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, SqliteConnection};
use crate::api::poem;


type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let conn_spec = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(subject::get)
            .service(subject::set)
            .service(poem::add_poem)
            .service(poem::get_poem)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}