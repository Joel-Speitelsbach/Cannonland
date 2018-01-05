use std::time::SystemTime;
use sdl2;
use sdl2::event::{Event,WindowEvent};
use sdl2::keyboard::Keycode;

use battlefield;
use present::{Presenter,PresenterState};
use control::{Controller};

pub fn run(opts: &[String]) {
    let sdl_context = sdl2::init().unwrap();
    let mut battlefield = battlefield::Battlefield::new();
    
    let mut presenter_state = PresenterState::new(&sdl_context);
    let mut controller = Controller::new();

    //init misc
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    let mut counter: i64 = 0;
    
    'mainloop: loop {
        
        // iterate battlefield
        let calc_time = SystemTime::now();
        let actions = controller.take_actions();
        /* TODO here: apply 'actions' to battlefield */
        battlefield.stride();
        if counter%60 == 0 {
            print!("calc needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));
        }
        if counter%100 == 0 {
            battlefield.shoot();
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
            print!(", present needed {} msecs", present_time.elapsed().unwrap().subsec_nanos() / (1000*1000));
            println!(", calc and present needed {} msecs", calc_time.elapsed().unwrap().subsec_nanos() / (1000*1000));
        }
        
        counter += 1;
        fps_manager.delay();
    }
}