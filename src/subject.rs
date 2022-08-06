use diesel::prelude::*;
use actix_web::{web, get, post, HttpResponse, Responder, Error};

use crate::DbPool;
use crate::models;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;
use crate::db::db_error::DbError;
use crate::db::models;
use super::db;

use super::db_error::DbError;

pub fn select_subject(
    date: &str,
    conn: &SqliteConnection
) -> Result<models::Subject, DbError> {
    use crate::db::schema::subjects::dsl::*;

    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok().expect("");

    let _dt = NaiveDateTime::new(
        date,
        NaiveTime::from_hms(0, 0, 0)
    );

    let _subject = subjects.filter(dt.eq(_dt))
        .first::<models::Subject>(conn)
        .expect("");

    Ok(_subject)
}

#[get("/subject/{date}")]
pub async fn get(pool: web::Data<DbPool>, from: web::Path<String>) -> Result<HttpResponse, Error> {
    let date = from.into_inner();

    let subject = web::block(move || {
        let conn = pool.get()?;
        select_subject(&date, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(subject.to_response()))
}

pub fn insert_subject(
    _subject: &str,
    _date: &str,
    conn: &SqliteConnection
) -> Result<models::Subject, DbError> {
    use crate::db::schema::subjects::dsl::*;

    let _date = NaiveDate::parse_from_str(_date, "%Y-%m-%d");
    match _date {
        Ok(_) => {}
        Err(error) => {
            println!("error: {}", error);
        }
    }

    let _dt = NaiveDateTime::new(
        _date.ok().expect(""),
        NaiveTime::from_hms(0, 0, 0)
    );

    let new_subject = models::Subject {
        id: Uuid::new_v4().to_string(),
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