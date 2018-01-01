use grid::color::Color;

#[derive(Copy, Clone)]
pub struct Particle {
    pub color: Color,
    //x_speed: i8,
    //y_speed: i8
}

impl Particle {

    pub fn new(color: Color) -> Particle {
        return Particle{color: color/*, x_speed: 0, y_speed: 0*/};
    }

    pub fn can_fall(&self) -> bool {
        return self.color.can_fall();
    }

    pub fn can_move_into(&self) -> bool {
        return self.color.can_move_into();
    }

}
