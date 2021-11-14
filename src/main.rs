//! # castline driver
//! <p> Usage: <em> castline [COMMAND] [ARGS] </em>
//! where COMMAND is one of: </p>
//! <ul>
//! <li>catch</li>
//! <li>trap</li>
//! <li>fish</li>
//! </ul>

use structopt::StructOpt;

mod cast;
mod catch;

/// Define command line options.
#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(
        name = "catch",
        about = "Displays internal information of datagrams on the specified port"
    )]
    Catch(catch::Options),
    #[structopt(name = "cast", about = "Send []to [HOST]")]
    Cast(cast::Options),
    #[structopt(
        name = "trap",
        about = "Given argument [AMOUNT]:[SIZE]; displays percentage of packets not acknowledged"
    )]
    Trap,
    #[structopt(
        name = "fish",
        about = "Displays meta-information about specified port's usage"
    )]
    Fish,
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Catch(mut options) => catch::run(&mut options),
        Opt::Cast(mut options) => cast::run(&mut options),
        Opt::Trap => run_trap(),
        Opt::Fish => run_fish(),
    }
}

fn run_trap() {
    println!("trap");
}

fn run_fish() {
    println!("fish");
}
