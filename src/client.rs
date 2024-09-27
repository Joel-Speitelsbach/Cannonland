use crate::battlefield;
use crate::control::Controller;
use crate::network;
use crate::message::{ServerMessage,ClientMessage,ServerMessageInit};
use crate::present::{self,Presenter,PresenterState};
use crate::program;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use crate::sound::Sound;


pub fn run_standalone(server_ip: &str) {
    // init window
    let win_size: (i32,i32) = battlefield::SIZE; 
    let sdl_context = sdl2::init().unwrap();
    let canvas = present::new_window(&sdl_context.video().unwrap(), win_size);
    let sound = Sound::init();
    let mut window = program::Window {
        sdl_context,
        canvas,
        sound,
    };

    run(server_ip, &mut window);
}


pub fn run(server_ip: &str, window: &mut program::Window) {

    // connect to server
    let other = match network::connect_to_server(server_ip) {
        Ok(ok) => ok,
        Err(err) => {
            println!("failed to connect to server");
            println!("debug info: {}", err);
            return;
        },
    };

    // recieve init message
    let init_msg: ServerMessageInit = other.recieve()
        .expect("failed to recieve init msg");
    println!("init_msg.player_id: {:?}", init_msg.player_id);
    let mut battlefield = init_msg.battlefield;


    // init game
    let texture_creator = window.canvas.texture_creator();
    let mut presenter_state = PresenterState::new(&mut window.canvas, &texture_creator, battlefield.size());
    let mut controller = Controller::new(&window.sdl_context);

    'mainloop: loop {

        // recieve
        let msg: ServerMessage = match other.recieve() {
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
        let messages = msg.client_messages;


        // update battlefield
        for (player_id,client_message) in &messages {
            for action in &client_message.actions {
                battlefield.execute_action(*player_id, action, &window.sound);
            }
        }
        battlefield.stride(&window.sound);


        // present battlefield
        let mut presenter = Presenter::new(&mut presenter_state, &mut battlefield);
        presenter.present();


        // events
        for event in window.sdl_context.event_pump().unwrap().poll_iter() {
            presenter.respond_to(&event);
            controller.use_event(&event);
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

        // send msg to server
        let actions = controller.poll_actions();
        let msg = ClientMessage {
            actions: actions,
        };
        if let Err(err) = other.send(&msg) {
            // connection lost
            println!("server disconnected");
            println!("debug info: {}", err);
            break 'mainloop;
        }
    }
}
