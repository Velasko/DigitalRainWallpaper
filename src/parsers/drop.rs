extern crate structopt;
use crate::utils::color::Color;
use derive_getters::Getters;

fn chance_validator(arg: String) -> Result<(), String> {
    match arg.parse::<f32>() {
        Err(_) => Err(format!("{arg} is not a numeric value")),
        Ok(value) => {
            if value > 0.0 && value < 1.0 {
                Ok(())
            } else {
                Err(format!("{arg} must be between 0 and 1 (non inclusive)"))
            }
        }
    }
}

#[derive(Debug, structopt::StructOpt, Getters)]
pub struct DropOptions {
    #[structopt(long, default_value = "green")]
    drop_colors: Vec<Color>,

    #[structopt(long, group = "layer", default_value = "3")]
    drop_speeds: Vec<usize>,

    #[structopt(long, default_value = "0.9", validator = chance_validator)]
    spawn_dist_height: f64,
}
