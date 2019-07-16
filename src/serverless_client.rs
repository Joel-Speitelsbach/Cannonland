use sdl2;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use battlefield;
use present::{self,Presenter,PresenterState};
use control::{Controller};
use message::PlayerAction;
use util;


pub fn run(_: &[String]) {
    // init battlefield
    let mut battlefield = battlefield::Battlefield::new();
    battlefield.execute_action(0, &PlayerAction::NewBunker);
    battlefield.execute_action(1, &PlayerAction::NewBunker);
    battlefield.execute_action(2, &PlayerAction::NewBunker);
 
    //init presenter
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = present::new_window(&sdl_context.video().unwrap(), battlefield.size());
    let texture_creator = canvas.texture_creator();
    let mut presenter_state = PresenterState::new(&mut canvas, &texture_creator, battlefield.size());
    
    //init controller
    let mut controller = Controller::new(&sdl_context);

    //init misc
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(60).unwrap();
    let mut prof_present = util::time::Prof::just_label("present");
    let mut prof_alles = util::time::Prof::just_label("alles");


    'mainloop: loop {
        prof_alles.start();

        // iterate battlefield
        let actions = controller.poll_actions();
        for action in actions {
            battlefield.execute_action(0, &action);
        }
        battlefield.stride();

        // events
        let mut presenter = Presenter::new(&mut presenter_state, &battlefield);
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            presenter.respond_to(&event);
            controller.use_event(&event);
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {},
            }
        }

        // present
        prof_present.start();
        presenter.present();
        prof_present.pause();
        
        // time control
        prof_alles.pause();
        fps_manager.delay();
    }
}
