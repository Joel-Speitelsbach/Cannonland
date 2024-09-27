// reads input actions

// joel: i don't like the design of this module.
//       probable reason: 'Controller' is mutable between game iterations.
//       i fear this style leads to unmanagable code, when the project grows.
//       let's be careful and think twice if we are about to replicate this
//       module design in other modules.


use sdl2::event::Event;
use sdl2::{Sdl,TimerSubsystem};
use sdl2::keyboard::Keycode;

use crate::message::{PlayerAction, ChangeWeapon};

pub struct Controller {
    left_pressed: (bool, i32), // (whether key is currently pressed
    right_pressed: (bool, i32), //   , timestamp)
    fire_pressed: (bool, i32),
    up_released: bool,
    down_released: bool,
    cannon_movement: i32,
    cannon_load: i32,
    fire: bool,
    timer: TimerSubsystem,
}

impl Controller {
    pub fn new(sdl_context: &Sdl) -> Controller {
        let timer = sdl_context.timer()
            .expect("could not initialise the time subsystem");
        Controller {
            left_pressed: (false,0),
            right_pressed: (false,0),
            fire_pressed: (false,0),
            up_released: false,
            down_released: false,
            cannon_movement: 0,
            cannon_load: 0,
            fire: false,
            timer: timer,
        }
    }

    pub fn use_event(&mut self, event: &Event) {
        match *event {
            Event::KeyDown { repeat: false, timestamp: time, keycode: k,.. } => match k {
                Some(Keycode::Right) => self.right_pressed = (true, time as i32),
                Some(Keycode::Left) => self.left_pressed = (true, time as i32),
                Some(Keycode::Return) => self.fire_pressed = (true, time as i32),
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
                Some(Keycode::Return) => if let (true,old_time) = self.fire_pressed {
                    self.fire_pressed = (false,0);
                    let time_diff = time as i32 - old_time;
                    self.cannon_load += time_diff;
                    self.fire = true;
                },
                Some(Keycode::Up) => {
                    self.up_released = true;
                },
                Some(Keycode::Down) => {
                    self.down_released = true;
                },
                _ => (),
            },
            _ => {},
        }
    }

    fn take_cannon_movement(&mut self) -> Option<PlayerAction> {
        let time = self.timer.ticks() as i32;
        let mut cannon_movement = self.cannon_movement;
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
        if cannon_movement == 0 {
            return None;
        }
        let angle = cannon_movement as f32 / 300.;
        Some(PlayerAction::TurnCannon {
            diff_angle: angle,
        })
    }

    fn take_prev_weapon(&mut self) -> Option<PlayerAction> {
        if self.up_released {
            self.up_released = false;
            return Some(PlayerAction::CangeWeapon(ChangeWeapon::Prev));
        }
        return None;
    }

    fn take_next_weapon(&mut self) -> Option<PlayerAction> {
        if self.down_released {
            self.down_released = false;
            return Some(PlayerAction::CangeWeapon(ChangeWeapon::Next));
        }
        return None;
    }

    fn poll_fire(&mut self) -> Vec<PlayerAction> {
        let mut actions = vec!();
        let time = self.timer.ticks() as i32;
        if let (true,old_time) = self.fire_pressed {
            let time_diff = time - old_time;
            self.fire_pressed = (true,time);
            self.cannon_load += time_diff;
        }
        let cannon_percent = self.cannon_load as f32 / 1000.;
        if cannon_percent > 0.02 || self.fire {
            actions.push(PlayerAction::IncreaseLoad {
                inc: cannon_percent,
            });
            self.cannon_load = 0;
        }
        if self.fire {
            actions.push(PlayerAction::Fire);
            self.fire = false;
        }
        actions
    }

    pub fn poll_actions(&mut self) -> Vec<PlayerAction> {
        let mut actions = vec!();
        if let Some(action) = self.take_cannon_movement() {
            actions.push(action);
        }
        if let Some(action) = self.take_prev_weapon() {
            actions.push(action);
        }
        if let Some(action) = self.take_next_weapon() {
            actions.push(action);
        }
        actions.append(&mut self.poll_fire());
        actions
    }
}
