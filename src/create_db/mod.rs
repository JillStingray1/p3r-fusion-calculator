use crate::persona::{self, Arcana, Persona, Skill, SkillType};
use serde_json::{Value, from_str};
use std::{collections::HashMap, fs::File, io::Read, rc::Rc};

pub fn make_persona_db() -> Vec<Persona> {
    // for now this is a stub function that gives an example of what the persona_list will look like
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
    vec![orpheus, nekomata, omoikane]
}

fn split_string(string: String, splitter: &str) -> Vec<String> {
    string
        .trim_start_matches(splitter)
        .trim()
        .split(splitter)
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>()
}

/// Converts a file located at ./persona_data/skills.md into a hashmap
/// which contains the name of the skill as the key to a
/// Skill struct corresponding to that.
pub fn make_skill_list() -> HashMap<String, Skill> {
    let mut skill_list = HashMap::new();
    let mut file =
        File::open("persona_data/skills.md").expect("No skill data file found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the skill file provided");
    drop(file);
    for skill_table in split_string(contents, "####") {
        let mut rows = split_string(skill_table, "\n").into_iter();
        let title: String = rows.next().expect("File is not in correct format");
        let header = rows.next().expect("File is not in correct format");
        let skill_type = SkillType::from_title(title);
        let description_index = split_string(header, "|")
            .iter()
            .position(|s| s == "Description")
            .expect("Can't find description column of skills table");
        rows.next();
        for row in rows {
            let entries = split_string(row, "|");
            skill_list.insert(
                entries[1].clone(),
                Skill {
                    skill_type,
                    name: entries[1].clone(),
                    description: entries[description_index].clone(),
                },
            );
        }
    }
    skill_list
}

pub fn make_persona_list() -> HashMap<String, Persona> {
    let mut persona_list = HashMap::new();
    let mut contents = String::new();
    File::open("persona_data/demon-data.json")
        .expect("No file found at persona_data/demon-data.json.")
        .read_to_string(&mut contents)
        .expect("Failed to read demon-data.json file");
    let json: Value = from_str(&contents)
        .expect("persona_data/demon-data.json is not properly formatted");
    let map = json.as_object().expect("failed to create map from json");
    for (persona_name, persona) in map {
        persona_list.insert(
            persona_name.clone(),
            Persona::from_json(persona_name, persona),
        );
    }
    persona_list
}
