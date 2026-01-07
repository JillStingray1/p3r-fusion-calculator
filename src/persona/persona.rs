use crate::persona;

use super::arcana::Arcana;
use super::skills::*;

pub struct Persona {
    pub name: String,
    pub arcana: Arcana,
    pub base_level: u8,
    pub affinities: [u8; 10],
    pub inheritance: Vec<SkillType>,
    pub skills: Vec<(Skill, u8)>,
    pub cost: u32,
    pub stats: [u8; 5],
}

impl Persona {
    /**
     * Gets the result of a fusion between 2 personae.
     *
     * This is determined by the using the fusion table to determine
     * the resultant arcana of the fusion, and then finding the lowest
     * level persona above the average level of the ingredients + 1
     *
     */
    pub fn fuse<'a>(
        &self,
        rhs: &'a Self,
        persona_list: &'a Vec<Self>,
    ) -> Option<&'a Self> {
        let fused_arcana = self.arcana + rhs.arcana;

        let mut result_persona: Option<&Self> = None;
        if fused_arcana != self.arcana {
            let fused_level = (self.base_level + rhs.base_level) / 2 + 1;
            let mut result_level = 99;
            for persona in persona_list {
                println!("{}", persona.name);
                if (persona.arcana == fused_arcana)
                    && (persona.base_level >= fused_level)
                    && (persona.base_level < result_level)
                {
                    println!("{:?}", fused_level);
                    result_level = persona.base_level;
                    result_persona = Some(persona)
                }
            }
        } else {
            let fused_level = (self.base_level + rhs.base_level) / 2 - 1;
            for persona in persona_list {
                let mut result_level = 0;
                if (persona.arcana == fused_arcana)
                    && (persona.base_level <= fused_level)
                    && (persona.base_level > result_level)
                {
                    result_level = persona.base_level;
                    result_persona = Some(persona)
                }
            }
        }
        result_persona
    }

    /**
     * Returns a list of all possible forward fusions of some persona in a fusion
     * database
     *
     *
     */
    pub fn find_all_forward_fusions<'a>(
        &self,
        persona_list: &'a Vec<Self>,
    ) -> Vec<(&'a Self, &'a Self)> {
        let mut forward_fusions = vec![];
        for persona in persona_list {
            match self.fuse(persona, persona_list) {
                Some(x) => forward_fusions.push((persona, x)),
                None => (),
            }
        }
        forward_fusions
    }
}
