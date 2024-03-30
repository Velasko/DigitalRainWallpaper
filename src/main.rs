extern crate lazy_static;
extern crate structopt;
use crate::structopt::StructOpt;
use lazy_static::lazy_static;

mod core;
mod parsers;
mod traits;
mod utils;

use crate::parsers::{canvas::CanvasParser, drop::DropOptions, ArgParser};
use traits::canvas::CanvasTrait;

use core::canvas::Canvas;
use core::drop::tail::Tail;
use core::drop::Drop;
use utils::charset::Katakana;

lazy_static! {
    pub static ref ARGS: ArgParser = ArgParser::from_args();
    pub static ref RNG: utils::random::Random = utils::random::Random::new();
}

fn main() {
    let canvas_parse = &ARGS.canvas;
    let mut item_test: Canvas<Drop<Tail<Katakana>>> = Canvas::parse(canvas_parse);
    item_test.make_drop();
    println!("{:#?}", item_test);

    println!("\n## Code end ##");
}
