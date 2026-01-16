mod create_db;
mod persona;
mod templates;

use actix_files::Files;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web::Path,
};
use templates::*;

use crate::create_db::make_db::make_persona_db;

#[get("/persona_list")]
async fn persona_list(req: HttpRequest) -> impl Responder {
    let template = PersonaListTemplate {
        persona_list: &make_persona_db(),
    };
    HttpResponse::Ok().body(template.render().unwrap())
}

#[get("/skill_list")]
async fn skills() -> impl Responder {
    HttpResponse::Ok().body("Todo")
}

#[get("/persona/{persona}")]
async fn persona_details(path: Path<String>) -> impl Responder {
    let persona_name = path.into_inner();
    HttpResponse::Ok().body(format!("TODO: get details for {}", persona_name))
}

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
