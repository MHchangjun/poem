use actix_web::{get, HttpResponse, Responder};

#[get("/subject")]
pub async fn today_subject() -> impl Responder {
    HttpResponse::Ok()
        .body("삼행시")
}