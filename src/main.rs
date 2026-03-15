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
    Floor,
    Round,
    Ceiling,
}

fn nearest_request() -> impl Parser<Op> {
    positional("N")
        .help("Floating point number to round to nearest preferred number.")
        .map(Op::Nearest)
}

fn series_request() -> impl Parser<Op> {
    let direction = positional("DIRECTION")
        .help("Rounding operation: floor, round, ceiling")
        .parse(|s: String| match s.as_str() {
            "floor" => Ok(Direction::Floor),
            "round" => Ok(Direction::Round),
            "ceiling" => Ok(Direction::Ceiling),
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
                Direction::Floor => r10c::prev(n),
                Direction::Round => r10c::near(n),
                Direction::Ceiling => r10c::next(n),
            };

            println!("{r}");
        }
    }

    Ok(())
}
