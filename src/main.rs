mod create_db;
mod persona;

use actix_files::Files;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web::Path,
};

use tera::{Context, Tera};

#[get("/persona_list")]
async fn persona_list(req: HttpRequest) -> impl Responder {
    let mut context = Context::new();
    let tera = Tera::one_off(include_str!("static/home.html"), &context, false)
        .expect("Unable to render template");
    HttpResponse::Ok().body(tera)
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
