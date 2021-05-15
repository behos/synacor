mod memory;
mod operations;
mod values;

use anyhow::{Context, Result};
use memory::Memory;
use std::{env, fs::read};

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let input_file = args.next().context("expected argument")?;
    let bytes = read(input_file).context("failed to open file")?;
    let mut mem = Memory::new();
    mem.load(bytes);

    println!("{:?}", mem.reserved);
    return Ok(());
}
