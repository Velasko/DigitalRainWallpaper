use crate::utils::charset_macro::make_subset;

use rand::seq::SliceRandom;

make_subset!(Katakana, "abcd");

pub enum CharSet {
    Katakana(Katakana),
}

pub trait CharacterSubset: Sized {
    fn parse(value: char) -> Option<Self>;
    fn as_char(&self) -> &char;

    fn get_charset() -> Vec<char>;

    fn pick_random() -> Self {
        let charset = Self::get_charset();
        let pick = charset
            .choose(&mut rand::thread_rng())
            .expect("Dataset must've had at least 1 value");
        Self::parse(*pick).expect("Picked from the acceptable strings")
    }
}
