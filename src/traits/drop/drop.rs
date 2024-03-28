extern crate structopt;
use structopt::StructOpt;

use super::parser::DropOptions;
use super::tail::*;
use crate::utils::coordinates::Coord;

pub trait DropTrait {
    type TailType: TailTrait;
    fn new(spawn: Coord, death: Coord, brightness: u8, tail_size: usize) -> Self;
    fn get_options(&self) -> DropOptions {
        DropOptions::from_args()
    }
}
