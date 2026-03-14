use std::{env::args, error::Error};

use r10c::*;

fn main() -> Result<(), Box<dyn Error>> {
    let n: f64 = args().nth(1).ok_or("Please provide a number.")?.parse()?;

    let r = near(n);

    eprint!("{n} -> ");
    println!("{r}");

    Ok(())
}
