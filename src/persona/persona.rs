use std::ops::Add;

use super::arcana::Arcana;
use super::skills::*;

pub struct Persona {
    name: String,
    arcana: Arcana,
    base_level: u8,
    affinites: [u8; 10],
    inheritance: Vec<SkillType>,
    skills: Vec<(Skill, u8)>,
    cost: u32,
    stats: [u8; 5],
}

impl Add for Persona {
    type Output = Persona;

    fn add(self, rhs: Self) -> Self::Output {
        let target = self.arcana + rhs.arcana;
        todo!()
    }
}
