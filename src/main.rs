mod memory;
mod operations;
mod program;
mod values;

use anyhow::{Context, Result};
use program::Program;
use std::{env, fs::read};

fn main() -> Result<()> {
    env_logger::init();
    let mut args = env::args();
    args.next();
    let input_file = args.next().context("expected argument")?;
    let bytes = read(input_file).context("failed to open file")?;
    let mut program = Program::new(bytes);
    program.execute();

    Ok(())
}
