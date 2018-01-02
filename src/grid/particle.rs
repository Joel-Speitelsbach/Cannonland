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

    pub fn get_symbol(&self) -> char {
        return self.color.get_symbol();
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        return self.color.get_rgba();
    }

    pub fn can_fall(&self) -> bool {
        return self.color.can_fall();
    }

    pub fn can_move_into(&self) -> bool {
        return self.color.can_move_into();
    }

    pub fn is_player(&self) -> bool {
        return self.color.is_player();
    }

}
