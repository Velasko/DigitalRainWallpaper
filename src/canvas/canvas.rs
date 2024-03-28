use crate::structopt::StructOpt;
use rand::{rngs::ThreadRng, thread_rng};

use crate::canvas::layer::LayerTrait;
use crate::common::color::Color;

#[derive(Debug, StructOpt)]
#[structopt(name = "Canvas")]
pub struct CanvasParser {
    #[structopt(
        short = "s",
        long = "size",
        value_name = "pixels",
        number_of_values = 2,
        required = true
    )]
    size: Vec<usize>,

    #[structopt(long)]
    persistency: f32,

    #[structopt(long, group = "layer", default_value = "3")]
    layer_groups: u8,

    #[structopt(long, group = "layer")]
    //could not set default here, set in parser instead
    layer_speeds: Option<Vec<usize>>,

    #[structopt(long, default_value = "black")]
    bg_color: Color,

    #[structopt(long, default_value = "green")]
    drop_color: Color,

    #[structopt(long)]
    drop_gradient: Option<Vec<Color>>,
}

#[derive(Debug)]
pub struct Canvas<L> {
    size: [usize; 2],
    layers: Vec<L>,
    rng: ThreadRng,
    bg_color: Color,
    drop_color: Color,
    drop_gradient: Option<Vec<Color>>,
}

pub trait CanvasTrait {
    type LayerType: LayerTrait;
    fn parse(parser: CanvasParser) -> Self;
}

impl<L> CanvasTrait for Canvas<L>
where
    L: LayerTrait,
{
    type LayerType = L;

    fn parse(parser: CanvasParser) -> Self {
        Self {
            size: parser.size.try_into().expect("Parser has size limit"),
            rng: thread_rng(),
            bg_color: parser.bg_color,
            drop_color: parser.drop_color,
            drop_gradient: parser.drop_gradient,
            layers: Self::LayerType::make_batch(
                parser.layer_groups,
                match parser.layer_speeds {
                    Some(speeds) => speeds,
                    None => vec![10, 20, 30],
                },
            ),
        }
    }
}
