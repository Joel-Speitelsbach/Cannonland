pub mod grid;

use std::f32;

pub struct Game {
    pub grid: grid::Grid
}

impl Game {

    pub fn new() -> Game {
        return Game{grid: grid::create_grid()};
    }

}

mod shot {
    struct Shot;
}

pub struct Bunker {
    pub x_pos: i16,
    pub y_pos: i16,
    pub radius: i16,
    angle_radians: f32,
    cannon_length: i16,
}

impl Bunker {

    pub fn new(x_pos: i16, y_pos: i16) -> Bunker {
        return Bunker {
            x_pos: x_pos, 
            y_pos: y_pos, 
            radius: 10, angle_radians: 
            f32::consts::PI*1.5, 
            cannon_length: 20,
        };
    }

    pub fn get_shot_pos_xy(&self) -> (i16, i16) {
        let sin_cos = self.angle_radians.sin_cos();
        return (
            self.x_pos + (self.cannon_length as f32 * sin_cos.1) as i16,
            self.y_pos + (self.cannon_length as f32 * sin_cos.0) as i16);
    }

    pub fn get_cannon_pos_x1y1x2y2(&self) -> (i16, i16, i16, i16) {
        let shot_pos = self.get_shot_pos_xy();
        return (self.x_pos, self.y_pos, shot_pos.0, shot_pos.1);
    }
}