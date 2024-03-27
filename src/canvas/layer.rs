use crate::drop::drop::DropTrait;

pub struct Layer<D> {
    speed: usize,
    drops: Vec<D>,
}

pub trait LayerTrait {
    type DropType: DropTrait;
    fn new() -> Self;
    fn set_speed(&mut self, speed: usize);
}

impl<D> LayerTrait for Layer<D>
where
    D: DropTrait,
{
    type DropType = D;

    fn new() -> Self {
        Self {
            speed: 10,
            drops: Vec::new(),
        }
    }

    fn set_speed(&mut self, speed: usize) {
        self.speed = speed;
    }

    // fn make_drop(&mut self) {
    //     self.drops.push(D::default());
    // }
}
