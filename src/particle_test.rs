
pub fn run() {
    println!("this is a particle test");

    let grid = Grid{grid: [[Particle{color: 1, xSpeed: 0, ySpeed: 0}; 4]; 4]};

    for y in 0..4 {
        for x in 0..4 {
            print!("{}", grid.grid[y][x].color);
        }
        println!();
    }
}

struct Grid {
    grid: [[Particle; 4]; 4]
}

#[derive(Copy, Clone)]
struct Particle {
    color: i8,
    xSpeed: i8,
    ySpeed: i8
}
