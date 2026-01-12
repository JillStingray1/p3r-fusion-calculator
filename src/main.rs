mod data;
mod persona;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use tera::{Context, Tera};

#[get("/example")]
async fn example() -> impl Responder {
    let mut context = Context::new();
    context.insert("variable", "wow");
    let tera =
        Tera::one_off(include_str!("templates/home.html"), &context, false)
            .expect("Unable to render template");
    HttpResponse::Ok().body(tera)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(example)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
