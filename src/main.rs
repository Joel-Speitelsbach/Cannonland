extern crate sdl2;

use std::env;

mod window;
mod joel;
// mod data;

use std::path::Path;

// #[cfg(test)]
// mod tests {
    // use std::env;
    // use std::path::Path;
    // use ::window;
    // #[test]
    // fn zoom() {

        // let args: Vec<_> = env::args().collect();

        // if args.len() < 2 {
            // println!("Usage: cargo run /path/to/image.(png|jpg)")
        // } else {
            // window::run(Path::new(&args[1]));
        // }
    // }
// }

// #[test]
// fn tool() {}

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: cargo run <module name> <args>")
    } 
    
    if &args[1] == "joel" {
        joel::run();
    } else {
        window::run(Path::new(&args[1]));
    }
}
