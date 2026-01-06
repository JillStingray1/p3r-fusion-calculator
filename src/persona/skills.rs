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
    skill_type: SkillType,
    power: u32,
    accuracy: u8,
    crit_rate: u8,
    description: String,
}
