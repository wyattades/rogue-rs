use tcod::console::Console;

#[macro_use]
extern crate generator;

pub mod ai;
pub mod config;
pub mod game;
pub mod map;
pub mod mem;
pub mod messages;
pub mod object;
pub mod rect;

use config::*;
use game::{handle_keys, render_all, Game, Tcod};

fn main() {
  let mut tcod = Tcod::new();

  let mut game = Game::new(&mut tcod);

  while !tcod.root.window_closed() {
    // clear the screen of the previous frame
    tcod.con.clear();

    tcod.check_for_events();

    // render the screen
    let fov_recompute = game.player.prev_position != game.objects[PLAYER].pos();
    render_all(&mut tcod, &mut game, fov_recompute);

    // handle keys and exit game if needed
    let exit = handle_keys(&mut tcod, &mut game);
    if exit {
      break;
    }

    game.update(&tcod);

    game.tick += 1;
  }
}
