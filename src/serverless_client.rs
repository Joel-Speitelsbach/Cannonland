use std::time::SystemTime;
use sdl2;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use battlefield;
use present::{Presenter,PresenterState};
use control::{Controller};

pub fn run(_: &[String]) {
    let sdl_context = sdl2::init().unwrap();
    let mut battlefield = battlefield::Battlefield::new();

    let mut presenter_state = PresenterState::new(&sdl_context, &battlefield);
    let mut controller = Controller::new(&sdl_context);

    //init misc
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(60).unwrap();
    let mut counter: i64 = 0;

    'mainloop: loop {

        // iterate battlefield
        let calc_time = SystemTime::now();
        let actions = controller.poll_actions();
        for action in actions {
            battlefield.execute_action(0, &action);
        }
        battlefield.stride();
        if counter%60 == 0 {
            print!("calc needed {} msecs",
                calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));
        }

        // events
        let mut presenter = Presenter::new(&mut presenter_state, &mut battlefield);
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

        // present
        let present_time = SystemTime::now();
        presenter.present();
        if counter%60 == 0 {
            print!(", present needed {} msecs",
                present_time.elapsed().unwrap().subsec_nanos() / (1000*1000));
            println!(", calc and present needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));
        }

        counter += 1;
        fps_manager.delay();
    }
}
