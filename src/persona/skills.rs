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

pub struct Skill {
    pub skill_type: SkillType,
    pub name: String,
    pub description: String,
}
