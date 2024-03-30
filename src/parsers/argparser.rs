extern crate structopt;
use structopt::StructOpt;

use super::{canvas::CanvasParser, drop::DropOptions};

#[derive(StructOpt, Debug)]
pub struct ArgParser {
    #[structopt(flatten)]
    pub canvas: CanvasParser,

    #[structopt(flatten)]
    pub drop: DropOptions,
}
