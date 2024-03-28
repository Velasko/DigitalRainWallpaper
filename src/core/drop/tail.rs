use std::collections::{vec_deque::Iter, VecDeque};

use crate::traits::drop::tail::TailTrait;
use crate::utils::charset::CharacterSubset;

pub struct Tail<C> {
    data: VecDeque<C>,
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
