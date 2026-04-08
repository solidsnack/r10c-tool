use std::error::Error;

use bpaf::*;
use r10c;

#[derive(Clone, Debug)]
enum Op {
    Nearest(f64),
    Series(Direction, f64),
    Generate(r10c::const_calculator::Boundaries),
}

#[derive(Clone, Debug)]
enum Direction {
    Prev,
    Near,
    Next,
}

fn nearest_request() -> impl Parser<Op> {
    positional("N")
        .help("Floating point number to round to nearest preferred number.")
        .map(Op::Nearest)
}

fn series_request() -> impl Parser<Op> {
    let direction = positional("DIRECTION")
        .help("Rounding operation: prev, near, next")
        .parse(|s: String| match s.as_str() {
            "prev" => Ok(Direction::Prev),
            "near" => Ok(Direction::Near),
            "next" => Ok(Direction::Next),
            _ => Err(format!("Unknown direction: {s}")),
        });
    let n = positional("N").help("Floating point number.");
    construct!(Op::Series(direction, n))
}

fn generate_request() -> impl Parser<Op> {
    positional("BOUNDS")
        .help("Name of bounds to generate: lower, mid, upper")
        .parse(|s: String| match s.as_str() {
            "lower" => Ok(r10c::const_calculator::Boundaries::Lower),
            "mid" => Ok(r10c::const_calculator::Boundaries::Mid),
            "upper" => Ok(r10c::const_calculator::Boundaries::Upper),
            _ => Err(format!("Unknown bounds: {s}")),
        })
        .map(Op::Generate)
}

fn main() -> Result<(), Box<dyn Error>> {
    let options: OptionParser<Op> =
        construct!([nearest_request(), series_request(), generate_request(),])
            .to_options();
    let parsed = options.run();

    match parsed {
        Op::Nearest(n) => {
            let r = r10c::near(n);

            println!("{r}");
        }
        Op::Generate(bounds) => {
            for n in bounds.calculate() {
                println!("{n}");
            }
        }
        Op::Series(direction, n) => {
            let r = match direction {
                Direction::Prev => r10c::prev(n),
                Direction::Near => r10c::near(n),
                Direction::Next => r10c::next(n),
            };

            println!("{r}");
        }
    }

    Ok(())
}
