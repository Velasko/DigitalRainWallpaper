use crate::utils::coordinates::Coord;

use crate::parsers::canvas::CanvasParser;
use crate::traits::drop::DropTrait;

pub trait CanvasTrait {
    type DropType: DropTrait;

    fn get_size(&self) -> Coord;

    fn parse(parser: &CanvasParser) -> Self;
    fn make_drop(&mut self);

    fn update(&mut self);

    fn random(&self, start: usize, end: usize) -> usize;

    fn random_chance(&self) -> f64 {
        (self.random(0, usize::MAX) as f64) / (usize::MAX as f64)
    }

    fn exp_dist(&self, prob: f64, max_value: usize) -> usize {
        let rng = self.random_chance();
        std::cmp::min(rng.log(prob).trunc() as usize, max_value)
    }

    fn random_pick<'a, T>(&self, options: &'a [T]) -> &'a T {
        let pos = self.random(0, options.len());
        &options[pos]
    }
}
