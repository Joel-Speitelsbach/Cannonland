use super::shot_type::ShotType;
use super::grid::particle_type::ParticleType;

#[derive(Serialize, Deserialize, Clone)]
pub struct Shot {
    pub shot_type: ShotType,
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_speed: f32,
    pub y_speed: f32
}

impl Shot {

    const GRAVITY: f32 = 0.1;

    pub fn new(
        shot_type: ShotType, 
        x_pos: f32, y_pos: f32, 
        angle_radians: f32, 
        charge: i32) 
        -> Shot 
    {
        let speed = charge as f32 / 10f32;
        Shot{
            shot_type: shot_type,
            x_pos: x_pos,
            y_pos: y_pos,
            x_speed: speed * angle_radians.cos(),
            y_speed: speed * angle_radians.sin()
        }
    }

    pub fn get_radius(&self) -> i32 {
        self.shot_type.get_radius()
    }

    pub fn get_impact_radius(&self) -> f32 {
        self.shot_type.get_impact_radius()
    }

    pub fn get_impact_target_type(&self) -> ParticleType {
        self.shot_type.get_impact_target_type()
    }

    pub fn get_harm(&self) -> i32 {
        self.shot_type.get_harm()
    }

    pub fn stride(&mut self) {
        self.x_pos += self.x_speed;
        self.y_pos += self.y_speed;
        self.y_speed += Shot::GRAVITY;
    }

    pub fn get_angle(&self) -> f32 {
        self.y_speed.atan2(self.x_speed) * (180.0/std::f32::consts::PI) + 90.0
    }

}
