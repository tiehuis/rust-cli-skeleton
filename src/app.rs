use std::fs::File;
use std::io::Read;
use error::*;

use args::Args;

fn read_file(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    let _ = f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn start() -> Result<()> {
    let args = Args::parse()
            .chain_err(|| "error parsing arguments")?;

    let content = read_file(&args.input_name)
            .chain_err(|| format!("unable to open file {}", args.input_name))?;

    println!("{}", content);
    Ok(())
}
