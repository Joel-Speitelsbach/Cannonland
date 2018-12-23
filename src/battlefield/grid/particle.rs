use super::particle_type::ParticleType;
use sdl2::pixels::Color;


#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub color: (u8,u8,u8,u8,),
    //x_speed: i8,
    //y_speed: i8
}


impl Particle {

    pub fn new(particle_type: ParticleType, color: Color) -> Particle {
        return Particle{
            particle_type: particle_type,
            color: color.rgba(),
            /*, x_speed: 0, y_speed: 0*/
        };
    }

    pub fn get_symbol(&self) -> char {
        return self.particle_type.get_symbol();
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        return self.color;
    }

    pub fn can_fall(&self) -> bool {
        return self.particle_type.can_fall();
    }

    pub fn can_move_into(&self) -> bool {
        return self.particle_type.can_move_into();
    }

    pub fn is_bunker(&self) -> bool {
        return self.particle_type.is_bunker();
    }

    pub fn empty_if_not_bunker(&mut self) -> () {
        if !self.particle_type.is_bunker() {
            self.particle_type = ParticleType::EMPTY;
            self.color = (0,0,0,100);
        }
    }

}
