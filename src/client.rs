use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::{Event};
use network;
use super::message::{ServerMessage,ClientMessage,ServerMessageInit};
use present::{self,Presenter,PresenterState};
use control::{Controller};
use config;
use battlefield::Battlefield;
use std::marker::PhantomData;
 

type ConnectError = String;
type StrideError = String;

struct Client {
    server: network::OtherSide,
}

impl Client {
    fn connect_to_server(server_ip: String) -> Result<(Client,ServerMessageInit),ConnectError> {
        Err("not implemented".to_string())
    }
    fn stride(&self, client_msg: ClientMessage) -> Result<ServerMessage,StrideError> {
        Err("not implemented".to_string())
    }
}

 
pub fn run(server_ip: String) {

    // connect to server
    let other = match network::Simple::connect_to_server(server_ip + ":" + config::PORT) {
        Ok(ok) => ok,
        Err(err) => {
            println!("failed to connect to server");
            println!("debug info: {}", err);
            return;
        },
    };
    other.set_nonblocking(false).unwrap();

    // recieve init message
    let init_msg: ServerMessageInit = network::Simple::recieve(&other)
        .expect("failed to recieve init msg");
    println!("init_msg.player_id: {:?}", init_msg.player_id);
    let mut battlefield = init_msg.battlefield;


    // init game
    let win_size = (battlefield.grid.width as u32, battlefield.grid.height as u32);
    let sdl_context = sdl2::init().unwrap();
    let canvas = present::new_window(&sdl_context.video().unwrap(), win_size);
    let texture_creator = canvas.texture_creator();
    let mut presenter_state = PresenterState::new(canvas, &texture_creator, &battlefield);
    let mut controller = Controller::new(&sdl_context);

    'mainloop: loop {

        // recieve
        let msg: ServerMessage = match network::Simple::recieve(&other) {
            Ok(msg) => {
                msg
            }
            Err(err) => {
                // connection lost
                println!("server disconnected");
                println!("debug info: {}", err);
                break 'mainloop
            }
        };
        // if msg.client_messages.len() > 0 {
        //     println!("server: {:?}", &msg);
        // }
        let messages = msg.client_messages;


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

        // send
        let actions = controller.poll_actions();
        let msg = ClientMessage {
            actions: actions,
        };
        if let Err(err) = network::Simple::send(&other, &msg) {
            // connection lost
            println!("server disconnected");
            println!("debug info: {}", err);
            break 'mainloop;
        }
    }
}
