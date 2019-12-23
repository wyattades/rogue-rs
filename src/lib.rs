use wasm_bindgen::prelude::wasm_bindgen;

pub mod ai;
pub mod colors;
pub mod config;
pub mod draw;
pub mod fov;
pub mod game;
pub mod map;
pub mod mem;
pub mod object;
pub mod rect;
pub mod ui;

use config::*;
use draw::Tcod;
use game::Game;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//   ( $( $t:tt )* ) => {
//       web_sys::console::log_1(&format!( $( $t )* ).into());
//   }
// }

#[wasm_bindgen]
pub struct GameData {
  game: Game,
  tcod: Tcod,
  key: i32,
  mouse: (i32, i32),
}

#[wasm_bindgen]
impl GameData {
  pub fn new() -> Self {
    Self {
      tcod: Tcod::new(SCREEN_WIDTH, SCREEN_HEIGHT),
      game: Game::new(),
      key: 0,
      mouse: (0, 0),
    }
  }

  pub fn move_mouse(&mut self, x: f32, y: f32) {
    self.mouse = (
      (x * SCREEN_WIDTH as f32) as i32,
      (y * SCREEN_HEIGHT as f32) as i32,
    );
  }

  pub fn press_key(&mut self, key: i32) {
    self.key = key;
    // self.game.handle_key_press(key);
  }

  pub fn tick(&mut self) {
    self.game.handle_keys(self.key);
    self.key = 0;
    self.game.update();
    self.game.render(&mut self.tcod, self.mouse);
  }

  pub fn render(&self) -> String {
    self.tcod.render_to_string()
  }
}
