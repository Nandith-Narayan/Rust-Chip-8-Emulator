//use std::time::Instant;

use pixels::Error;

mod cpu;
mod render;
mod font_data;

fn main() -> Result<(), Error>{


    render::run_main_loop()?;

    Ok(())
}
