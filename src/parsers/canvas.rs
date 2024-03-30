extern crate structopt;
use crate::utils::color::Color;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "Canvas")]
pub struct CanvasParser {
    #[structopt(short, long, number_of_values = 2, required = true)]
    size: Vec<usize>,

    #[structopt(long, default_value = "black")]
    bg_color: Color,

    #[structopt(long, default_value = "0.5")]
    drop_spawn_chance: f64,
}

impl CanvasParser {
    pub fn get_size(&self) -> [usize; 2] {
        self.size.clone().try_into().expect("Parser has size limit")
    }

    pub fn get_bg_color(&self) -> Color {
        self.bg_color.clone()
    }

    pub fn get_drop_spawn_chance(&self) -> f64 {
        self.drop_spawn_chance
    }
}
