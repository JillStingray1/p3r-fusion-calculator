use crate::persona::arcana;

use super::arcana::Arcana;
use super::skills::*;

pub struct Persona {
    pub name: String,
    pub arcana: Arcana,
    pub base_level: u8,
    pub special_recipe: bool,
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
                if (persona.arcana == fused_arcana)
                    && (persona.base_level >= fused_level)
                    && (persona.base_level < result_level)
                    && (!persona.special_recipe)
                {
                    result_level = persona.base_level;
                    result_persona = Some(persona)
                }
            }
        } else {
            let fused_level = (self.base_level + rhs.base_level) / 2 - 1;
            let mut result_level = 0;
            for persona in persona_list {
                if (persona.arcana == fused_arcana)
                    && (persona.base_level <= fused_level)
                    && (persona.base_level > result_level)
                    && (!persona.special_recipe)
                {
                    result_level = persona.base_level;
                    result_persona = Some(persona)
                }
            }
        }
        result_persona
    }

    /**
     * Returns a list of all possible forward fusions of a persona
     *
     * This returns the references of the other ingrdient and its
     * corresponding result persona
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
    /**
     * Finds all recipies which creates a persona
     */
    pub fn find_all_reverse_fusions<'a>(
        &self,
        persona_list: &'a Vec<Self>,
    ) -> Option<Vec<(&'a Self, &'a Self)>> {
        if self.special_recipe {
            None
        } else {
            let fusion_pairs = self.arcana.get_possible_combos();
            for (arcana_1, arcana_2) in fusion_pairs {}
            todo!()
        }
    }
}

#[cfg(test)]
mod persona_tests {
    use crate::persona;

    use super::*;

    #[test]
    fn test_fuse() {
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
        let persona_db = vec![orpheus, nekomata, omoikane];

        let result = persona_db[0].fuse(&persona_db[1], &persona_db);
        assert_eq!(result.unwrap().name, persona_db[2].name);
    }
}
