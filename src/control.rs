// reads input actions

use sdl2::event::Event;
use sdl2::{Sdl,TimerSubsystem};
use sdl2::keyboard::Keycode;

use message::PlayerAction;

pub struct Controller {
    left_pressed: (bool, i32),
    right_pressed: (bool, i32),
    cannon_movement: i32,
    timer: TimerSubsystem,
}

impl Controller {
    pub fn new(sdl_context: &Sdl) -> Controller {
        let timer = sdl_context.timer()
            .expect("could not initialise the time subsystem");
        Controller {
            left_pressed: (false,0),
            right_pressed: (false,0),
            cannon_movement: 0,
            timer: timer,
        }
    }
    pub fn use_event(&mut self, event: &Event) {
        match *event {
            Event::KeyDown { repeat: false, timestamp: time, keycode: k,.. } => match k {
                Some(Keycode::Right) => self.right_pressed = (true, time as i32),
                Some(Keycode::Left) => self.left_pressed = (true, time as i32),
                _ => (),
            },
            Event::KeyUp { repeat: false, timestamp: time, keycode: k,..} => match k {
                Some(Keycode::Right) => if let (true,old_time) = self.right_pressed {
                    self.right_pressed = (false,0);
                    let time_diff = time as i32 - old_time;
                    self.cannon_movement += time_diff;
                },
                Some(Keycode::Left) => if let (true,old_time) = self.left_pressed {
                    self.left_pressed = (false,0);
                    let time_diff = time as i32 - old_time;
                    self.cannon_movement -= time_diff;
                },
                _ => (),
            },
            _ => {},
        }
    }
    pub fn take_actions(&mut self) -> Vec<PlayerAction> {
        let time = self.timer.ticks() as i32;
        let mut cannon_movement = self.cannon_movement;
        if cannon_movement == 0 {
            return vec!();
        }
        self.cannon_movement = 0;
        if let (true,old_time) = self.right_pressed {
            let time_diff = time - old_time;
            self.right_pressed = (true,time);
            cannon_movement += time_diff;
        }
        if let (true,old_time) = self.left_pressed {
            let time_diff = time - old_time;
            self.left_pressed = (true,time);
            cannon_movement -= time_diff;
        }
        let angle = cannon_movement as f32 / 300.;
        println!("diff angle: {}", &angle);
        vec!(
            PlayerAction::TurnCannon {
                diff_angle: angle,
            },
        )
    }
}
