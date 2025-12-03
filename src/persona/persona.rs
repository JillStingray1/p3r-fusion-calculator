use super::arcana::Arcana;
use super::skills::*;

pub struct Persona {
    arcana: Arcana,
    base_level: i8,
    affinites: [i8; 9],
    inheritance: Skill,
}
