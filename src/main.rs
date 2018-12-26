extern crate sdl2;

#[macro_use]
extern crate serde_derive;

mod present;
mod battlefield;
mod message;
mod control;
mod serverless_client;
mod network;
mod server;
mod client;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage:\
            \n     cargo run <module name> <args>"
        );
        ::std::process::exit(0);
    }

    match args[1].as_str() {
        "particle_test" => battlefield::grid::particle_test::run(),
        "present" => serverless_client::run(&args[2..]),
       "client" => client::run(&args[2..]),
        "server" => server::run(&args[2..]),
        x => println!("module name {} does not exist", x)
    }
}
