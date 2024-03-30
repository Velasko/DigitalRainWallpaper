use crate::parsers::canvas::CanvasParser;
use crate::traits::canvas::CanvasTrait;
use crate::traits::drop::*;
use crate::utils::color::Color;
use crate::utils::coordinates::Coord;
use crate::RNG;

#[derive(Debug)]
pub struct Canvas<D> {
    size: [usize; 2],
    drops: Vec<D>,
    bg_color: Color,
    drop_spawn_chance: f64,
}

impl<D> CanvasTrait for Canvas<D>
where
    D: DropTrait,
{
    type DropType = D;

    fn get_size(&self) -> Coord {
        Coord::from(self.size)
    }

    fn parse(parser: &CanvasParser) -> Self {
        Self {
            size: parser.get_size(),
            bg_color: parser.get_bg_color(),
            drops: Vec::new(),
            drop_spawn_chance: parser.get_drop_spawn_chance(),
        }
    }

    fn update(&mut self) {
        for drop in &mut self.drops {
            drop.fall();
        }

        while RNG.random_chance() < self.drop_spawn_chance {
            self.make_drop();
        }
    }

    fn make_drop(&mut self) {
        let drop = Self::DropType::make_random(self);
        self.drops.push(drop);
    }
}
