use crate::parsers::ArgParser;
use crate::structopt::StructOpt;

use crate::traits::canvas::CanvasTrait;
use crate::utils::{color::Color, coordinates::Coord};
use crate::ARGS;

use super::traits::{tail::TailTrait, DropTrait};

#[derive(Debug)]
pub struct Drop<T> {
    pos: Coord,
    height_spawn: Coord,
    height_death: Coord,
    brightness: u8,
    tail: T,

    speed: &'static usize,
    color_gradient: &'static Color,
}

impl<T> DropTrait for Drop<T>
where
    T: TailTrait,
{
    type TailType = T;

    fn fall(&mut self) {
        self.pos.y = self.pos.y + self.speed;
    }

    fn make_random(canvas: &impl CanvasTrait) -> Self {
        let [x_max, y_max] = canvas.get_size().into();

        let x_coord = canvas.random(0, x_max);
        let y_spawn = canvas.exp_dist(0.5, y_max);
        let y_death = y_max - canvas.exp_dist(0.5, y_spawn);

        let drop_options = &ARGS.drop;

        Self {
            pos: Coord {
                x: x_coord,
                y: y_spawn,
            },
            height_spawn: Coord {
                x: x_coord,
                y: y_spawn,
            },
            height_death: Coord {
                x: x_coord,
                y: y_death,
            },
            brightness: 255,
            tail: T::new(3),

            speed: canvas.random_pick(drop_options.drop_speeds()),
            color_gradient: canvas.random_pick(drop_options.drop_colors()),
        }
    }
}
