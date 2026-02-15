use super::*;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

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
#[derive(Debug, Eq, PartialEq, Hash)]
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
#[derive(Debug)]
pub enum Recipes<'a> {
    Normal(Vec<(&'a Persona, &'a Persona)>),
    Special(Vec<&'a Persona>),
}

impl Persona {
    /// converts a json strucuture that stores the a persona in demon-data.json into a persona
    pub fn from_json(
        persona_name: &str,
        // skill_list: &HashMap<String, Skill>,
        persona_data: &Value,
        special_fusions: &HashMap<String, Vec<String>>,
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

        // gets the affinities as an array of chars (converting from string)
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

        // skills need to be stored with their learned level, the json stores
        // inital skills as 0.1, 0.2 and 0.3 to denote which skill slot they appear in
        for (skill_name, value) in skill_map {
            let learned_level = if value.as_f64().unwrap() < 1.0 {
                0
            } else {
                // theurgies are stored as learnt at level 100, the data stores it as 5207
                // so trying to convert from u64 will fail
                u8::try_from(value.as_u64().unwrap()).unwrap_or(100)
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
            name: persona_name.to_string(),
            arcana,
            base_level,
            affinities,
            special_recipe: special_fusions.contains_key(persona_name),
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
        use Arcana::*;
        let fused_arcana = self.arcana + rhs.arcana;
        if self.name == rhs.name
            || (self.arcana == Death && rhs.arcana == Aeon)
            || (self.arcana == Aeon && rhs.arcana == Death)
        {
            return None;
        }
        let mut result_persona: Option<&Self> = None;
        if fused_arcana != self.arcana {
            let fused_level = (self.base_level + rhs.base_level) / 2
                + (self.base_level + rhs.base_level) % 2;
            let mut result_level = 99;
            for persona in persona_list.values() {
                if (persona.arcana == fused_arcana)
                    && (persona.base_level >= fused_level)
                    && (persona.base_level < result_level)
                    && (persona.name != self.name)
                    && (!persona.special_recipe)
                {
                    result_level = persona.base_level;
                    result_persona = Some(persona)
                }
            }
        } else {
            let fused_level = (self.base_level + rhs.base_level) / 2
                + (self.base_level + rhs.base_level) % 2;
            let mut result_level = 0;
            for persona in persona_list.values() {
                if (persona.arcana == fused_arcana)
                    && (persona.base_level <= fused_level)
                    && (persona.base_level > result_level)
                    && (persona.name != self.name)
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
    pub fn find_all_forward_fusions<'a>(
        &self,
        persona_list: &'a HashMap<String, Self>,
    ) -> Vec<(&'a Self, &'a Self)> {
        let mut forward_fusions = vec![];
        for persona in persona_list.values() {
            if let Some(x) = self.fuse(persona, persona_list) {
                forward_fusions.push((persona, x))
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
        special_fusions: &HashMap<String, Vec<String>>,
    ) -> Recipes<'a> {
        use Recipes::*;
        if self.special_recipe {
            let names_in_recipe = special_fusions.get(&self.name).unwrap();
            let mut recipe = vec![];
            for name in names_in_recipe {
                recipe.push(persona_list.get(name).unwrap());
            }
            Special(recipe)
        } else {
            let mut reverse_fusions = HashSet::new();
            let fusion_pairs = self.arcana.get_possible_combos();
            for (arcana_1, arcana_2) in fusion_pairs {
                for persona_1 in persona_list.values() {
                    for persona_2 in persona_list.values() {
                        if (persona_1.arcana == arcana_1)
                            && (persona_2.arcana == arcana_2)
                            && (self.name != persona_1.name)
                            && (self.name != persona_2.name)
                            && (match persona_1.fuse(persona_2, persona_list) {
                                None => false,
                                Some(result) => result.name == self.name,
                            })
                        {
                            if persona_1.base_level < persona_2.base_level {
                                reverse_fusions.insert((persona_1, persona_2));
                            } else {
                                reverse_fusions.insert((persona_2, persona_1));
                            }
                        }
                    }
                }
            }
            Normal(reverse_fusions.into_iter().collect())
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
            .fuse(persona_db.get("Nekomata").unwrap(), &persona_db);
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
            .find_all_reverse_fusions(&persona_db, &HashMap::new());
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
