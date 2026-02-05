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

    /// Converts the title of the skills.md markdown files
    /// into skill types.
    pub fn from_title(str: String) -> Self {
        use SkillType::*;
        match str.as_str() {
            "Sla Skills" => Slash,
            "Str Skills" => Strike,
            "Pie Skills" => Pierce,
            "Fir Skills" => Fire,
            "Ice Skills" => Ice,
            "Win Skills" => Wind,
            "Ele Skills" => Elec,
            "Lig Skills" => Light,
            "Dar Skills" => Dark,
            "Alm Skills" => Almighty,
            "Rec Skills" => Healing,
            "Ail Skills" => Ailment,
            "Sup Skills" | "Spe Skills" => Support,
            _ => Passive,
        }
    }
}

#[derive(Debug)]
pub struct Skill {
    pub skill_type: SkillType,
    pub name: String,
    pub description: String,
}
