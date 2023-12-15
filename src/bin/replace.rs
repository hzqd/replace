use aoko::{
    no_std::{algebraic::sum::TimeUnit, pipelines::pipe::Pipe},
    standard::functions::fun::{measure_time_with_value, time_conversion_with_unit},
};
use replaces::get_args;
use std::{fs, time::Duration};

fn replace() -> (impl FnOnce(Duration) -> f32, TimeUnit) {
    let args = get_args();
    let str = fs::read_to_string(args.input)
        .unwrap()
        .replace(&args.from, &args.to);
    fs::write(args.output, str).unwrap();
    time_conversion_with_unit(args.time)
}

fn main() {
    measure_time_with_value(replace)
        .pipe(|(e, (f, u))| println!("Execution time: {} {u:?}.", f(e)));
}
