use std::collections::vec_deque::Iter;

use crate::utils::charset::CharacterSubset;

pub trait TailTrait {
    type CharacterSet: CharacterSubset;

    fn new(size: usize) -> Self;
    fn insert(&mut self, data: Self::CharacterSet);
    fn iter(&self) -> Iter<'_, Self::CharacterSet>;
}
