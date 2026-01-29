use crate::persona::{
    arcana::Arcana,
    persona::Persona,
    skills::{Skill, SkillType},
};

pub fn make_persona_db() -> Vec<Persona> {
    // for now this is a stub function that gives an example of what the persona_list will look like
    use Arcana::*;
    let orpheus = Persona {
        name: String::from("Orpheus"),
        arcana: Fool,
        base_level: 1,
        special_recipe: false,
        affinities: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        inheritance: vec![],
        skills: vec![(
            Skill {
                skill_type: SkillType::Fire,
                name: String::from("Agi"),
                description: String::from("Weak fire damage to one enemy"),
            },
            0,
        )],
        cost: 0,
        stats: [2, 2, 2, 2, 2],
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
