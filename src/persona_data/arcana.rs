use std::collections::HashSet;
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
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
    /// Returns an iterator over the arcana in order that
    /// they are in persona
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

    /// Converts a Arcana enum into a number corresponding to their position in persona tarot
    /// (for sorting purposes)
    pub fn as_usize(self) -> usize {
        self as usize
    }

    /// Converts &str into Arcana
    pub fn from_str(str: &str) -> Self {
        use Arcana::*;
        match str {
            "Fool" => Fool,
            "Magician" => Magician,
            "Priestess" => Priestess,
            "Empress" => Empress,
            "Emperor" => Emperor,
            "Hierophant" => Hierophant,
            "Lovers" => Lovers,
            "Chariot" => Chariot,
            "Justice" => Justice,
            "Hermit" => Hermit,
            "Fortune" => Fortune,
            "Strength" => Strength,
            "Hanged" => HangedMan,
            "Death" => Death,
            "Temperance" => Temperance,
            "Devil" => Devil,
            "Tower" => Tower,
            "Star" => Star,
            "Moon" => Moon,
            "Sun" => Sun,
            "Judgement" => Judgement,
            _ => Aeon,
        }
    }

    /// Gets the possible arcana combos in fusion that results in
    /// the arcana the method is called on
    pub fn get_possible_combos(self) -> HashSet<(Arcana, Arcana)> {
        let mut arcana_combos = HashSet::new();
        for arcana_1 in Arcana::iterator() {
            let possible_arcanas = self - arcana_1;
            for arcana_2 in possible_arcanas {
                arcana_combos.insert(if arcana_1 < arcana_2 {
                    (arcana_1, arcana_2)
                } else {
                    (arcana_2, arcana_1)
                });
            }
        }
        arcana_combos
    }
}

impl Display for Arcana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Implementation for the addition off arcanae in fusion
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

    // Emperor
    (Emperor, Hierophant) => Strength,
    (Emperor, Lovers) => Chariot,
    (Emperor, Chariot) => Devil,
    (Emperor, Justice) => HangedMan,
    (Emperor, Hermit) => Hierophant,
    (Emperor, Fortune) => Star,
    (Emperor, Strength) => Magician,
    (Emperor, HangedMan) => Death,
    (Emperor, Death) => Hermit,
    (Emperor, Temperance) => Star,
    (Emperor, Devil) => Moon,
    (Emperor, Tower) => Strength,
    (Emperor, Star) => Hierophant,
    (Emperor, Moon) => Lovers,
    (Emperor, Sun) => Temperance,
    (Emperor, Judgement) => Sun,
    (Emperor, Aeon) => Fortune,

    // Hierophant
    (Hierophant, Lovers) => Magician,
    (Hierophant, Chariot) => Justice,
    (Hierophant, Justice) => Fool,
    (Hierophant, Hermit) => Chariot,
    (Hierophant, Fortune) => Moon,
    (Hierophant, Strength) => Fortune,
    (Hierophant, HangedMan) => Strength,
    (Hierophant, Death) => Fortune,
    (Hierophant, Temperance) => Hermit,
    (Hierophant, Devil) => Priestess,
    (Hierophant, Tower) => Temperance,
    (Hierophant, Star) => Moon,
    (Hierophant, Moon) => Magician,
    (Hierophant, Sun) => Tower,
    (Hierophant, Judgement) => Emperor,
    (Hierophant, Aeon) => Sun,

    // Lovers
    (Lovers, Chariot) => Priestess,
    (Lovers, Justice) => Emperor,
    (Lovers, Hermit) => Fool,
    (Lovers, Fortune) => Temperance,
    (Lovers, Strength) => Hermit,
    (Lovers, HangedMan) => Justice,
    (Lovers, Death) => HangedMan,
    (Lovers, Temperance) => Death,
    (Lovers, Devil) => Star,
    (Lovers, Tower) => Sun,
    (Lovers, Star) => Death,
    (Lovers, Moon) => Empress,
    (Lovers, Sun) => Devil,
    (Lovers, Judgement) => Moon,
    (Lovers, Aeon) => Tower,

    // Chariot
    (Chariot, Justice) => Magician,
    (Chariot, Hermit) => Lovers,
    (Chariot, Fortune) => Priestess,
    (Chariot, Strength) => Temperance,
    (Chariot, HangedMan) => Strength,
    (Chariot, Death) => Hierophant,
    (Chariot, Temperance) => Hermit,
    (Chariot, Devil) => HangedMan,
    (Chariot, Tower) => Star,
    (Chariot, Star) => Fortune,
    (Chariot, Moon) => Temperance,
    (Chariot, Sun) => Strength,
    (Chariot, Judgement) => Empress,
    (Chariot, Aeon) => Hermit,

    // Justice
    (Justice, Hermit) => Magician,
    (Justice, Fortune) => HangedMan,
    (Justice, Strength) => Star,
    (Justice, HangedMan) => Priestess,
    (Justice, Death) => Hermit,
    (Justice, Temperance) => Moon,
    (Justice, Devil) => Temperance,
    (Justice, Tower) => Sun,
    (Justice, Star) => Hermit,
    (Justice, Moon) => Temperance,
    (Justice, Sun) => Magician,
    (Justice, Judgement) => Fool,
    (Justice, Aeon) => Judgement,

    // Hermit
    (Hermit, Fortune) => Justice,
    (Hermit, Strength) => Emperor,
    (Hermit, HangedMan) => Temperance,
    (Hermit, Death) => Chariot,
    (Hermit, Temperance) => Magician,
    (Hermit, Devil) => Strength,
    (Hermit, Tower) => Emperor,
    (Hermit, Star) => Fool,
    (Hermit, Moon) => Hierophant,
    (Hermit, Sun) => Star,
    (Hermit, Judgement) => Temperance,
    (Hermit, Aeon) => Devil,

    // Fortune
    (Fortune, Strength) => Sun,
    (Fortune, HangedMan) => Magician,
    (Fortune, Death) => Star,
    (Fortune, Temperance) => Tower,
    (Fortune, Devil) => Empress,
    (Fortune, Tower) => Aeon,
    (Fortune, Star) => Magician,
    (Fortune, Moon) => Death,
    (Fortune, Sun) => Judgement,
    (Fortune, Judgement) => Sun,
    (Fortune, Aeon) => Moon,

    // Strength
    (Strength, HangedMan) => Chariot,
    (Strength, Death) => Empress,
    (Strength, Temperance) => Moon,
    (Strength, Devil) => Lovers,
    (Strength, Tower) => HangedMan,
    (Strength, Star) => Priestess,
    (Strength, Moon) => Devil,
    (Strength, Sun) => Lovers,
    (Strength, Judgement) => Devil,
    (Strength, Aeon) => Fool,

    // HangedMan
    (HangedMan, Death) => Strength,
    (HangedMan, Temperance) => Hierophant,
    (HangedMan, Devil) => Priestess,
    (HangedMan, Tower) => Death,
    (HangedMan, Star) => Empress,
    (HangedMan, Moon) => Chariot,
    (HangedMan, Sun) => Aeon,
    (HangedMan, Judgement) => Tower,
    (HangedMan, Aeon) => Death,

    // Death
    (Death, Temperance) => Devil,
    (Death, Devil) => Tower,
    (Death, Tower) => Aeon,
    (Death, Star) => Sun,
    (Death, Moon) => HangedMan,
    (Death, Sun) => Justice,
    (Death, Judgement) => Devil,

    // Temperance
    (Temperance, Devil) => Fool,
    (Temperance, Tower) => Devil,
    (Temperance, Star) => Fortune,
    (Temperance, Moon) => Priestess,
    (Temperance, Sun) => Chariot,
    (Temperance, Judgement) => Empress,
    (Temperance, Aeon) => Justice,

    // Devil
    (Devil, Tower) => Judgement,
    (Devil, Star) => Justice,
    (Devil, Moon) => Fool,
    (Devil, Sun) => Death,
    (Devil, Judgement) => Death,
    (Devil, Aeon) => Star,

    // Tower
    (Tower, Star) => Judgement,
    (Tower, Moon) => Fortune,
    (Tower, Sun) => Hierophant,
    (Tower, Judgement) => Aeon,
    (Tower, Aeon) => Sun,

    // Star
    (Star, Moon) => Sun,
    (Star, Sun) => Justice,
    (Star, Judgement) => Tower,
    (Star, Aeon) => Judgement,

    // Moon
    (Moon, Sun) => Tower,
    (Moon, Judgement) => Fortune,
    (Moon, Aeon) => Judgement,

    // Sun
    (Sun, Judgement) => Aeon,
    (Sun, Aeon) => Empress,

    // Judgement
    (Judgement, Aeon) => Fool,
}
