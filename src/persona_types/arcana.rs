use std::ops::{Add, Sub};

#[derive(Debug)]
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


        impl Sub for Arcana {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                use Arcana::*;
                match (self, rhs) {
                    $(
                        ($arc_3, $arc_2) => $arc_1,
                        ($arc_3, $arc_1) => $arc_2,
                    )*
                    (x, _) => x
                }
            }
        }
    };
}

define_arcana_ops! {
    (Fool, Fool) => Fool,
    (Fool, Magician) => Hermit,
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
}
