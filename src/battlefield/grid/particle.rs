use super::particle_type::ParticleType;


#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub color: (u8,u8,u8,u8,),
    //x_speed: i8,
    //y_speed: i8
}


impl Particle {

    pub fn new(particle_type: ParticleType) -> Particle {
        return Particle::new_with_color(particle_type, particle_type.get_rgba());
    }

    pub fn new_with_color(particle_type: ParticleType, color: (u8,u8,u8,u8,)) -> Particle {
        return Particle{
            particle_type,
            color,
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

    #[allow(dead_code)]
    pub fn is_bunker(&self) -> bool {
        return self.particle_type.is_bunker();
    }

    pub fn replace_if_possible(&mut self, replacement: ParticleType) -> () {
        if self.replacement_possible(replacement) {
            self.particle_type = replacement;
            self.color = replacement.get_rgba();
        }
    }

    fn replacement_possible(&self, replacement: ParticleType) -> bool {
        if self.particle_type.is_bunker() {
            return false;
        }
        if replacement == ParticleType::EMPTY {
            return true;
        }
        return self.particle_type.can_move_into();
    }

}
