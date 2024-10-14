pub mod grid;
pub mod shot;
pub mod bunker;
pub mod shot_type;
mod weapon_depot;

extern crate rand;
extern crate rand_isaac;
use self::rand::Rng;
use std::collections::HashMap;
use std::f32;
use crate::message::{PlayerAction,PlayerID,ChangeWeapon};
use self::grid::particle_type;
use self::grid::Grid;
use self::shot::Shot;
use self::bunker::Bunker;
use crate::sound::Sound;


pub const SIZE: (i32,i32) = (800,500);


#[derive(Serialize, Deserialize, Clone)]
pub struct Battlefield {
    pub grid: Grid,
    pub bunkers: HashMap<PlayerID,bunker::Bunker>,
    pub shots: Vec<shot::Shot>,
    rand_gen: rand_isaac::IsaacRng,
}


impl Battlefield {

    pub fn new() -> Battlefield {
        let grid = Grid::load_from_file("pics/terra_valley.png");
        let battlefield = Battlefield {
            grid,
            bunkers: HashMap::new(),
            shots: Vec::new(),
            rand_gen: rand::SeedableRng::from_entropy(),
        };
        assert_eq!(battlefield.size(),SIZE,
            "size of battlefield file (first form) is not correct (second form)");
        battlefield
    }


    pub fn stride(&mut self, sound: &Sound) {
        self.collide(&sound);

        self.grid.stride();
        self.grid.update_bunkers(&mut self.bunkers);
        self.stride_shots();
    }


    pub fn execute_action(&mut self, bunker_id: PlayerID, action: &PlayerAction, sound: &Sound) {
        match *action {
            PlayerAction::TurnCannon { diff_angle: angle } => {
                let bunker = &mut self.bunkers.get_mut(&bunker_id).unwrap();
                bunker.change_angle_radians_trim_overflow(angle);
            },
            PlayerAction::IncreaseLoad { inc: inc_load } => {
                let bunker = &mut self.bunkers.get_mut(&bunker_id).unwrap();
                bunker.increment_charge(
                    (inc_load * 100.) as i32
                );
            },
            PlayerAction::CangeWeapon(ChangeWeapon::Next) => {
                self.bunkers.get_mut(&bunker_id).unwrap().next_weapon();
            },
            PlayerAction::CangeWeapon(ChangeWeapon::Prev) => {
                self.bunkers.get_mut(&bunker_id).unwrap().prev_weapon();
            },
            PlayerAction::Fire => {
                self.shoot(bunker_id, sound);
            },
            PlayerAction::NewBunker => {
                self.new_bunker(bunker_id)
            },
            PlayerAction::DeleteBunker => {
                self.delete_bunker(bunker_id)
            },
        }
    }


    fn delete_bunker(&mut self, bunker_id: PlayerID) {
        self.bunkers.remove(&bunker_id);
    }


    fn new_bunker(&mut self, bunker_id: PlayerID) {
        let width = self.grid.width;
        let min_dist = width / (self.number_of_bunkers() * 2 + 1);
        let x_pos = 'search: loop {
            let x_pos = self.rand_gen.gen_range(0, width);
            for (_,bunker) in &self.bunkers {
                let collide = (bunker.x_pos - x_pos).abs() < min_dist;
                if collide {
                    continue 'search;
                }
            }
            break x_pos;
        };

        self.grid.add_bunker(bunker_id, (x_pos,0));
        self.bunkers.insert(bunker_id, Bunker::new_at_nowhere(
            particle_type::Bunker::from_num(bunker_id)));
        self.grid.update_bunkers(&mut self.bunkers);
    }


    fn number_of_bunkers(&self) -> i32 {
        self.bunkers.len() as i32
    }


    fn shoot(&mut self, bunker_id: PlayerID, sound: &Sound) {
        let bunker = self.bunkers.get_mut(&bunker_id).unwrap();
        if !bunker.is_alive() {
            return;
        }

        let shoot_pos = bunker.get_shoot_pos();
        let shot = shot::Shot::new(
            bunker.get_current_weapon(), 
            shoot_pos.0 as f32, 
            shoot_pos.1 as f32, 
            bunker.get_angle_radians(), 
            bunker.get_charge()
        );
        sound.play(&shot.shot_type.get_shoot_sound());
        self.shots.push(shot);

        bunker.reset_charge();
    }
 

    fn collide(&mut self, sound: &Sound) {
        let mut shots_to_remove = vec!();
        for i in (0..self.shots.len()).rev() {
            let shot = &self.shots[i];
            let x_pos = shot.x_pos as i32;
            let y_pos = shot.y_pos as i32;

            let mut shot_collides = false;

            if Battlefield::shot_collides_with_bunkers(&self.bunkers, &shot) {
                shot_collides = true;
                Battlefield::harm_bunkers(&mut self.bunkers, &shot, &mut self.grid);
            } else if self.grid.collides_at_position(x_pos, y_pos) {
                shot_collides = true;
            } else if shot.collides_with_shot(&self.shots) {
                shot_collides = true;
            }

            if shot_collides {
                sound.play(&shot.shot_type.get_impact_sound());
                self.grid.replace_radius_where_possible(
                    shot.get_impact_target_type(), 
                    x_pos, y_pos, 
                    shot.get_impact_radius() as i32
                );
                shots_to_remove.push(i);
            }
        }
        for i in shots_to_remove {
            self.shots.remove(i);
        }
    }

    fn shot_collides_with_bunkers(bunkers: &HashMap<PlayerID,Bunker>, shot: &Shot) -> bool {
        for i in (0..bunkers.len() as i32).rev() {
            if !bunkers[&i].is_alive() {
                continue;
            }
            if bunkers[&i].would_harm_in_radius(
                    shot.x_pos as i32,
                    shot.y_pos as i32,
                    shot.get_radius())
            {
                return true;
            }
        }
        false
    }

    fn harm_bunkers(bunkers: &mut HashMap<PlayerID,Bunker>, shot: &Shot, grid: &mut Grid) {
        for i in (0..bunkers.len() as i32).rev() {
            if !bunkers[&i].is_alive() {
                continue;
            }
            bunkers.get_mut(&i).unwrap().harm_if_in_radius(
                shot.x_pos               as i32, 
                shot.y_pos               as i32, 
                shot.get_impact_radius() as i32, 
                shot.get_harm(),
                grid,
            );
        }
    }


    fn stride_shots(&mut self) {
        for i in (0..self.shots.len()).rev() {
            self.shots[i].stride();
            if self.shots[i].y_pos > self.grid.height as f32 + 100f32 {
                self.shots.remove(i);
            }
        }
    }

    pub fn size(&self) -> (i32,i32) {
        (self.grid.width as i32, self.grid.height as i32)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::shot_type::ShotType;
    use super::particle_type::ParticleType;

    #[test]
    fn collide() {
        let mut battlefield = Battlefield::new();
        battlefield.new_bunker(1);

        let bunker_start_health;
        let bunker_radius;
        {
            let bunker = battlefield.bunkers.get_mut(&0).unwrap();
            bunker.x_pos = 100;
            bunker.y_pos = 100;
            bunker_start_health = bunker.get_health();
            bunker_radius = bunker.get_radius();
        }

        let x_pos_of_nearest_impact_without_harm = 100.0 + bunker_radius as f32 + ShotType::ROCKET.get_impact_radius();

        battlefield.grid.grid.get_mut(100).unwrap().get_mut(x_pos_of_nearest_impact_without_harm as usize).unwrap().particle_type = ParticleType::BETON;
        battlefield.shots.push(Shot::new(ShotType::ROCKET, x_pos_of_nearest_impact_without_harm, 100.0, 0.0, 0));
        battlefield.collide(&Sound::stub());
        assert_eq!(battlefield.bunkers.get_mut(&0).unwrap().get_health(), bunker_start_health);

        battlefield.grid.grid.get_mut(100).unwrap().get_mut(x_pos_of_nearest_impact_without_harm as usize - 1).unwrap().particle_type = ParticleType::BETON;
        battlefield.shots.push(Shot::new(ShotType::ROCKET, x_pos_of_nearest_impact_without_harm-1.0, 100.0, 0.0, 0));
        battlefield.collide(&Sound::stub());
        assert_eq!(battlefield.bunkers.get_mut(&0).unwrap().get_health(), bunker_start_health - ShotType::ROCKET.get_harm());
    }
}
