use crate::make_persona_db;
use crate::templates::*;

use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web::Path};

/// Renders the persona list route
///
/// This displays details for each persona
/// such as their names, arcana, stats and resistances
#[get("/persona_list")]
pub async fn persona_list(req: HttpRequest) -> impl Responder {
    let template = PersonaListTemplate {
        persona_list: &make_persona_db(),
    };
    HttpResponse::Ok().body(template.render().unwrap())
}

/// shows a list of skills, the personas that learn them, and skill card sources
#[get("/skill_list")]
pub async fn skills() -> impl Responder {
    HttpResponse::Ok().body("Todo")
}

/// gives the full details for a persona, including what skills they can inherit
/// and the skills they can learn
#[get("/persona/{persona}")]
pub async fn persona_details(path: Path<String>) -> impl Responder {
    let persona_name = path.into_inner();
    HttpResponse::Ok().body(format!("TODO: get details for {}", persona_name))
}
