mod create_db;
mod persona;
mod routes;
mod templates;
use actix_files::Files;
use actix_web::{App, HttpServer};
use create_db::make_persona_db;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "src/static/.").show_files_listing())
            .service(persona_list)
            .service(skills)
            .service(persona_details)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
