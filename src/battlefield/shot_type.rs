use super::grid::particle_type::ParticleType;

#[derive(Serialize, Deserialize, Clone)]
pub enum ShotType {
    CANNON,
    ROCKET,
    SNOW
}

impl ShotType {

    pub fn get_radius(&self) -> u8 {
        match self {
            &ShotType::CANNON => 4,
            &ShotType::ROCKET => 4,
            &ShotType::SNOW => 4
        }
    }

    pub fn get_impact_radius(&self) -> f32 {
        match self {
            &ShotType::CANNON => 10f32,
            &ShotType::ROCKET => 25f32,
            &ShotType::SNOW => 20f32
        }
    }

    pub fn get_impact_target_type(&self) -> ParticleType {
        match self {
            &ShotType::CANNON => ParticleType::EMPTY,
            &ShotType::ROCKET => ParticleType::EMPTY,
            &ShotType::SNOW => ParticleType::SNOW
        }
    }

    pub fn get_harm(&self) -> u8 {
        match self {
            &ShotType::CANNON => 10,
            &ShotType::ROCKET => 20,
            &ShotType::SNOW => 0
        }
    }

}
