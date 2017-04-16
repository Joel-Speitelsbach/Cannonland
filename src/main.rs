extern crate sdl2;

use std::env;

mod window;

use std::path::Path;

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        window::run(Path::new(&args[1]));
    }
}