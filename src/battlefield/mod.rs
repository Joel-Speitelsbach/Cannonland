pub mod grid;
pub mod shot;
pub mod bunker;
pub mod shot_type;
mod weapon_depot;

extern crate rand;
use self::rand::Rng;
use std::f32;
use message::{PlayerAction,PlayerID,ChangeWeapon};
use self::grid::Grid;
use self::grid::particle_type::ParticleType;
use self::shot::Shot;
use self::bunker::{Bunker,Player};


#[derive(Serialize, Deserialize, Clone)]
pub struct Battlefield {
    pub grid: Grid,
    pub bunkers: Vec<Player>,
    pub shots: Vec<shot::Shot>,
    rand_gen: rand::prng::isaac::IsaacRng,
}


impl Battlefield {
    
    pub fn new() -> Battlefield {
        let bunkers = vec![Player::NotAlive; 8];

        let grid = Grid::load_from_file("pics/terra_valley.png");
        Battlefield {
            grid,
            bunkers,
            shots: Vec::new(),
            rand_gen: rand::FromEntropy::from_entropy(),
        }
    }
    

    pub fn stride(&mut self) {
        self.collide();

        self.grid.stride();
        self.grid.update_bunkers(&mut self.bunkers);
        self.stride_shots();
    }


    pub fn remove_dead_bunkers(&mut self) {
        for i in 0..self.bunkers.len() {
            if let Player::Alive(bunker) = self.bunkers[i].clone() {
                if !bunker.alive() {
                    let x = bunker.x_pos as usize;
                    let y = bunker.y_pos as usize;
                    self.bunkers[i] = Player::NotAlive;
                    self.grid.set_pixel(ParticleType::EMPTY, x, y);
                }
            }
        }

    }
    

    pub fn execute_action(&mut self, bunker_id: PlayerID, action: &PlayerAction) {
        match *action {
            PlayerAction::TurnCannon { diff_angle: angle } => {
                let bunker = &mut self.bunkers[bunker_id as usize];
                let bunker = match bunker {
                    Player::NotAlive => return,
                    Player::Alive(bunker) => bunker,
                };
                bunker.change_angle_radians_trim_overflow(angle);
            },
            PlayerAction::IncreaseLoad { inc: inc_load } => {
                let bunker = &mut self.bunkers[bunker_id as usize];
                let bunker = match bunker {
                    Player::NotAlive => return,
                    Player::Alive(bunker) => bunker,
                };
                bunker.increment_charge(
                    (inc_load * 100.) as u8
                );
            },
            PlayerAction::CangeWeapon(ChangeWeapon::Next) => {
                let bunker = &mut self.bunkers[bunker_id as usize];
                let bunker = match bunker {
                    Player::NotAlive => return,
                    Player::Alive(bunker) => bunker,
                };
                bunker.next_weapon();
            },
            PlayerAction::CangeWeapon(ChangeWeapon::Prev) => {
                let bunker = &mut self.bunkers[bunker_id as usize];
                let bunker = match bunker {
                    Player::NotAlive => return,
                    Player::Alive(bunker) => bunker,
                };
                bunker.prev_weapon();
            },
            PlayerAction::Fire => {
                self.shoot(bunker_id);
            },
            PlayerAction::NewBunker => {
                self.new_bunker(bunker_id)
            },
        }
    }
    
    
    fn new_bunker(&mut self, bunker_id: PlayerID) {
        let width = self.grid.width as i16;
        let min_dist = width / (self.number_of_bunkers() * 2 + 1);
        let x_pos = 'search: loop {
            let x_pos = self.rand_gen.gen_range::<i16>(0, width);
            'bunker: for bunker in &self.bunkers {
                let bunker = match bunker {
                    Player::Alive(bunker) => bunker,
                    Player::NotAlive => continue 'bunker,
                };
                let collide = (bunker.x_pos - x_pos as i16).abs() < min_dist;
                if collide {
                    continue 'search;
                }
            }
            break x_pos;
        };

        self.grid.add_bunker(bunker_id, (x_pos as usize,0));
        self.grid.update_bunkers(&mut self.bunkers);
    }
    

    fn shoot(&mut self, bunker_id: PlayerID) {
        let bunker = &mut self.bunkers[bunker_id as usize];
        let bunker = match bunker {
                Player::NotAlive => return,
                Player::Alive(bunker) => bunker,
            };

        let shoot_pos = bunker.get_shoot_pos_xy();
        let shot = shot::Shot::new(
                bunker.get_current_weapon(),
                shoot_pos.0 as f32, shoot_pos.1 as f32,
                bunker.get_angle_radians(),
                bunker.get_charge()
            );
        self.shots.push(shot);

        bunker.reset_charge();
    }


    fn collide(&mut self) {
        for i in (0..self.shots.len()).rev() {
            let x_pos = self.shots[i].x_pos as usize;
            let y_pos = self.shots[i].y_pos as usize;

            if Battlefield::collide_with_bunkers_true_for_hit(&mut self.bunkers, &self.shots[i])
            || self.grid.collides_at_position(x_pos, y_pos) {
                self.grid.replace_radius_where_possible(
                    self.shots[i].get_impact_target_type(),
                    x_pos,
                    y_pos,
                    self.shots[i].get_impact_radius() as usize
                );
                self.shots.remove(i);
                self.remove_dead_bunkers();
            }
        }
    }
    

    fn collide_with_bunkers_true_for_hit(
        bunkers: &mut Vec<Player>,
        shot: &Shot,
        ) -> bool
    {
        let mut hit = false;

        for player in bunkers {
            let bunker = match player {
                Player::NotAlive => continue,
                Player::Alive(b) => b,
            };
            if Battlefield::collide_with_bunker_true_for_hit(bunker, shot) {
                hit = true;
            }
        }

        return hit;
    }
    

    fn collide_with_bunker_true_for_hit(bunker: &mut Bunker, shot: &Shot) -> bool {
        if bunker.hit_at(
            shot.x_pos as i16,
            shot.y_pos as i16,
            shot.get_radius())
        {
            bunker.harm(shot.get_harm());
            return true;
        }
        return false;
    }
    

    fn stride_shots(&mut self) {
        for i in (0..self.shots.len()).rev() {
            self.shots[i].stride();
            if self.shots[i].y_pos > self.grid.height as f32 + 100f32 {
                self.shots.remove(i);
            }
        }
    }

    fn number_of_bunkers(&self) -> i16 {
        let mut counter = 0;
        for bunker in &self.bunkers {
            if let Player::Alive(_) = bunker {
                counter += 1;
            }
        }
        counter
    }
}
