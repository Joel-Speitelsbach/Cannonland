
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Color {
    EMPTY,
    BLUR,
    DIRT,
    ROCK,
    BETON,
    SNOW,
    WATER,
    PlayerBlue,
    PlayerRed,
    PlayerGreen,
    PlayerYellow,
    PlayerTeal,
    PlayerPurple,
    PlayerGrey,
    PlayerOrange
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
            &Color::PlayerBlue => '1',
            &Color::PlayerRed => '2',
            &Color::PlayerGreen => '3',
            &Color::PlayerYellow => '4',
            &Color::PlayerTeal => '5',
            &Color::PlayerPurple => '6',
            &Color::PlayerGrey => '7',
            &Color::PlayerOrange => '8'
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
            &Color::PlayerBlue => (0,0,255,255),
            &Color::PlayerRed => (255,0,0,255),
            &Color::PlayerGreen => (0,255,0,255),
            &Color::PlayerYellow => (255,255,0,255),
            &Color::PlayerTeal => (0,255,255,255),
            &Color::PlayerPurple => (255,0,255,255),
            &Color::PlayerGrey => (194,194,194,255),
            &Color::PlayerOrange => (194,160,0,255)
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
            &Color::PlayerBlue => true,
            &Color::PlayerRed => true,
            &Color::PlayerGreen => true,
            &Color::PlayerYellow => true,
            &Color::PlayerTeal => true,
            &Color::PlayerPurple => true,
            &Color::PlayerGrey => true,
            &Color::PlayerOrange => true
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
            &Color::PlayerBlue => false,
            &Color::PlayerRed => false,
            &Color::PlayerGreen => false,
            &Color::PlayerYellow => false,
            &Color::PlayerTeal => false,
            &Color::PlayerPurple => false,
            &Color::PlayerGrey => false,
            &Color::PlayerOrange => false
        }
    }

}
