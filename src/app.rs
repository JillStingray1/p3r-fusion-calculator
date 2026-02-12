use crate::persona_data::{Persona, Skill};
use crate::templates::*;
use actix_web::web;
use actix_web::{HttpResponse, Responder, get, web::Path};
use std::collections::HashMap;

pub struct AppData {
    pub persona_list: HashMap<String, Persona>,
    pub skill_list: HashMap<String, Skill>,
}

/// Renders the persona list route
///
/// This displays details for each persona
/// such as their names, arcana, stats and resistances
#[get("/persona_list")]
pub async fn persona_list(data: web::Data<AppData>) -> impl Responder {
    let template = PersonaListTemplate {
        persona_list: data.persona_list.values().collect(),
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
    let searched_persona: Option<&Persona> =
        data.persona_list.get(&persona_name);
    match searched_persona {
        Some(found_persona) => {
            let mut skill_list = vec![];
            for (skill_name, learned_level) in &found_persona.skills {
                skill_list.push((
                    data.skill_list.get(skill_name).unwrap(),
                    learned_level,
                ));
            }
            skill_list.sort_by(|x, y| x.1.cmp(y.1));
            let template = PersonaTemplate {
                persona: found_persona,
                forward_fusions: found_persona
                    .find_all_forward_fusions(&data.persona_list),
                reverse_fusions: found_persona
                    .find_all_reverse_fusions(&data.persona_list),
                skill_list,
            };
            HttpResponse::Ok().body(template.render().unwrap())
        }
        None => HttpResponse::BadRequest().body("Persona does not exist"),
    }
}
