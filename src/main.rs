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
mod fish;
mod trap;

/// Define command line options.
#[derive(Debug, StructOpt)]
enum Opt {
    #[structopt(
        name = "catch",
        about = "Displays internal information of datagrams on the specified port"
    )]
    Catch(catch::Options),
    #[structopt(name = "cast", about = "Send [DATA] to [HOST]")]
    Cast(cast::Options),
    #[structopt(
        name = "trap",
        about = "Given argument [AMOUNT]:[SIZE]; displays percentage of packets not acknowledged"
    )]
    Trap(trap::Options),
    #[structopt(
        name = "fish",
        about = "Displays meta-information about specified port's usage"
    )]
    Fish(fish::Options),
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Catch(mut options) => catch::run(&mut options),
        Opt::Cast(mut options) => cast::run(&mut options),
        Opt::Trap(mut options) => trap::run(&mut options),
        Opt::Fish(mut options) => fish::run(&mut options),
    }
}
