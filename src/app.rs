use crate::make_persona_db;
use crate::persona::{Persona, Skill};
use crate::templates::*;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web::Path};

pub struct AppData {
    pub persona_list: Vec<Persona>,
    pub skill_list: Vec<Skill>,
}

/// Renders the persona list route
///
/// This displays details for each persona
/// such as their names, arcana, stats and resistances
#[get("/persona_list")]
pub async fn persona_list(
    req: HttpRequest,
    data: web::Data<AppData>,
) -> impl Responder {
    let template = PersonaListTemplate {
        persona_list: &data.persona_list,
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
pub async fn persona_details(
    path: Path<String>,
    data: web::Data<AppData>,
) -> impl Responder {
    let persona_name = path.into_inner();
    let mut target_persona: Option<&Persona> = None;
    for persona in &data.persona_list {
        if persona.name == persona_name {
            target_persona = Some(persona);
        }
    }
    let template = PersonaTemplate {
        persona: target_persona.unwrap(),
        forward_fusions: target_persona
            .unwrap()
            .find_all_forward_fusions(&data.persona_list),
        special_fusion: None,
    };
    HttpResponse::Ok().body(template.render().unwrap())
}
