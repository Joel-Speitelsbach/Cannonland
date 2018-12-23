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

    pub fn get_destruction_radius(&self) -> f32 {
        match self {
            &ShotType::CANNON => 10f32,
            &ShotType::ROCKET => 25f32,
            &ShotType::SNOW => 0f32
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
