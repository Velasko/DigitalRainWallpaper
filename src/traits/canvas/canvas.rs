use crate::utils::coordinates::Coord;

use crate::parsers::canvas::CanvasParser;
use crate::traits::drop::DropTrait;

pub trait CanvasTrait {
    type DropType: DropTrait;

    fn get_size(&self) -> Coord;

    fn parse(parser: &CanvasParser) -> Self;
    fn make_drop(&mut self);

    fn update(&mut self);
}
