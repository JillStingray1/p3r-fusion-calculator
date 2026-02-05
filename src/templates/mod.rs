use crate::persona::{
    Recipes::{self, *},
    Skill,
    persona::Persona,
};
pub use askama::Template;

#[derive(Template)]
#[template(path = "persona_list.html")]
/// Template for the persona list page, requires
/// a list of personas, and will display relevant information
/// such as stats, resistances, name and arcana in a sorted table
pub struct PersonaListTemplate<'a> {
    pub persona_list: Vec<&'a Persona>,
}

#[derive(Template)]
#[template(path = "persona.html")]
pub struct PersonaTemplate<'a> {
    pub persona: &'a Persona,
    pub forward_fusions: Vec<(&'a Persona, &'a Persona)>,
    pub reverse_fusions: Recipes<'a>,
    pub skill_list: Vec<(&'a Skill, &'a u8)>,
}
