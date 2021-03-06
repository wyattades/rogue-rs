use crate::config::*;
use crate::game::Game;
use crate::mem::mut_two;

// pub trait Ai {
//   fn action(&self, monster_id: usize, game: &mut Game);
// }

#[derive(Debug, Clone, Copy)]
pub struct Ai {
  pub speed: i32,
}

impl Ai {
  pub fn action(&self, monster_id: usize, game: &mut Game) {
    if game.tick % (self.speed as u64) != 0 {
      return;
    }

    // a basic monster takes its turn. If you can see it, it can see you
    let (monster_x, monster_y) = game.objects[monster_id].pos();
    if game.fov.is_in_fov(monster_x, monster_y)
      && game.objects[PLAYER].fighter.map_or(false, |f| f.hp > 0)
    {
      let (dx, dy) = game.objects[monster_id].delta_to(&game.objects[PLAYER]);

      // if (dx == 0 && dy.abs() <= 1) || (dy == 0 && dx.abs() <= 1) {
      if dy.abs() <= 1 && dx.abs() <= 1 {
        // close enough, attack!
        let (monster, player) = mut_two(monster_id, PLAYER, &mut game.objects);
        monster.attack(player, &mut game.messages);
      } else {
        // move towards player if far away
        let (player_x, player_y) = game.objects[PLAYER].pos();
        game.move_towards(monster_id, player_x, player_y);
      }
    }
  }
}
