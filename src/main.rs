mod app;
mod create_db;
mod persona_data;
mod templates;
use actix_files::Files;
use actix_web::{
    App, HttpServer,
    web::{self, Data},
};
use app::*;
use create_db::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    make_special_fusions();
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(AppData {
                persona_list: make_persona_list(),
                skill_list: make_skill_list(),
            }))
            .service(Files::new("/static", "src/static/.").show_files_listing())
            .service(persona_list)
            .service(skills)
            .service(persona_details)
            .service(web::redirect("/", "/persona_list"))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
