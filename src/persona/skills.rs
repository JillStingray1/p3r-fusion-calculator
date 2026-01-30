use std::fmt::Display;

/// enum that contains all the skill elements
#[derive(Copy, Clone, Debug)]
pub enum SkillType {
    Slash,
    Strike,
    Pierce,
    Fire,
    Ice,
    Elec,
    Wind,
    Dark,
    Light,
    Healing,
    Support,
    Ailment,
    Passive,
    Almighty,
}

impl Display for SkillType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

///
impl SkillType {
    pub fn iterator() -> impl Iterator<Item = SkillType> {
        use SkillType::*;
        [
            Slash, Strike, Pierce, Fire, Ice, Elec, Wind, Dark, Light, Healing,
            Support, Ailment, Passive, Almighty,
        ]
        .iter()
        .copied()
    }
}

pub struct Skill {
    pub skill_type: SkillType,
    pub name: String,
    pub description: String,
}
