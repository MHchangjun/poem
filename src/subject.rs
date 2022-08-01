use actix_web::{get, post, HttpResponse, Responder};

#[get("/subject")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok()
        .body("삼행시")
}

#[post("/subject")]
pub async fn set(subject: String) -> impl Responder {
    HttpResponse::Ok()
        .body(subject)
}