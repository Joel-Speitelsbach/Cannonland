use grid;

pub struct Game {
    pub grid: grid::Grid
}

impl Game {

    pub fn new() -> Game {
        return Game{grid: grid::create_grid()};
    }

}
