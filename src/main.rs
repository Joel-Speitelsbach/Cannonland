extern crate sdl2;

#[macro_use]
extern crate serde_derive;

use std::env;

mod joel;
mod grid;
mod window;
mod game;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: cargo run <module name> <args>")
    }

    match args[1].as_str() {
        "joel" => joel::run(),
        "particle_test" => grid::particle_test::run(),
        "window" => window::run(),
        x => println!("module name {} does not exist", x)
    }
}
