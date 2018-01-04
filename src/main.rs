extern crate sdl2;

#[macro_use]
extern crate serde_derive;

mod present;
mod game;
mod message;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Usage: cargo run <module name> <args>")
    }

    match args[1].as_str() {
        "particle_test" => game::grid::particle_test::run(),
        "present" => present::run(),
        x => println!("module name {} does not exist", x)
    }
}
