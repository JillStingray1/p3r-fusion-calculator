use super::*;
use serde_json::Value;
use std::collections::HashMap;

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
#[derive(Debug)]
pub struct Persona {
    pub name: String,
    pub arcana: Arcana,
    pub base_level: u8,
    pub special_recipe: bool,
    pub affinities: [char; 10],
    pub inheritance: Vec<SkillType>,
    pub skills: Vec<(String, u8)>,
    pub cost: u64,
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
    /// converts a json strucuture that stores the a persona in demon-data.json into a persona
    pub fn from_json(
        persona_name: &String,
        // skill_list: &HashMap<String, Skill>,
        persona_data: &Value,
    ) -> Self {
        let arcana = Arcana::from_str(
            persona_data
                .get("race")
                .expect("No race/arcana stored for persona")
                .as_str()
                .expect("Arcana is not stored as string"),
        );
        let base_level = u8::try_from(
            persona_data
                .get("lvl")
                .expect("No level found for persona")
                .as_u64()
                .expect("Value was not a number"),
        )
        .expect("Base level was too large");
        let affinities = persona_data
            .get("resists")
            .expect("No race/arcana stored for persona")
            .as_str()
            .expect("Arcana is not stored as string")
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let skill_map = persona_data
            .get("skills")
            .expect("Skills not formatted properly")
            .as_object()
            .expect("Skills not formatted properly");
        let mut skills = vec![];
        for (skill_name, value) in skill_map {
            let learned_level = if value.as_f64().unwrap() < 1.0 {
                0
            } else {
                match u8::try_from(value.as_u64().unwrap()) {
                    Ok(x) => x,
                    Err(_) => 100, // case for theurgies, which are learnt at 5271
                }
            };
            skills.push((skill_name.clone(), learned_level));
        }
        let mut stats = [0; 5];
        let mut stats_sum = 0;
        let stats_array = persona_data
            .get("stats")
            .expect("Stats not found")
            .as_array()
            .expect("stats not formated properly");
        if stats_array.len() != 5 {
            panic!("There are more or less than 5 stats provided");
        };
        for ind in 0..5 {
            let stat = stats_array[ind]
                .as_u64()
                .expect("stat is not a positive number");
            stats[ind] = u8::try_from(stat).expect("");
            stats_sum += stat
        }
        Persona {
            name: persona_name.clone(),
            arcana,
            base_level,
            affinities,
            special_recipe: false,
            inheritance: vec![],
            cost: 2000 + stats_sum.pow(2),
            skills,
            stats,
        }
    }

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
        persona_list: &'a HashMap<String, Self>,
    ) -> Option<&'a Self> {
        let fused_arcana = self.arcana + rhs.arcana;

        let mut result_persona: Option<&Self> = None;
        if fused_arcana != self.arcana {
            let fused_level = (self.base_level + rhs.base_level) / 2 + 1;
            let mut result_level = 99;
            for (_, persona) in persona_list {
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
            for (_, persona) in persona_list {
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
        persona_list: &'a HashMap<String, Self>,
    ) -> Vec<(&'a Self, &'a Self)> {
        let mut forward_fusions = vec![];
        for (_, persona) in persona_list {
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
        persona_list: &'a HashMap<String, Self>,
    ) -> Recipes {
        use Recipes::*;
        if self.special_recipe {
            // ! TODO: Get special Recipes
            Special(vec![])
        } else {
            let mut reverse_fusions = vec![];
            let fusion_pairs = self.arcana.get_possible_combos();
            for (arcana_1, arcana_2) in fusion_pairs {
                for (_, persona_1) in persona_list {
                    for (_, persona_2) in persona_list {
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

    use super::*;
    /// generates a small list of personae to test fusion with
    fn make_persona_list() -> HashMap<String, Persona> {
        use Arcana::*;
        let orpheus = Persona {
            name: String::from("Orpheus"),
            arcana: Fool,
            base_level: 1,
            special_recipe: false,
            affinities: "---s-w--w-"
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
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
            affinities: "---s-w--w-"
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
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
            affinities: "---s-w--w-"
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            inheritance: vec![],
            skills: vec![],
            cost: 0,
            stats: [0, 0, 0, 0, 0],
        };
        HashMap::from([
            (String::from("Orpheus"), orpheus),
            (String::from("Nekomata"), nekomata),
            (String::from("Omoikane"), omoikane),
        ])
    }

    /// tests the fusion of 2 personas
    /// orpheus + nekomata = omoikane
    #[test]
    fn test_fuse() {
        let persona_db = make_persona_list();
        let result = persona_db
            .get("Orpheus")
            .unwrap()
            .fuse(&persona_db.get("Nekomata").unwrap(), &persona_db);
        assert_eq!(
            result.unwrap().name,
            persona_db.get("Omoikane").unwrap().name
        );
    }

    /// tests reverse fusion, omoikane can be fused from orpheus + nekomata
    #[test]
    fn test_reverse_fuse() {
        use Recipes::*;
        let persona_db = make_persona_list();
        let result = persona_db
            .get("Omoikane")
            .unwrap()
            .find_all_reverse_fusions(&persona_db);
        match result {
            Normal(result) => assert_eq!(
                (&result[0].0.name, &result[0].1.name),
                (
                    &persona_db.get("Orpheus").unwrap().name,
                    &persona_db.get("Nekomata").unwrap().name
                )
            ),
            _ => panic!("Omoikane should be fused from normal recipes."),
        }
    }
}
