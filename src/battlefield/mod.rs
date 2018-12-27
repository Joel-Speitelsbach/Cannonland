pub mod grid;
pub mod shot;
pub mod bunker;
pub mod shot_type;
mod weapon_depot;

extern crate rand;
use self::rand::Rng;
use std::f32;
use message::{PlayerAction,PlayerID,ChangeWeapon};
use self::grid::particle_type;
use self::grid::Grid;
use self::shot::Shot;
use self::bunker::Bunker;


#[derive(Serialize, Deserialize, Clone)]
pub struct Battlefield {
    pub grid: Grid,
    pub bunkers: Vec<bunker::Bunker>,
    pub shots: Vec<shot::Shot>,
    rand_gen: rand::prng::isaac::IsaacRng,
}


impl Battlefield {
    
    pub fn new() -> Battlefield {
        let mut bunkers = Vec::with_capacity(8);
        for i in 0..8 {
            bunkers.push(bunker::Bunker::new_at_nowhere(
                particle_type::Bunker::from_num(i)
            ));
        }

        let grid = Grid::load_from_file(&"pics/terra_valley.png".to_owned());
        Battlefield { grid: grid, bunkers, shots: Vec::new(),
            rand_gen: rand::FromEntropy::from_entropy(),
        }
    }
    

    pub fn stride(&mut self) {
        self.collide();

        self.grid.stride();
        self.grid.update_bunkers(&mut self.bunkers);
        self.stride_shots();
    }
    

    pub fn execute_action(&mut self, bunker_id: PlayerID, action: &PlayerAction) {
        match *action {
            PlayerAction::TurnCannon { diff_angle: angle } => {
                let bunker = &mut self.bunkers[bunker_id as usize];
                bunker.change_angle_radians_trim_overflow(angle);
            },
            PlayerAction::IncreaseLoad { inc: inc_load } => {
                let bunker = &mut self.bunkers[bunker_id as usize];
                bunker.increment_charge(
                    (inc_load * 100.) as u8
                );
            },
            PlayerAction::CangeWeapon(ChangeWeapon::Next) => {
                self.bunkers[bunker_id as usize].next_weapon();
            },
            PlayerAction::CangeWeapon(ChangeWeapon::Prev) => {
                self.bunkers[bunker_id as usize].prev_weapon();
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
        let x_pos = self.rand_gen.gen::<usize>() % self.grid.width;
        self.bunkers[bunker_id as usize] = bunker::Bunker::new_at_nowhere(
            particle_type::Bunker::from_num(bunker_id)
        );
        self.grid.add_bunker(bunker_id, (x_pos,0));
    }
    

    fn shoot(&mut self, bunker_id: PlayerID) {
        let bunker = &mut self.bunkers[bunker_id as usize];
        if !bunker.alive() {
            return;
        }

        let shoot_pos = bunker.get_shoot_pos_xy();
        let shot = shot::Shot::new(bunker.get_current_weapon(), shoot_pos.0 as f32, shoot_pos.1 as f32, bunker.get_angle_radians(), bunker.get_charge());
        self.shots.push(shot);

        bunker.reset_charge();
    }
    

    fn collide(&mut self) {
        for i in (0..self.shots.len()).rev() {
            let x_pos = self.shots[i].x_pos as usize;
            let y_pos = self.shots[i].y_pos as usize;

            if Battlefield::collide_with_bunkers_true_for_hit(&mut self.bunkers, &self.shots[i])
            || self.grid.collides_at_position(x_pos, y_pos) {
                self.grid.replace_radius_where_possible(self.shots[i].get_impact_target_type(), x_pos, y_pos, self.shots[i].get_impact_radius() as usize);
                self.shots.remove(i);
            }
        }
    }
    

    fn collide_with_bunkers_true_for_hit(bunkers: &mut Vec<Bunker>,
            shot: &Shot) -> bool {
        let mut hit = false;

        for i in (0..bunkers.len()).rev() {
            if !bunkers[i].alive() { continue; }
            if Battlefield::collide_with_bunker_true_for_hit(&mut bunkers[i], shot) {
                hit = true;
            }
        }

        return hit;
    }
    

    fn collide_with_bunker_true_for_hit(bunker: &mut Bunker, shot: &Shot) -> bool {
        if bunker.hit_at(shot.x_pos as i16, shot.y_pos as i16, shot.get_radius()) {
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
}
