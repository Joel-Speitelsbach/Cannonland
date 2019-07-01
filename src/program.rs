#![allow(
    dead_code,
    unused_variables,
    unused_imports)
]
use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::{Event};
use super::message::{ServerMessage,ClientMessage,ServerMessageInit};
use present::{self,Presenter,PresenterState};
use control::{Controller};
use config;
use battlefield::Battlefield;


pub fn run() {


    // init game
    let win_size: (i32,i32) = (800, 600); 
    let sdl_context = sdl2::init().unwrap();
    let canvas = present::new_window(&sdl_context.video().unwrap(), win_size);
    let texture_creator = canvas.texture_creator();
    let presenter_state = PresenterState::new(canvas, &texture_creator, win_size);

    // loop {
        /*

        //// UNTERMENÜS
        if mauszeiger_click() {
            client::run();
        }
        if taste_für_server() 
            && server_nicht_gestartet 
        {
            als_hintergrundprozess(server::run);
        }

        zeichne_hauptmenü();
        screen.blit_surface();

        */
    // }
    println!("hallo welt");
}
