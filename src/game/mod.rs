pub mod grid;

use std::f32;
use message::PlayerAction;

pub struct Game {
    pub grid: grid::Grid,
    shots: Vec<shot::Shot>,
    bunkers: Vec<Bunker>
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

mod shot {

    pub struct Shot {
        pub x_pos: f32,
        pub y_pos: f32,
        pub x_speed: f32,
        pub y_speed: f32,
        pub destruction_radius: f32
    }

    impl Shot {

        const GRAVITY: f32 = 0.1;

        pub fn new(x_pos: f32, y_pos: f32, angle_radians: f32, charge_percent: u8) -> Shot {
            let speed = charge_percent as f32 / 10f32;
            return Shot{
                x_pos: x_pos,
                y_pos: y_pos,
                x_speed: speed * angle_radians.cos(),
                y_speed: speed * angle_radians.sin(),
                destruction_radius: 10f32
            };
        }

        pub fn stride(&mut self) {
            self.x_pos += self.x_speed;
            self.y_pos += self.y_speed;
            self.y_speed += Shot::GRAVITY;
        }

    }

}

pub struct Bunker {
    pub x_pos: i16,
    pub y_pos: i16,
    pub radius: i16,
    angle_radians: f32,
    cannon_length: i16,
    charge_percent: u8
}

impl Bunker {

    pub fn new_at_origin() -> Bunker {
        return Bunker::new(0, 0);
    }

    pub fn new(x_pos: i16, y_pos: i16) -> Bunker {
        return Bunker {
            x_pos: x_pos,
            y_pos: y_pos,
            radius: 10,
            angle_radians: /*f32::consts::PI*1.5*/4.0,  // TODO set to f32::consts::PI*1.5
            cannon_length: 20,
            charge_percent: /*0*/42 // TODO set to 0
        };
    }

    pub fn get_shoot_pos_xy(&self) -> (i16, i16) {
        let sin_cos = self.angle_radians.sin_cos();
        return (
            self.x_pos + (self.cannon_length as f32 * sin_cos.1) as i16,
            self.y_pos + (self.cannon_length as f32 * sin_cos.0) as i16);
    }

    pub fn get_cannon_pos_x1y1x2y2(&self) -> (i16, i16, i16, i16) {
        let shot_pos = self.get_shoot_pos_xy();
        return (self.x_pos, self.y_pos, shot_pos.0, shot_pos.1);
    }
}
