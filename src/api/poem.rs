use actix_web::{Error, get, HttpResponse, post, web};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::db::poem::Poem;

use super::DbPool;

#[post("/poem")]
pub async fn add_poem(pool: web::Data<DbPool>, from: web::Json<NewPoem>) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        Poem::insert(&from, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/poem/{id}")]
pub async fn get_poem(pool: web::Data<DbPool>, from: web::Path<String>) -> Result<HttpResponse, Error> {
    let subject_id = from.into_inner();

    let poem = web::block(move || {
        let conn = pool.get()?;
        Poem::get(&subject_id, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let poem = poem.iter()
        .map(|x| to_response(x))
        .collect::<Vec<PoemResponse>>();

    Ok(HttpResponse::Ok().json(poem))
}

#[post("/poem/like/{poem_id}")]
pub async fn like_poem(pool: web::Data<DbPool>, from: web::Path<String>) -> Result<HttpResponse, Error> {
    let poem_id = from.into_inner();

    web::block(move || {
        let conn = pool.get()?;
        Poem::like(&poem_id, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Deserialize)]
pub struct NewPoem {
    pub subject_id: String,
    pub body: Vec<String>,
}

impl NewPoem {
    pub fn to_entity(&self) -> Poem {
        Poem {
            id: Uuid::new_v4().to_string(),
            subject_id: self.subject_id.to_owned(),
            body: serde_json::to_string(&self.body.to_owned()).expect("not json array type"),
            like: 0
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PoemResponse {
    pub id: String,
    pub subject_id: String,
    pub body: Vec<String>,
    pub like: i32
}

fn to_response(poem: &Poem) -> PoemResponse {
    let _body: Vec<String> = serde_json::from_str(&poem.body).expect("");

    PoemResponse {
        id: poem.id.to_owned(),
        subject_id: poem.subject_id.to_owned(),
        body: _body,
        like: poem.like
    }
}