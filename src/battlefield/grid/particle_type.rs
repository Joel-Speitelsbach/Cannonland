
use crate::message::PlayerID;


#[allow(dead_code)]
#[derive(Copy,Clone)]
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
    Bunker (Bunker),
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum Bunker {
    Blue,
    Red,
    Green,
    Yellow,
    Teal,
    Purple,
    Grey,
    Orange,
}


impl ParticleType {

    pub fn get_symbol(&self) -> char {
        match self {
            &ParticleType::EMPTY => ' ',
            &ParticleType::BLUR  => '\'',
            &ParticleType::DIRT  => 'D',
            &ParticleType::ROCK  => '#',
            &ParticleType::BETON => 'B',
            &ParticleType::SNOW  => '*',
            &ParticleType::WATER => '~',
            &ParticleType::Bunker(Bunker::Blue)   => '1',
            &ParticleType::Bunker(Bunker::Red)    => '2',
            &ParticleType::Bunker(Bunker::Green)  => '3',
            &ParticleType::Bunker(Bunker::Yellow) => '4',
            &ParticleType::Bunker(Bunker::Teal)   => '5',
            &ParticleType::Bunker(Bunker::Purple) => '6',
            &ParticleType::Bunker(Bunker::Grey)   => '7',
            &ParticleType::Bunker(Bunker::Orange) => '8'
        }
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            &ParticleType::EMPTY => (0,0,0,0),
            &ParticleType::BLUR  => (0,0,0,0),
            &ParticleType::DIRT  => (128,64,0,255),
            &ParticleType::ROCK  => (128,128,128,255),
            &ParticleType::BETON => (194,194,194,255),
            &ParticleType::SNOW  => (255,255,255,255),
            &ParticleType::WATER => (0,0,200,255),
            &ParticleType::Bunker(Bunker::Blue)   => (0,0,255,255),
            &ParticleType::Bunker(Bunker::Red)    => (255,0,0,255),
            &ParticleType::Bunker(Bunker::Green)  => (0,255,0,255),
            &ParticleType::Bunker(Bunker::Yellow) => (255,255,0,255),
            &ParticleType::Bunker(Bunker::Teal)   => (0,255,255,255),
            &ParticleType::Bunker(Bunker::Purple) => (255,0,255,255),
            &ParticleType::Bunker(Bunker::Grey)   => (194,194,194,255),
            &ParticleType::Bunker(Bunker::Orange) => (194,160,0,255)
        }
    }

    pub fn can_fall(&self) -> bool {
        match self {
            &ParticleType::EMPTY     => false,
            &ParticleType::BLUR      => false,
            &ParticleType::DIRT      => true,
            &ParticleType::ROCK      => true,
            &ParticleType::BETON     => false,
            &ParticleType::SNOW      => true,
            &ParticleType::WATER     => true,
            &ParticleType::Bunker(_) => true,
        }
    }

    pub fn can_move_into(&self) -> bool {
        match self {
            &ParticleType::EMPTY => true,
            &ParticleType::BLUR  => false,
            &ParticleType::DIRT  => false,
            &ParticleType::ROCK  => false,
            &ParticleType::BETON => false,
            &ParticleType::SNOW  => false,
            &ParticleType::WATER => false,
            &ParticleType::Bunker(_) => false,
        }
    }

    pub fn is_bunker(&self) -> bool {
        match self {
            &ParticleType::Bunker(_) => true,
            _                        => false,
        }
    }

}

impl Bunker {
    pub fn from_num(num: PlayerID) -> ParticleType {
        match num {
            0 => ParticleType::Bunker(Bunker::Blue),
            1 => ParticleType::Bunker(Bunker::Red),
            2 => ParticleType::Bunker(Bunker::Green),
            3 => ParticleType::Bunker(Bunker::Yellow),
            4 => ParticleType::Bunker(Bunker::Teal),
            5 => ParticleType::Bunker(Bunker::Purple),
            6 => ParticleType::Bunker(Bunker::Grey),
            7 => ParticleType::Bunker(Bunker::Orange),
            x => panic!("particle_type::Bunker::from_num() with {}", x),
        }
    }
}
