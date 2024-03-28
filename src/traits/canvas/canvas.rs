use super::parser::CanvasParser;
use crate::traits::drop::DropTrait;

pub trait CanvasTrait {
    type DropType: DropTrait;
    fn parse(parser: CanvasParser) -> Self;
}
