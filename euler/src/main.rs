extern crate problems;

use std::env;

use problems::p1;

fn main() {
    if env::args().count() != 2 {
        println!("Introduce the problem number");
        return;
    }

    let arg = env::args().nth(1).unwrap();
    let argn: i32 = arg.parse().ok().unwrap_or(1);

    match argn {
        1 => p1::solve(),
        x @ _ => println!("Problem {} not implemented", x),
    }
}
