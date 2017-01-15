extern crate problems;

use std::env;

fn main() {
    if env::args().count() != 2 {
        println!("Introduce the problem number");
        return;
    }

    let arg = env::args().nth(1).unwrap();
    let argn: i32 = arg.parse().ok().unwrap_or(1);

    match argn {
        1 => println!("Solution: {}", problems::p1::solve()),
        2 => println!("Solution: {}", problems::p2::solve()),
        3 => println!("Solution: {}", problems::p3::solve()),
        x @ _ => println!("Problem {} not implemented", x),
    }
}
