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

    // connect to server
    let (client,serverMsgInit) = Client::connect_to_server("localhost".to_string()).unwrap();
    let ServerMessageInit{ mut battlefield, player_id } = serverMsgInit;

    // init battlefield
    let win_size = (battlefield.grid.width as u32, battlefield.grid.height as u32);
    let sdl_context = sdl2::init().unwrap();
    let canvas = present::new_window(&sdl_context.video().unwrap(), win_size);
    let texture_creator = canvas.texture_creator();
    let mut presenter_state = PresenterState::new(canvas, &texture_creator, &battlefield);
    let mut controller = Controller::new(&sdl_context);

    // temp variables
    let mut msg_for_server = ClientMessage{ actions: vec!() };

    'mainloop: loop {
        // message from server
        let server_msg = client.stride(msg_for_server).unwrap();


        // update battlefield
        for (player_id,client_message) in &server_msg.client_messages {
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


        // msg for server
        let actions = controller.poll_actions();
        msg_for_server = ClientMessage {
            actions
        };
    }
}