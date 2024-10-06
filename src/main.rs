extern crate sdl2;

#[macro_use]
extern crate serde_derive;

mod present;
mod program;
mod battlefield;
mod message;
mod control;
mod serverless_client;
mod network;
mod server;
mod client;
mod util;
mod config;
mod sound;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 { program::run() }
    else {
        match args[1].as_str() {
            "particle_test" => battlefield::grid::particle_test::run(),
            "present" => serverless_client::run_standalone(),
            "client" => {
                if args.len() < 3 {
                    eprintln!(
                        "Usage: cargo run client [hostname]\
                        \n       cargo run client [ip address of server]
                        "
                    );
                    std::process::exit(-1);
                }
                client::run_standalone(&args[2])
            },
            "server" => server::run(),
            "program" => program::run(),
            "sound" => sound::test(),
            x => {
                println!("module name {x} does not exist");
                eprintln!(
                    "Usage:\
                    \n     cargo run [module name] [args] 
                    "
                );
                std::process::exit(-1);
            },
        }
    }
}
