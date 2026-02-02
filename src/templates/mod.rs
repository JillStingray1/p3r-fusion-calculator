use crate::persona::{persona::Persona, skills::SkillType};
pub use askama::Template;

#[derive(Template)]
#[template(path = "persona_list.html")]
/// Template for the persona list page, requires
/// a list of personas, and will display relevant information
/// such as stats, resistances, name and arcana in a sorted table
pub struct PersonaListTemplate<'a> {
    pub persona_list: &'a Vec<Persona>,
}

#[derive(Template)]
#[template(path = "persona.html")]
pub struct PersonaTemplate<'a> {
    pub persona: &'a Persona,
    pub forward_fusions: Vec<(&'a Persona, &'a Persona)>,
    pub special_fusion: Option<Vec<&'a Persona>>,
}
