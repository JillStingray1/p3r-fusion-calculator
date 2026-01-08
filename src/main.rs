mod data;
mod persona;

fn main() {
    use persona::arcana::Arcana::*;
    println!("result: {:?}", Empress - Priestess);
    let orpheus = persona::persona::Persona {
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
    let nekomata = persona::persona::Persona {
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
    let omoikane = persona::persona::Persona {
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
    println!("{}", result.unwrap().name);
    let result_1 = persona_db[0].find_all_forward_fusions(&persona_db);
    for (ingredient, result) in result_1 {
        println!("Orpheus + {} => {}", ingredient.name, result.name);
    }
}
