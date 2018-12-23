/*////////// Grundgerüst ///////////////
fn run (args &[String]) {
    let battlefield = Bf.new();
    loop {
        for serverMessage in serverMessages {
            for actions in serverMessage {
                battlefield.alter(action);
            }
            battlefield.stride();
        }
        let presenter = Presenter::new(&mut battlefield);
        presenter.present();
        for event in poll_events() {
            controller.useEvent(&event);
            presenter.useEvent(&event);
        }
        let actions = controller.takeActions();
        sendMessageToServer(actions);
    }
}
//////////////////////////////////// */

use sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::{Event};
use network;
use super::message::{ServerMessage,ClientMessage,ServerMessageInit};
use present::{Presenter,PresenterState};
use control::{Controller};


pub fn run(opts: &[String]) {
    println!("opts: {:?}", opts);
    
    // connect to server
    let other = match network::Simple::connect_to_server("127.0.0.1:8080") {
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
    let sdl_context = sdl2::init().unwrap();
    let mut presenter_state = PresenterState::new(&sdl_context, &battlefield);
    let mut controller = Controller::new(&sdl_context);
    
    
    'mainloop: loop {
        
        // recieve
        let msg: ServerMessage = 'recieve: loop {
            match network::Simple::recieve(&other) {
                Ok(msg) => {
                    println!("server: {:?}", &msg);
                    break msg;
                }
                Err(err) => {
                    println!("coundn't recieve from server: {}", err);
                    println!("retrying...", );
                    continue 'recieve;
                }
            }
        };
        let messages = msg.client_messages;
        
        
        // update battlefield
        for (player_id,client_message) in messages {
            for action in client_message.actions {
                battlefield.execute_action(player_id, &action);
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
        let actions = controller.take_actions();
        if actions.len() > 0 {
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
}
