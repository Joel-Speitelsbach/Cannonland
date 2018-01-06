pub mod grid;
pub mod shot;
pub mod bunker;

use std::f32;
use message::{PlayerAction,PlayerID};
use self::grid::color::Color;

pub struct Battlefield {
    pub grid: grid::Grid,
    pub bunkers: Vec<bunker::Bunker>,
    pub shots: Vec<shot::Shot>
}

impl Battlefield {

    pub fn new() -> Battlefield {
        let mut bunkers = Vec::with_capacity(8);
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerBlue));
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerRed));
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerGreen));
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerYellow));
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerTeal));
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerPurple));
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerGrey));
        bunkers.push(bunker::Bunker::new_at_nowhere(Color::BunkerOrange));

        return Battlefield{ grid: grid::create_grid(), bunkers, shots: Vec::new() };
    }

    pub fn stride(&mut self) {
        self.collide();

        self.grid.stride();
        self.grid.update_bunkers(&mut self.bunkers);
        self.stride_shots();
    }

    pub fn execute_action(&mut self, player_id: PlayerID, action: &PlayerAction) {
        let mut bunker = &mut self.bunkers[player_id as usize];
        match *action {
            PlayerAction::TurnCannon { diff_angle: angle } => {
                bunker.change_angle_radians_trim_overflow(angle);
            },
            _ => (),
        }
    }

    pub fn shoot(&mut self, bunker_id: u8) {
        let bunker = &self.bunkers[bunker_id as usize];

        let shoot_pos = bunker.get_shoot_pos_xy();
        let shot = shot::Shot::new(shoot_pos.0 as f32, shoot_pos.1 as f32, bunker.get_angle_radians(), bunker.get_charge());
        self.shots.push(shot);
    }

    fn collide(&mut self) {
        for i in (0..self.shots.len()).rev() {
            let x_pos = self.shots[i].x_pos as usize;
            let y_pos = self.shots[i].y_pos as usize;
            if self.grid.collides_at_position(x_pos, y_pos) {
                self.grid.delete_radius_leave_out_bunkers(x_pos, y_pos, self.shots[i].destruction_radius as usize);
                self.shots.remove(i);
            }
        }
    }

    fn stride_shots(&mut self) {
        for i in (0..self.shots.len()).rev() {
            self.shots[i].stride();
            if self.shots[i].y_pos > self.grid.height as f32 + 100f32 {
                self.shots.remove(i);
            }
        }
    }

}
