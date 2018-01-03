
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub enum Color {
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

impl Color {

    pub fn get_symbol(&self) -> char {
        match self {
            &Color::EMPTY => ' ',
            &Color::BLUR => '\'',
            &Color::DIRT => 'D',
            &Color::ROCK => '#',
            &Color::BETON => 'B',
            &Color::SNOW => '*',
            &Color::WATER => '~',
            &Color::BunkerBlue => '1',
            &Color::BunkerRed => '2',
            &Color::BunkerGreen => '3',
            &Color::BunkerYellow => '4',
            &Color::BunkerTeal => '5',
            &Color::BunkerPurple => '6',
            &Color::BunkerGrey => '7',
            &Color::BunkerOrange => '8'
        }
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            &Color::EMPTY => (0,0,0,0),
            &Color::BLUR => (0,0,0,0),
            &Color::DIRT => (128,64,0,255),
            &Color::ROCK => (128,128,128,255),
            &Color::BETON => (194,194,194,255),
            &Color::SNOW => (255,255,255,255),
            &Color::WATER => (0,0,200,255),
            &Color::BunkerBlue => (0,0,255,255),
            &Color::BunkerRed => (255,0,0,255),
            &Color::BunkerGreen => (0,255,0,255),
            &Color::BunkerYellow => (255,255,0,255),
            &Color::BunkerTeal => (0,255,255,255),
            &Color::BunkerPurple => (255,0,255,255),
            &Color::BunkerGrey => (194,194,194,255),
            &Color::BunkerOrange => (194,160,0,255)
        }
    }

    pub fn can_fall(&self) -> bool {
        match self {
            &Color::EMPTY => false,
            &Color::BLUR => false,
            &Color::DIRT => true,
            &Color::ROCK => true,
            &Color::BETON => false,
            &Color::SNOW => true,
            &Color::WATER => true,
            &Color::BunkerBlue => true,
            &Color::BunkerRed => true,
            &Color::BunkerGreen => true,
            &Color::BunkerYellow => true,
            &Color::BunkerTeal => true,
            &Color::BunkerPurple => true,
            &Color::BunkerGrey => true,
            &Color::BunkerOrange => true
        }
    }

    pub fn can_move_into(&self) -> bool {
        match self {
            &Color::EMPTY => true,
            &Color::BLUR => false,
            &Color::DIRT => false,
            &Color::ROCK => false,
            &Color::BETON => false,
            &Color::SNOW => false,
            &Color::WATER => false,
            &Color::BunkerBlue => false,
            &Color::BunkerRed => false,
            &Color::BunkerGreen => false,
            &Color::BunkerYellow => false,
            &Color::BunkerTeal => false,
            &Color::BunkerPurple => false,
            &Color::BunkerGrey => false,
            &Color::BunkerOrange => false
        }
    }

    pub fn is_bunker(&self) -> bool {
        match self {
            &Color::EMPTY => false,
            &Color::BLUR => false,
            &Color::DIRT => false,
            &Color::ROCK => false,
            &Color::BETON => false,
            &Color::SNOW => false,
            &Color::WATER => false,
            &Color::BunkerBlue => true,
            &Color::BunkerRed => true,
            &Color::BunkerGreen => true,
            &Color::BunkerYellow => true,
            &Color::BunkerTeal => true,
            &Color::BunkerPurple => true,
            &Color::BunkerGrey => true,
            &Color::BunkerOrange => true
        }
    }

}
