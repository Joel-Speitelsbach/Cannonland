
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Color {
    EMPTY,
    BLUR,
    DIRT,
    ROCK,
    SNOW,
    WATER
}

impl Color {

    pub fn get_symbol(&self) -> char {
        match self {
            &Color::EMPTY => ' ',
            &Color::BLUR => '\'',
            &Color::DIRT => 'D',
            &Color::ROCK => '#',
            &Color::SNOW => '*',
            &Color::WATER => '~'
        }
    }

    pub fn can_fall(&self) -> bool {
        match self {
            &Color::EMPTY => false,
            &Color::BLUR => false,
            &Color::DIRT => true,
            &Color::ROCK => true,
            &Color::SNOW => true,
            &Color::WATER => true
        }
    }

    pub fn can_move_into(&self) -> bool {
        match self {
            &Color::EMPTY => true,
            &Color::BLUR => false,
            &Color::DIRT => false,
            &Color::ROCK => false,
            &Color::SNOW => false,
            &Color::WATER => false
        }
    }

}
