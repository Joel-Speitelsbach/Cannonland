use super::grid::particle_type::ParticleType;


#[derive(Serialize, Deserialize, Clone)]
pub enum ShotType {
    CANNON,
    ROCKET,
    SNOW
}


impl ShotType {

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            ShotType::CANNON => (125,125,125,255),
            ShotType::ROCKET  => (175,175,175,255),
            ShotType::SNOW  => (255,255,255,255)
        }
    }

    pub fn get_radius(&self) -> i32 {
        match self {
            ShotType::CANNON => 3,
            ShotType::ROCKET => 4,
            ShotType::SNOW => 3
        }
    }

    pub fn get_impact_radius(&self) -> f32 {
        match self {
            ShotType::CANNON => 10.,
            ShotType::ROCKET => 25.,
            ShotType::SNOW => 20.
        }
    }

    pub fn get_impact_target_type(&self) -> ParticleType {
        match self {
            ShotType::CANNON => ParticleType::EMPTY,
            ShotType::ROCKET => ParticleType::EMPTY,
            ShotType::SNOW => ParticleType::SNOW
        }
    }

    pub fn get_harm(&self) -> i32 {
        match self {
            ShotType::CANNON => 10,
            ShotType::ROCKET => 20,
            ShotType::SNOW => 0
        }
    }

}
