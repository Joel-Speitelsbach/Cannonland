pub mod grid;
mod shot;
mod bunker;

use std::f32;
use message::PlayerAction;

pub struct Game {
    pub grid: grid::Grid,
    shots: Vec<shot::Shot>,
    bunkers: Vec<bunker::Bunker>
}

impl Game {

    pub fn new() -> Game {
        let bunkers = Vec::with_capacity(8);
        // TODO add bunkers and pass to grid
        return Game{grid: grid::create_grid(), shots: Vec::new(), bunkers};
    }

    pub fn get_shots(&self) -> &Vec<shot::Shot> {
        return &self.shots;
    }

    pub fn shoot(&mut self/*, bunker: &Bunker*/) {
        let bunker = self.grid.bunkers.get(&grid::color::Color::BunkerYellow).unwrap();  // TODO delete this line

        let shoot_pos = bunker.get_shoot_pos_xy();
        let shot = shot::Shot::new(shoot_pos.0 as f32, shoot_pos.1 as f32, bunker.angle_radians, bunker.charge_percent);
        self.shots.push(shot);
    }

    pub fn stride(&mut self) {
        self.collide();

        self.grid.stride();
        self.stride_shots();
    }

    pub fn alter(&mut self, action: &PlayerAction) {/*TODO*/}

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
