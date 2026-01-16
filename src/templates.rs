use crate::persona::persona::Persona;
pub use askama::Template;

#[derive(Template)]
#[template(path = "persona_list.html")]
pub struct PersonaListTemplate<'a> {
    pub persona_list: &'a Vec<Persona>,
}
