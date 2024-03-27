use std::collections::{vec_deque::Iter, VecDeque};

use crate::common::charset::CharacterSubset;

pub struct Tail<C> {
    data: VecDeque<C>,
}

pub trait TailTrait {
    type CharacterSet: CharacterSubset;
    fn new(size: usize) -> Self;
    fn insert(&mut self, data: Self::CharacterSet);
    fn iter(&self) -> Iter<'_, Self::CharacterSet>;
}

impl<C> TailTrait for Tail<C>
where
    C: CharacterSubset,
{
    type CharacterSet = C;

    fn new(size: usize) -> Self {
        let mut data_vec = VecDeque::with_capacity(size);
        for _ in 0..size {
            data_vec.push_front(C::pick_random());
        }
        Tail { data: data_vec }
    }

    fn insert(&mut self, data: C) {
        self.data.pop_back();
        self.data.push_front(data);
    }

    fn iter(&self) -> Iter<'_, C> {
        self.data.iter()
    }
}
