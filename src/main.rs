mod asg;
mod syn1;
mod interpreter;
mod types;

use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_synapse_file>", args[0]);
        std::process::exit(1);
    }

    let asg = syn1::load_synapse_file(&args[1])?;
    interpreter::execute(&asg)?;
    Ok(())
}
