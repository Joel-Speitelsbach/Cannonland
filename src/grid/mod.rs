pub mod particle_test;
pub mod particle;
pub mod color;

use grid::color::Color;
use grid::particle::Particle;

pub fn create_grid() -> Grid {
    let mut grid = Grid::new(800, 500);

    grid.set_rect(Color::DIRT, 40, 40, 80, 80);
    grid.set_rect(Color::ROCK, 20, 100, 140, 210);
    grid.set_rect(Color::SNOW, 300, 20, 390, 140);
    grid.set_rect(Color::WATER, 150, 100, 300, 200);
    grid.set_rect(Color::ROCK, 350, 140, 400, 240);
    grid.set_rect(Color::BETON, 350, 400, 600, 450);

    return grid;
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Particle>>
}

impl Grid {

    pub fn new(width: usize, height: usize) -> Grid {
        let mut grid_vec = Vec::with_capacity(height);

        for y in 0..height {
            grid_vec.push(Vec::with_capacity(width));
            for _ in 0..width {
                grid_vec[y].push(Particle::new(Color::EMPTY));
            }
        }

        return Grid{width: width, height: height, grid: grid_vec};
    }

    pub fn set_rect(&mut self, color: Color, x_start: usize, y_start: usize, x_end: usize, y_end: usize) -> () {
        for y in y_start..y_end {
            for x in x_start..x_end {
                self.grid[y][x].color = color;
            }
        }
    }

    pub fn stride(&mut self) -> () {
        self.fall_down();
        self.fall_side(1);
        self.fall_side(-1);
        self.clear_blur();
    }

    fn fall_down(&mut self) -> () {
        for y in (0..self.height-1).rev() {
            for x in 0..self.width {
                if self.grid[y][x].can_fall() && self.grid[y+1][x].can_move_into() {
                    self.grid[y+1][x] = self.grid[y][x];
                    self.grid[y][x].color = Color::BLUR;
                }
            }
        }
    }

    fn fall_side(&mut self, sign: i8) -> () {
        let sign = sign as i32;

        let x_start: usize;
        let x_end: usize;
        if sign == 1 {
            x_start = 0;
            x_end = self.width-1;
        } else if sign == -1 {
            x_start = 1;
            x_end = self.width;
        } else {
            panic!("sign must be 1 or -1 but is {}", sign);
        }

        for y in 0..self.height-2 {
            for x in x_start..x_end {
                let x32 = x as i32;
                if self.grid[y][x].can_fall()
                && self.grid[y+1][x].color != Color::BLUR
                && (y == 0 || self.grid[y-1][x].color != Color::BLUR)
                && self.grid[y][(x32+sign) as usize].color == Color::EMPTY
                && self.grid[y+1][(x32+sign) as usize].color == Color::EMPTY
                && self.grid[y+2][(x32+sign) as usize].color == Color::EMPTY {
                    self.grid[y][(x32+sign) as usize] = self.grid[y][x];
                    self.grid[y][x].color = Color::BLUR;
                    self.grid[y+1][(x32+sign) as usize].color = Color::BLUR;
                    self.grid[y+2][(x32+sign) as usize].color = Color::BLUR;
                }
            }
        }
    }

    fn clear_blur(&mut self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x].color == Color::BLUR {
                    self.grid[y][x].color = Color::EMPTY;
                }
            }
        }
    }

}
