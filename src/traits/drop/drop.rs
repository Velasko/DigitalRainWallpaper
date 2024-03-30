extern crate structopt;
use structopt::StructOpt;

use crate::utils::coordinates::Coord;

use super::tail::*;
use crate::parsers::drop::DropOptions;
use crate::traits::canvas::CanvasTrait;

pub trait DropTrait {
    type TailType: TailTrait;

    fn fall(&mut self);
    fn make_random(canvas: &impl CanvasTrait) -> Self;
}
