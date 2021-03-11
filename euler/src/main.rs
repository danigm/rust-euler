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
        4 => println!("Solution: {}", problems::p4::solve()),
        5 => println!("Solution: {}", problems::p5::solve()),
        6 => println!("Solution: {}", problems::p6::solve()),
        7 => println!("Solution: {}", problems::p7::solve()),
        8 => println!("Solution: {}", problems::p8::solve()),
        9 => println!("Solution: {}", problems::p9::solve()),
        10 => println!("Solution: {}", problems::p10::solve()),
        x @ _ => println!("Problem {} not implemented", x),
    }
}
