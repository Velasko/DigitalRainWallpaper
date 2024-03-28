extern crate structopt;
use crate::utils::color::Color;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "Canvas")]
pub struct CanvasParser {
    #[structopt(short = "s", long = "size", number_of_values = 2, required = true)]
    size: Vec<usize>,

    #[structopt(long)]
    persistency: f32,

    #[structopt(long, default_value = "black")]
    bg_color: Color,
}

impl CanvasParser {
    pub fn get_size(&self) -> [usize; 2] {
        self.size.clone().try_into().expect("Parser has size limit")
    }

    pub fn get_bg_color(&self) -> Color {
        self.bg_color.clone()
    }
}
