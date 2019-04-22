use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::{Event};
use super::message::{ServerMessage,ClientMessage,ServerMessageInit};
use present::{self,Presenter,PresenterState};
use control::{Controller};
use config;
use battlefield::Battlefield;
use client::Client;


pub fn run() {

    let (client,serverMsgInit) = Client::connect_to_server("localhost".to_string()).unwrap();
    let battlefield = serverMsgInit.battlefield;

    // init battlefield
    let win_size = (battlefield.grid.width as u32, battlefield.grid.height as u32);
    let sdl_context = sdl2::init().unwrap();
    let canvas = present::new_window(&sdl_context.video().unwrap(), win_size);
    let texture_creator = canvas.texture_creator();
    let mut presenter_state = PresenterState::new(canvas, &texture_creator, &battlefield);
    let mut controller = Controller::new(&sdl_context);

    'mainloop: loop {
        client.stride(client_msg: ClientMessage)

        // update battlefield
        for (player_id,client_message) in &messages {
            for action in &client_message.actions {
                battlefield.execute_action(*player_id, action);
            }
        }
        battlefield.stride();


        // present battlefield
        let mut presenter = Presenter::new(&mut presenter_state, &mut battlefield);
        presenter.present();


        // events
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            presenter.respond_to(&event);
            controller.use_event(&event);
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }
}