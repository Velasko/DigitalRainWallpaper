extern crate structopt;
use crate::utils::color::Color;

#[derive(Debug, structopt::StructOpt)]
pub struct DropOptions {
    #[structopt(long, default_value = "green")]
    drop_gradient: Vec<Color>,

    #[structopt(long, group = "layer")]
    drop_speeds: Vec<usize>,
}
