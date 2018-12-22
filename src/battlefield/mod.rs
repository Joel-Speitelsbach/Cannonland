pub mod grid;
pub mod shot;
pub mod bunker;

use std::f32;
use message::{PlayerAction,PlayerID};
use self::grid::particle_type::ParticleType;
use self::grid::Grid;


#[derive(Serialize, Deserialize, Clone)]
pub struct Battlefield {
    pub grid: grid::Grid,
    pub bunkers: Vec<bunker::Bunker>,
    pub shots: Vec<shot::Shot>
}

impl Battlefield {

    pub fn new() -> Battlefield {
        let mut bunkers = Vec::with_capacity(8);
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerBlue));
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerRed));
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerGreen));
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerYellow));
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerTeal));
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerPurple));
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerGrey));
        bunkers.push(bunker::Bunker::new_at_nowhere(ParticleType::BunkerOrange));

        let grid = Grid::load_from_file(&"pics/terra_valley_small.png".to_owned());
        return Battlefield{ grid: grid, bunkers, shots: Vec::new() };
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
            PlayerAction::Fire => {
                self.shoot(bunker_id as u8);
            }
            _ => (),
        }
    }

    fn shoot(&mut self, bunker_id: u8) {
        let bunker = &mut self.bunkers[bunker_id as usize];

        let shoot_pos = bunker.get_shoot_pos_xy();
        let shot = shot::Shot::new(shoot_pos.0 as f32, shoot_pos.1 as f32, bunker.get_angle_radians(), bunker.get_charge());
        self.shots.push(shot);

        bunker.reset_charge();
    }

    fn collide(&mut self) {
        for i in (0..self.shots.len()).rev() {
            let x_pos = self.shots[i].x_pos as usize;
            let y_pos = self.shots[i].y_pos as usize;

            if Battlefield::collide_with_bunkers_true_for_hit(&mut self.bunkers, &self.shots[i])
                || self.grid.collides_at_position(x_pos, y_pos) {
                self.grid.delete_radius_leave_out_bunkers(x_pos, y_pos, self.shots[i].destruction_radius as usize);
                self.shots.remove(i);
            }
        }
    }

    fn collide_with_bunkers_true_for_hit(bunkers: &mut Vec<bunker::Bunker>, shot: &shot::Shot) -> bool {
        let mut hit = false;

        for i in (0..bunkers.len()).rev() {
            if Battlefield::collide_with_bunker_true_for_hit(&mut bunkers[i], shot) {
                if bunkers[i].get_health() <= 0 {
                    bunkers.remove(i);
                }
                hit = true;
            }
        }

        return hit;
    }

    fn collide_with_bunker_true_for_hit(bunker: &mut bunker::Bunker, shot: &shot::Shot) -> bool {
        if bunker.hit_at(shot.x_pos as i16, shot.y_pos as i16, shot.radius) {
            bunker.harm(shot.harm);
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
