use std::{thread, time};
use std::mem;

use grid::color::Color;
use grid::Grid;

pub fn run() {
    println!("this is a particle test");

    println!("{}", mem::size_of::<Color>());

    let mut grid = Grid::new(64, 24);

    grid.set_rect(Color::DIRT, 4, 4, 8, 8);
    grid.set_rect(Color::ROCK, 2, 10, 14, 21);
    grid.set_rect(Color::SNOW, 30, 2, 55, 14);
    grid.set_rect(Color::WATER, 15, 4, 42, 20);
    grid.set_rect(Color::ROCK, 35, 14, 55, 24);

    print_grid(&grid);
    for _ in 0..150 {
        grid.stride();
        print_grid(&grid);
        thread::sleep(time::Duration::from_millis(50));
    }

}

fn print_grid(grid: &Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            print!("{}", grid.grid[y][x].get_symbol());
        }
        println!();
    }
}
