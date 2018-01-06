use std::f32;
use std::cmp;

use super::grid::color::Color;

pub struct Bunker {
    color: Color,
    pub x_pos: i16,
    pub y_pos: i16,
    radius: i16,
    angle_radians: f32,
    cannon_length: i16,
    charge: u8,
    max_charge: u8,
    health_points: u8,
    max_health_points: u8
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
            charge: 0,
            max_charge: 100,
            health_points: 100,
            max_health_points: 100
        };
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        return self.color.get_rgba();
    }

    pub fn get_color(&self) -> Color {
        return self.color;
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

    pub fn get_radius(&self) -> i16 {
        return self.radius;
    }

    pub fn get_angle_radians(&self) -> f32 {
        return self.angle_radians;
    }

    pub fn change_angle_radians_trim_overflow(&mut self, angle_change: f32) {
        self.angle_radians = f32::min(f32::max(self.angle_radians+angle_change, f32::consts::PI), f32::consts::PI*2.0);
    }

    pub fn get_charge(&self) -> u8 {
        return self.charge;
    }

    pub fn increment_charge(&mut self, charge_amount: u8) {
        self.charge = cmp::max(self.charge+charge_amount, self.max_charge);
    }

    pub fn reset_charge(&mut self) {
        self.charge = 0;
    }

    pub fn harm(&mut self, harm_amount: u8) {
        self.health_points = cmp::min(self.health_points-harm_amount, 0);
    }

    pub fn heal(&mut self, heal_amount: u8) {
        self.health_points = cmp::max(self.health_points+heal_amount, self.max_health_points);
    }

}
