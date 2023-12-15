use aoko::no_std::algebraic::sum::TimeUnit;
use clap::Parser;

/// A CLI tool for replace *.txt string

#[derive(Parser)]
#[clap(version = "0.0.0", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify the input file name
    #[clap()]
    pub input: String,

    /// Specify the output file name
    #[clap(short, long, default_value = "replaced.txt")]
    pub output: String,

    /// Specify the String to be replaced
    #[clap(short, long)]
    pub from: String,

    /// Specify the String after replacement
    #[clap(short, long)]
    pub to: String,

    /// Specify the time unit, support nanos, micros, millis, secs
    #[clap(short, long, default_value = "millis")]
    pub time: TimeUnit,
}

pub fn get_args() -> Args {
    Args::parse()
}
