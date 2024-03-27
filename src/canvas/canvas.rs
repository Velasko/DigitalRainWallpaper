use crate::canvas::layer::LayerTrait;
use crate::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct CanvasParser {
    #[structopt(short = "s", long = "size", number_of_values = 2, required = true)]
    size: Vec<usize>,

    #[structopt(long)]
    transparency: bool,

    #[structopt(long, default_value = "5")]
    layer_count: u8,

    #[structopt(long)]
    layer_speeds: Vec<usize>,
}

impl CanvasParser {
    pub fn as_canvas<C>(self) -> C
    where
        C: CanvasTrait,
    {
        C::parse(self)
    }
}

#[derive(Debug)]
pub struct Canvas<L> {
    size: [usize; 2],
    persistency: u8, //current frame transparency
    layers: Vec<L>,
}

pub trait CanvasTrait {
    // type LayerType: LayerTrait;
    fn parse(parser: CanvasParser) -> Self;
}

impl<L> CanvasTrait for Canvas<L>
// where
//     L: LayerTrait,
{
    // type LayerType = L;
    fn parse(parser: CanvasParser) -> Self {
        Self {
            size: parser
                .size
                .try_into()
                .expect("CanvasParser didn't limit size ?"),
            persistency: 255,
            layers: Vec::new(),
        }
    }
}
