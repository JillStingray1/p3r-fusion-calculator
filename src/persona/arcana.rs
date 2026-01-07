use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arcana {
    Fool,
    Magician,
    Priestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    Fortune,
    Strength,
    HangedMan,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    Aeon,
}

impl Arcana {
    pub fn iterator() -> impl Iterator<Item = Arcana> {
        use Arcana::*;
        [
            Fool, Magician, Priestess, Empress, Emperor, Hierophant, Lovers,
            Chariot, Justice, Hermit, Fortune, Strength, HangedMan, Death,
            Temperance, Devil, Tower, Star, Moon, Sun, Judgement, Aeon,
        ]
        .iter()
        .copied()
    }
}

macro_rules! define_arcana_ops {
    ($(($arc_1:ident, $arc_2:ident) => $arc_3:ident),* $(,)?) => {


        impl Add for Arcana {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                use Arcana::*;
                match (self, rhs) {
                    $(($arc_1, $arc_2) | ($arc_2, $arc_1) => $arc_3,)*
                    (x, _) => x
                }
            }
        }
    };
}

impl Sub for Arcana {
    type Output = Vec<Arcana>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = vec![];
        for arcana in Arcana::iterator() {
            if arcana + rhs == self {
                result.push(arcana);
            }
        }
        result
    }
}

define_arcana_ops! {
    // Fool
    (Fool, Magician) => Hierophant,
    (Fool, Priestess) => Magician,
    (Fool, Empress) => Star,
    (Fool, Emperor) => Temperance,
    (Fool, Hierophant) => HangedMan,
    (Fool, Lovers) => Justice,
    (Fool, Chariot) => Emperor,
    (Fool, Justice) => Lovers,
    (Fool, Hermit) => Priestess,
    (Fool, Fortune) => Strength,
    (Fool, Strength) => Death,
    (Fool, HangedMan) => Devil,
    (Fool, Death) => Fortune,
    (Fool, Temperance) => Chariot,
    (Fool, Devil) => Hermit,
    (Fool, Tower) => Moon,
    (Fool, Star) => Devil,
    (Fool, Moon) => Emperor,
    (Fool, Sun) => Judgement,
    (Fool, Judgement) => Aeon,
    (Fool, Aeon) => Death,

    // Magician
    (Magician, Priestess) => Justice,
    (Magician, Empress) => HangedMan,
    (Magician, Emperor) => Lovers,
    (Magician, Hierophant) => Hermit,
    (Magician, Lovers) => Chariot,
    (Magician, Chariot) => Devil,
    (Magician, Justice) => Hierophant,
    (Magician, Hermit) => Moon,
    (Magician, Fortune) => Lovers,
    (Magician, Strength) => Emperor,
    (Magician, HangedMan) => Fool,
    (Magician, Death) => Priestess,
    (Magician, Temperance) => Justice,
    (Magician, Devil) => Temperance,
    (Magician, Tower) => Chariot,
    (Magician, Star) => Strength,
    (Magician, Moon) => Strength,
    (Magician, Sun) => Empress,
    (Magician, Judgement) => Star,
    (Magician, Aeon) => Sun,

    // Priestess
    (Priestess, Empress) => Temperance,
    (Priestess, Emperor) => Justice,
    (Priestess, Hierophant) => Lovers,
    (Priestess, Lovers) => Magician,
    (Priestess, Chariot) => Fool,
    (Priestess, Justice) => Lovers,
    (Priestess, Hermit) => Strength,
    (Priestess, Fortune) => HangedMan,
    (Priestess, Strength) => Moon,
    (Priestess, HangedMan) => Hierophant,
    (Priestess, Death) => Justice,
    (Priestess, Temperance) => Fortune,
    (Priestess, Devil) => Emperor,
    (Priestess, Tower) => Empress,
    (Priestess, Star) => Emperor,
    (Priestess, Moon) => Star,
    (Priestess, Sun) => Hierophant,
    (Priestess, Judgement) => HangedMan,
    (Priestess, Aeon) => Empress,

    // Empress
    (Empress, Emperor) => Chariot,
    (Empress, Hierophant) => Tower,
    (Empress, Lovers) => Moon,
    (Empress, Chariot) => Hermit,
    (Empress, Justice) => Emperor,
    (Empress, Hermit) => Sun,
    (Empress, Fortune) => Strength,
    (Empress, Strength) => Fool,
    (Empress, HangedMan) => Star,
    (Empress, Death) => Lovers,
    (Empress, Temperance) => Hierophant,
    (Empress, Devil) => Tower,
    (Empress, Tower) => Devil,
    (Empress, Star) => Priestess,
    (Empress, Moon) => Aeon,
    (Empress, Sun) => Emperor,
    (Empress, Judgement) => Lovers,
    (Empress, Aeon) => Priestess,
}
