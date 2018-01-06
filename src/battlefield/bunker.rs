use std::f32;

use super::grid::color::Color;

pub struct Bunker {
    pub color: Color,
    pub x_pos: i16,
    pub y_pos: i16,
    pub radius: i16,
    angle_radians: f32,
    cannon_length: i16,
    pub charge_percent: u8
}

impl Bunker {

    pub fn new_at_nowhere(color: Color) -> Bunker {
        return Bunker::new(color, 4096, 4096);
    }

    pub fn new(color: Color, x_pos: i16, y_pos: i16) -> Bunker {
        return Bunker {
            color,
            x_pos: x_pos,
            y_pos: y_pos,
            radius: 10,
            angle_radians: f32::consts::PI*1.5,
            cannon_length: 20,
            charge_percent: 0
        };
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        return self.color.get_rgba();
    }

    pub fn get_angle_radians(&self) -> f32 {
        return self.angle_radians;
    }

    pub fn change_angle_radians_trim_overflow(&mut self, angle_change: f32) {
        self.angle_radians = f32::min(f32::max(self.angle_radians+angle_change, f32::consts::PI), f32::consts::PI*2.0);
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
