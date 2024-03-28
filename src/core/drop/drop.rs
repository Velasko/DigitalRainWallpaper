use crate::utils::coordinates::Coord;

use crate::traits::drop::tail::TailTrait;
use crate::traits::drop::DropTrait;

pub struct Drop<T> {
    pos: Coord,
    height_spawn: Coord,
    height_death: Coord,
    brightness: u8,
    tail: T,
}

impl<T> DropTrait for Drop<T>
where
    T: TailTrait,
{
    type TailType = T;

    fn new(spawn: Coord, death: Coord, brightness: u8, tail_size: usize) -> Self {
        Self {
            pos: spawn.clone(),
            height_spawn: spawn,
            height_death: death,
            brightness: brightness,
            tail: T::new(tail_size),
        }
    }
}
