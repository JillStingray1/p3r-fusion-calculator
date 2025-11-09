mod persona_types;

fn main() {
    use persona_types::arcana::Arcana::*;
    println!("Hello, world! {:?}", (Judgement - Sun));
}
