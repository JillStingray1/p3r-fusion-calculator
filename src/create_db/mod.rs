use crate::persona::{arcana::Arcana, persona::Persona};

pub fn make_persona_db() -> Vec<Persona> {
    use Arcana::*;
    let orpheus = Persona {
        name: String::from("Orpheus"),
        arcana: Fool,
        base_level: 1,
        special_recipe: false,
        affinities: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        inheritance: vec![],
        skills: vec![],
        cost: 0,
        stats: [0, 0, 0, 0, 0],
    };
    let nekomata = Persona {
        name: String::from("Nekomata"),
        arcana: Magician,
        base_level: 3,
        special_recipe: false,
        affinities: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        inheritance: vec![],
        skills: vec![],
        cost: 0,
        stats: [0, 0, 0, 0, 0],
    };
    let omoikane = Persona {
        name: String::from("Omoikane"),
        arcana: Hierophant,
        base_level: 7,
        special_recipe: false,
        affinities: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        inheritance: vec![],
        skills: vec![],
        cost: 0,
        stats: [0, 0, 0, 0, 0],
    };
    vec![orpheus, nekomata, omoikane]
}
