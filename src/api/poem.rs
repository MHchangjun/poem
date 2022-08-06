use actix_web::{web, post, HttpResponse, Error};

use crate::db::poem;
use crate::db::poem::Poem;

use super::DbPool;

#[post("/poem")]
pub async fn add_poem(pool: web::Data<DbPool>, from: web::Json<poem::NewPoem>) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        Poem::insert(&from, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}