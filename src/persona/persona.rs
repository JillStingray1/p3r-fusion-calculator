use std::rc::Rc;

use super::*;

/// The `Persona`` struct contains relevant details for individual personas
///
/// ### Fields
///
/// `name`: A `String` that contains the name of the persona
/// `arcana`: the `Arcana` that the persona belongs to
/// `special_recipe`: a `bool` that indicates if the persona is fused through special fusio
/// `affinities`: a persona's affinities, which indicates its weaknesses and resistances
/// `inheritance`: TODO maybe change this to a proper enum
/// `skills`: A `Vec` that contains a tuple, who's first element is a Skill that the persona learns, and
pub struct Persona {
    pub name: String,
    pub arcana: Arcana,
    pub base_level: u8,
    pub special_recipe: bool,
    pub affinities: [u8; 10],
    pub inheritance: Vec<SkillType>,
    pub skills: Vec<(Rc<Skill>, u8)>,
    pub cost: u32,
    pub stats: [u8; 5],
}

/// Enum for representing a persona's possible recipes
///
/// Either a persona is fusable through normal fusion, or it is fused through
/// special fusion. Normal fusion personas can have many possible recipies consisting
/// of 2 personas, whist special fusion can only have 1 fixed recipe consisting of any
/// number of personas
pub enum Recipes<'a> {
    Normal(Vec<(&'a Persona, &'a Persona)>),
    Special(Vec<&'a Persona>),
}

impl Persona {
    /// Gets the result of a fusion between 2 personae.
    ///
    /// This is determined by the using the fusion table to determine
    /// the resultant arcana of the fusion, and then finding the lowest
    /// level persona above the average level of the ingredients + 1
    ///
    ///
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

    /// Returns a list of all possible forward fusions of a persona
    ///
    /// This returns the references of the other ingrdient and its
    /// corresponding result persona
    ////
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
    /// Finds all recipes which creates the persona that this is called on
    ///
    /// This is done by searching all arcana pairs that result in a persona, and then
    /// finding personas belonging to each pair that fuses into the result.
    ///
    pub fn find_all_reverse_fusions<'a>(
        &'a self,
        persona_list: &'a Vec<Self>,
    ) -> Recipes {
        use Recipes::*;
        if self.special_recipe {
            Special(vec![])
        } else {
            let mut reverse_fusions = vec![];
            let fusion_pairs = self.arcana.get_possible_combos();
            for (arcana_1, arcana_2) in fusion_pairs {
                for persona_1 in persona_list {
                    for persona_2 in persona_list {
                        if (persona_1.arcana == arcana_1)
                            && (persona_2.arcana == arcana_2)
                            && (match persona_1.fuse(persona_2, persona_list) {
                                None => false,
                                Some(result) => result.name == self.name,
                            })
                        {
                            reverse_fusions.push((persona_1, persona_2));
                        }
                    }
                }
            }
            return Normal(reverse_fusions);
        }
    }
}

#[cfg(test)]
mod persona_tests {
    use core::panic;
    use std::result;

    use super::*;
    /// generates a small list of personae to test fusion with
    fn make_persona_list() -> Vec<Persona> {
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

    /// tests the fusion of 2 personas
    /// orpheus + nekomata = omoikane
    #[test]
    fn test_fuse() {
        let persona_db = make_persona_list();
        let result = persona_db[0].fuse(&persona_db[1], &persona_db);
        assert_eq!(result.unwrap().name, persona_db[2].name);
    }

    /// tests reverse fusion, omoikane can be fused from orpheus + nekomata
    #[test]
    fn test_reverse_fuse() {
        use Recipes::*;
        let persona_db = make_persona_list();
        let result = persona_db[2].find_all_reverse_fusions(&persona_db);
        match result {
            Normal(result) => assert_eq!(
                (&result[0].0.name, &result[0].1.name),
                (&persona_db[0].name, &persona_db[1].name)
            ),
            _ => panic!("Omoikane should be fused from normal recipes."),
        }
    }
}
