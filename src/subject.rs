use diesel::prelude::*;
use actix_web::{web, get, post, HttpResponse, Responder};

use crate::DbPool;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/subject")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok()
        .body("삼행시")
}

pub fn insert_subject(
    _subject: &str,
    conn: &SqliteConnection
) -> Result<models::Subject, DbError> {
    use crate::schema::subjects::dsl::*;

    let new_subject = models::Subject {
        id: "test".to_owned(),
        subject: _subject.to_owned(),
    };

    diesel::insert_into(subjects)
        .values(&new_subject)
        .execute(conn)?;

    Ok(new_subject)
}

#[post("/subject")]
pub async fn set(pool: web::Data<DbPool>, subject: String) -> impl Responder {
    let subject = web::block(move || {
        let conn = pool.get()?;
        insert_subject(&subject, &conn)
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