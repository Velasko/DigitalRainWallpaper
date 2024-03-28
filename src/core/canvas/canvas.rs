use rand::{rngs::ThreadRng, thread_rng};

use crate::traits::canvas::parser::CanvasParser;
use crate::traits::drop::*;
use crate::utils::color::Color;

#[derive(Debug)]
pub struct Canvas<D> {
    size: [usize; 2],
    drops: Vec<D>,
    rng: ThreadRng,
    bg_color: Color,
}

pub trait CanvasTrait {
    type DropType: DropTrait;
    fn parse(parser: CanvasParser) -> Self;
}

impl<D> CanvasTrait for Canvas<D>
where
    D: DropTrait,
{
    type DropType = D;

    fn parse(parser: CanvasParser) -> Self {
        Self {
            size: parser.get_size(),
            rng: thread_rng(),
            bg_color: parser.get_bg_color(),
            drops: Vec::new(),
        }
    }
}
