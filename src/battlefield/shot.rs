

#[derive(Serialize, Deserialize)]
pub struct Shot {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub radius: u8,
    pub destruction_radius: f32,
    pub harm: u8
}

impl Shot {

    const GRAVITY: f32 = 0.1;

    pub fn new(x_pos: f32, y_pos: f32, angle_radians: f32, charge: u8) -> Shot {
        let speed = charge as f32 / 10f32;
        return Shot{
            x_pos: x_pos,
            y_pos: y_pos,
            x_speed: speed * angle_radians.cos(),
            y_speed: speed * angle_radians.sin(),
            radius: 4,
            destruction_radius: 10f32,
            harm: 10
        };
    }

    pub fn stride(&mut self) {
        self.x_pos += self.x_speed;
        self.y_pos += self.y_speed;
        self.y_speed += Shot::GRAVITY;
    }

}
