use diesel::prelude::*;
use actix_web::{web, get, post, HttpResponse, Responder};

use crate::DbPool;
use crate::models;
use chrono::{NaiveDateTime};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/subject")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok()
        .body("삼행시")
}

pub fn insert_subject(
    _subject: &str,
    _dt: &str,
    conn: &SqliteConnection
) -> Result<models::Subject, DbError> {
    use crate::schema::subjects::dsl::*;

    let _dt = NaiveDateTime::parse_from_str(_dt, "%Y-%m-%d").ok();

    let new_subject = models::Subject {
        id: "123".to_owned(),
        subject: _subject.to_owned(),
        dt: _dt
    };

    diesel::insert_into(subjects)
        .values(&new_subject)
        .execute(conn)?;

    Ok(new_subject)
}

#[post("/subject")]
pub async fn set(pool: web::Data<DbPool>, from: web::Json<models::NewSubject>) -> impl Responder {
    let subject = web::block(move || {
        let conn = pool.get()?;
        insert_subject(&from.subject, &from.dt, &conn)
    })
        .await;

    match subject {
        Ok(_) => {
            HttpResponse::Ok()
                // .body(subject)
        }
        Err(_) => {
            HttpResponse::NoContent()
        }
    }
}