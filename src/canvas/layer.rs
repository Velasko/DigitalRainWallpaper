use crate::structopt::StructOpt;

use crate::drop::drop::DropTrait;

#[derive(Debug)]
pub struct Layer<D> {
    speed: usize,
    drops: Vec<D>,
}

pub trait LayerTrait {
    // type DropType: DropTrait;
    fn new(speed: usize) -> Self;

    fn make_batch(layer_groups: u8, layer_speeds: Vec<usize>) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut batch = Vec::with_capacity(usize::from(layer_groups) * layer_speeds.len());
        for group in 0..layer_groups {
            for speed in &layer_speeds {
                batch.push(Self::new(*speed));
            }
        }
        batch
    }
}

impl<D> LayerTrait for Layer<D>
// where
//     D: DropTrait,
{
    // type DropType = D;

    fn new(speed: usize) -> Self {
        Self {
            speed,
            drops: Vec::new(),
        }
    }

    // fn make_drop(&mut self) {
    //     self.drops.push(D::default());
    // }
}
