mod poem;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(poem::today_subject)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}