pub mod particle_test;
mod particle;
pub mod particle_type;  // TODO: make private


use self::particle_type::{ParticleType,Bunker};
use self::particle::Particle;
use super::bunker;
use message::PlayerID;
use sdl2::image::LoadSurface;
use sdl2::pixels::PixelFormatEnum;
use std::cmp;


#[allow(dead_code)]
pub fn create_test_grid() -> Grid {
    let mut grid = Grid::new(800, 500);

    grid.set_rect(ParticleType::DIRT, 40, 40, 80, 80);
    grid.set_rect(ParticleType::ROCK, 20, 100, 140, 210);
    grid.set_rect(ParticleType::SNOW, 300, 20, 390, 140);
    grid.set_rect(ParticleType::WATER, 150, 100, 300, 200);
    grid.set_rect(ParticleType::ROCK, 350, 140, 400, 240);
    grid.set_rect(ParticleType::BETON, 350, 400, 600, 450);

    grid.set_rect(ParticleType::Bunker(Bunker::Blue), 50, 40, 51, 41);
    grid.set_rect(ParticleType::Bunker(Bunker::Red), 150, 40, 151, 41);
    grid.set_rect(ParticleType::Bunker(Bunker::Green), 250, 40, 251, 41);
    grid.set_rect(ParticleType::Bunker(Bunker::Yellow), 350, 40, 351, 41);
    grid.set_rect(ParticleType::Bunker(Bunker::Teal), 450, 40, 451, 41);
    grid.set_rect(ParticleType::Bunker(Bunker::Purple), 550, 40, 551, 41);
    grid.set_rect(ParticleType::Bunker(Bunker::Grey), 650, 40, 651, 41);
    grid.set_rect(ParticleType::Bunker(Bunker::Orange), 750, 40, 751, 41);

    return grid;
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub grid: Vec<Vec<Particle>>
}


impl Grid {

    pub fn new(width: i32, height: i32) -> Grid {
        let mut grid = Vec::with_capacity(height as usize);

        for y in 0..height as usize {
            grid.push(Vec::with_capacity(width as usize));
            for _ in 0..width {
                grid[y].push(Particle::new(ParticleType::EMPTY));
            }
        }

        return Grid{width, height, grid};
    }


    pub fn load_from_file(file_name: &str) -> Grid {
        let surface = sdl2::surface::Surface::from_file(file_name)
            .expect("could not load image");
        let (width, height) = surface.size();
        let (width, height) = (width as i32, height as i32);
        let canvas = surface.into_canvas().unwrap();
        let pixels = canvas.read_pixels(None, PixelFormatEnum::ABGR8888).unwrap();

        let pix = |px: usize| {
            print!("  ");
            for i in 0..4 {
                print!("{} ", pixels[i + px*4]);
            }
            println!();
        };
        println!("grid::load_from_file()");
        for px in 0..6 {
            pix(px);
        }

        let mut grid = Grid::new(width, height);
        for y in 0..height as usize {
            for x in 0..width as usize {
                let pos = (x + y * width as usize) * 4;
                let (r,g,b,a) = (
                    pixels[pos+0],
                    pixels[pos+1],
                    pixels[pos+2],
                    pixels[pos+3],
                );
                if a < 100 {
                    grid.grid[y][x].color = ParticleType::EMPTY.get_rgba();
                    grid.grid[y][x].particle_type = ParticleType::EMPTY;
                } else {
                    grid.grid[y][x].color = (r,g,b,255);
                    grid.grid[y][x].particle_type = ParticleType::ROCK;
                }
            }
        }
        grid
    }


    pub fn add_bunker(&mut self, bunker_num: PlayerID, pos: (i32,i32)) {
        let particle_type = particle_type::Bunker::from_num(bunker_num);
        let (x,y) = pos;
        self.set_pixel(particle_type, x, y)
    }


    pub fn set_rect(&mut self, particle_type: ParticleType, x_start: i32, y_start: i32,
            x_end: i32, y_end: i32) -> () {
        for y in y_start..y_end {
            for x in x_start..x_end {
                self.set_pixel(particle_type, x,y);
            }
        }
    }


    fn set_pixel(&mut self, particle_type: ParticleType, x: i32, y: i32) {
        self.grid
            [y as usize]
            [x as usize].particle_type = particle_type;
    }


    pub fn collides_at_position(&self, x_pos: i32, y_pos: i32) -> bool {
            self.is_inside_grid(x_pos, y_pos) 
        && 
                self.grid
                    [y_pos as usize]
                    [x_pos as usize].particle_type 
            != 
                ParticleType::EMPTY
    }


    fn is_inside_grid(&self, x_pos: i32, y_pos: i32) -> bool {
           (x_pos as u32)  <  self.width  as u32 
        && (y_pos as u32)  <  self.height as u32
    }


    pub fn replace_radius_where_possible(
        &mut self, 
        particle_type: ParticleType, 
        x_pos: i32, 
        y_pos: i32, 
        radius: i32
    ) {
        let x_start = cmp::max(0, x_pos - radius);
        let y_start = cmp::max(0, y_pos - radius);
        let x_end = cmp::min(self.width, x_pos+radius);
        let y_end = cmp::min(self.height, y_pos+radius);

        for y in y_start..y_end {
            for x in x_start..x_end {
                if 
                    (x_pos - x).pow(2) 
                    +
                    (y_pos - y).pow(2)
                < 
                    radius.pow(2)
                {
                    self.grid
                        [y as usize]
                        [x as usize]
                        .replace_if_possible(particle_type);
                }
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
                let (x,y) = (x as usize, y as usize);
                if self.grid[y][x].can_fall() && self.grid[y+1][x].can_move_into() {
                    self.grid[y+1][x] = self.grid[y][x];
                    self.grid[y][x].particle_type = ParticleType::BLUR;
                }
            }
        }
    }


    fn fall_side(&mut self, sign: i32) -> () {
        let x_start: i32;
        let x_end: i32;
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
                let (x,y) = (x as usize, y as usize);
                if self.grid[y][x].can_fall()
                && self.grid[y+1][x].particle_type != ParticleType::BLUR
                && (y == 0 || self.grid[y-1][x].particle_type != ParticleType::BLUR)
                && self.grid[y][(x32+sign) as usize].particle_type == ParticleType::EMPTY
                && self.grid[y+1][(x32+sign) as usize].particle_type == ParticleType::EMPTY
                && self.grid[y+2][(x32+sign) as usize].particle_type == ParticleType::EMPTY {
                    self.grid[y][(x32+sign) as usize] = self.grid[y][x];
                    self.grid[y][x].particle_type = ParticleType::BLUR;
                    self.grid[y+1][(x32+sign) as usize].particle_type = ParticleType::BLUR;
                    self.grid[y+2][(x32+sign) as usize].particle_type = ParticleType::BLUR;
                }
            }
        }
    }


    fn clear_blur(&mut self) -> () {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if self.grid[y][x].particle_type == ParticleType::BLUR {
                    self.grid[y][x].particle_type = ParticleType::EMPTY;
                    self.grid[y][x].color = ParticleType::EMPTY.get_rgba();
                }
            }
        }
    }


    pub fn update_bunkers(&mut self, bunkers: &mut Vec<bunker::Bunker>) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                self.update_bunker_at(x, y, bunkers);
            }
        }
    }


    fn update_bunker_at(
        &mut self,
        x_pos: i32,
        y_pos: i32,
        bunkers: &mut Vec<bunker::Bunker>,
    ) -> ()
    {
        let particle_type = 
            self.grid
            [y_pos as usize]
            [x_pos as usize]
            .particle_type;

        if !particle_type.is_bunker() {
            return
        }

        for bunker in bunkers {
            if bunker.get_color() == particle_type {
                bunker.x_pos = x_pos;
                bunker.y_pos = y_pos;
                bunker.player_active = true;
                return;
            }
        }

        // remove unknown bunker from grid
        self.grid
            [y_pos as usize]
            [x_pos as usize]
            .particle_type 
            = ParticleType::EMPTY;
    }

}
