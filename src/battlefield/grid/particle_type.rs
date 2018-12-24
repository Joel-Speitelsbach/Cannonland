
#[allow(dead_code)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum ParticleType {
    EMPTY,
    BLUR,
    DIRT,
    ROCK,
    BETON,
    SNOW,
    WATER,
    BunkerBlue,
    BunkerRed,
    BunkerGreen,
    BunkerYellow,
    BunkerTeal,
    BunkerPurple,
    BunkerGrey,
    BunkerOrange
}


impl ParticleType {

    pub fn get_symbol(&self) -> char {
        match self {
            &ParticleType::EMPTY => ' ',
            &ParticleType::BLUR => '\'',
            &ParticleType::DIRT => 'D',
            &ParticleType::ROCK => '#',
            &ParticleType::BETON => 'B',
            &ParticleType::SNOW => '*',
            &ParticleType::WATER => '~',
            &ParticleType::BunkerBlue => '1',
            &ParticleType::BunkerRed => '2',
            &ParticleType::BunkerGreen => '3',
            &ParticleType::BunkerYellow => '4',
            &ParticleType::BunkerTeal => '5',
            &ParticleType::BunkerPurple => '6',
            &ParticleType::BunkerGrey => '7',
            &ParticleType::BunkerOrange => '8'
        }
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            &ParticleType::EMPTY => (0,0,0,100),
            &ParticleType::BLUR => (0,0,0,100),
            &ParticleType::DIRT => (128,64,0,255),
            &ParticleType::ROCK => (128,128,128,255),
            &ParticleType::BETON => (194,194,194,255),
            &ParticleType::SNOW => (255,255,255,255),
            &ParticleType::WATER => (0,0,200,255),
            &ParticleType::BunkerBlue => (0,0,255,255),
            &ParticleType::BunkerRed => (255,0,0,255),
            &ParticleType::BunkerGreen => (0,255,0,255),
            &ParticleType::BunkerYellow => (255,255,0,255),
            &ParticleType::BunkerTeal => (0,255,255,255),
            &ParticleType::BunkerPurple => (255,0,255,255),
            &ParticleType::BunkerGrey => (194,194,194,255),
            &ParticleType::BunkerOrange => (194,160,0,255)
        }
    }

    pub fn can_fall(&self) -> bool {
        match self {
            &ParticleType::EMPTY => false,
            &ParticleType::BLUR => false,
            &ParticleType::DIRT => true,
            &ParticleType::ROCK => true,
            &ParticleType::BETON => false,
            &ParticleType::SNOW => true,
            &ParticleType::WATER => true,
            &ParticleType::BunkerBlue => true,
            &ParticleType::BunkerRed => true,
            &ParticleType::BunkerGreen => true,
            &ParticleType::BunkerYellow => true,
            &ParticleType::BunkerTeal => true,
            &ParticleType::BunkerPurple => true,
            &ParticleType::BunkerGrey => true,
            &ParticleType::BunkerOrange => true
        }
    }

    pub fn can_move_into(&self) -> bool {
        match self {
            &ParticleType::EMPTY => true,
            &ParticleType::BLUR => false,
            &ParticleType::DIRT => false,
            &ParticleType::ROCK => false,
            &ParticleType::BETON => false,
            &ParticleType::SNOW => false,
            &ParticleType::WATER => false,
            &ParticleType::BunkerBlue => false,
            &ParticleType::BunkerRed => false,
            &ParticleType::BunkerGreen => false,
            &ParticleType::BunkerYellow => false,
            &ParticleType::BunkerTeal => false,
            &ParticleType::BunkerPurple => false,
            &ParticleType::BunkerGrey => false,
            &ParticleType::BunkerOrange => false
        }
    }

    pub fn is_bunker(&self) -> bool {
        match self {
            &ParticleType::EMPTY => false,
            &ParticleType::BLUR => false,
            &ParticleType::DIRT => false,
            &ParticleType::ROCK => false,
            &ParticleType::BETON => false,
            &ParticleType::SNOW => false,
            &ParticleType::WATER => false,
            &ParticleType::BunkerBlue => true,
            &ParticleType::BunkerRed => true,
            &ParticleType::BunkerGreen => true,
            &ParticleType::BunkerYellow => true,
            &ParticleType::BunkerTeal => true,
            &ParticleType::BunkerPurple => true,
            &ParticleType::BunkerGrey => true,
            &ParticleType::BunkerOrange => true
        }
    }

}
