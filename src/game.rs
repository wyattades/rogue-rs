use rand::Rng;

use tcod::colors;
use tcod::console::{self, BackgroundFlag, Console, TextAlignment};
use tcod::console::{FontLayout, FontType, Offscreen, Root};
use tcod::input::{self, Event, Key, Mouse};
use tcod::map::Map as FovMap;

use crate::ai::Ai;
use crate::config::*;
use crate::map::Map;
use crate::mem::mut_two;
use crate::messages::{render_bar, Messages};
use crate::object::{Fighter, Object};
use crate::rect::Rect;

pub struct Tcod {
  pub root: Root,
  pub con: Offscreen,
  pub panel: Offscreen,
  pub fov: FovMap,
  pub key: Key,
  pub mouse: Mouse,
}

impl Tcod {
  pub fn new() -> Self {
    tcod::system::set_fps(LIMIT_FPS);

    Self {
      root: Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init(),
      con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
      panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
      fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
      key: Default::default(),
      mouse: Default::default(),
    }
  }

  pub fn check_for_events(&mut self) {
    match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
      Some((_, Event::Mouse(m))) => self.mouse = m,
      Some((_, Event::Key(k))) => self.key = k,
      _ => self.key = Default::default(),
    }
  }
}

fn place_objects(room: &Rect, objects: &mut Vec<Object>) {
  // choose random number of monsters
  let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

  for _ in 0..num_monsters {
    // choose random spot for this monster
    let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
    let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

    let mut monster = if rand::random::<f32>() < 0.8 {
      // 80% chance of getting an orc
      // create an orc
      let mut orc = Object::new(x, y, 'o', colors::DESATURATED_GREEN, "orc", true);
      orc.fighter = Some(Fighter {
        max_hp: 10,
        hp: 10,
        defense: 0,
        power: 3,
        mtype: 1,
      });
      orc.ai = Some(Ai { speed: 5 });

      orc
    } else {
      let mut troll = Object::new(x, y, 'T', colors::DARKER_GREEN, "troll", true);
      troll.fighter = Some(Fighter {
        max_hp: 16,
        hp: 16,
        defense: 1,
        power: 4,
        mtype: 2,
      });
      troll.ai = Some(Ai { speed: 8 });
      troll
    };

    monster.alive = true;
    objects.push(monster);
  }
}

pub struct Player {
  pub prev_position: (i32, i32),
  pub attack_ticks: i32,
}

pub struct Game {
  pub map: Map,
  pub messages: Messages,
  pub objects: Vec<Object>,
  pub player: Player,
  pub tick: u64,
}

impl Game {
  pub fn new(tcod: &mut Tcod) -> Self {
    // create object representing the player
    let mut player = Object::new(0, 0, '@', colors::WHITE, "player", false);
    player.alive = true;
    player.fighter = Some(Fighter {
      max_hp: 30,
      hp: 30,
      defense: 2,
      power: 5,
      mtype: 0,
    });

    let mut game = Game {
      map: Map::new(),
      messages: Messages::new(),
      objects: vec![player],
      tick: 0,
      player: Player {
        prev_position: (-1, -1),
        attack_ticks: 0,
      },
    };

    // populate the FOV map, according to the generated map
    for y in 0..MAP_HEIGHT {
      for x in 0..MAP_WIDTH {
        tcod.fov.set(
          x,
          y,
          !game.map.tile_at(x, y).block_sight,
          !game.map.tile_at(x, y).blocked,
        );
      }
    }

    // populate rooms with objects
    for i in 0..game.map.rooms.len() {
      let room = &game.map.rooms[i];

      if i == 0 {
        let (x, y) = room.center();
        game.objects[PLAYER].set_pos(x, y);
      } else {
        // add enemies/objects
        place_objects(room, &mut game.objects);
      }
    }

    // a warm welcoming message!
    game.messages.add(
      "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
      colors::RED,
    );

    game
  }

  pub fn update(&mut self, tcod: &Tcod) {
    // let monsters take their turn
    if self.objects[PLAYER].alive {
      for id in 1..self.objects.len() {
        // TODO: how to achieve this without Copy?
        if let Some(ai) = self.objects[id].ai {
          ai.action(id, &tcod, self);
        }
      }
    }

    // player's weapon attacks monsters
    if let Some((ax, ay)) = self.objects[PLAYER].attacking {
      let (px, py) = self.objects[PLAYER].pos();
      for id in 1..self.objects.len() {
        if self.objects[id].alive {
          let (x, y) = self.objects[id].pos();
          let (dx, dy) = (px + ax - x, py + ay - y);

          if (dx == 0 && dy.abs() <= 1) || (dx.abs() <= 1 && dy == 0) {
            let (target, source) = mut_two(id, PLAYER, &mut self.objects);
            source.attack(target, &mut self.messages);
          }
        }
      }

      self.player.attack_ticks += 1;
      if self.player.attack_ticks >= 5 {
        self.player.attack_ticks = 0;
        self.objects[PLAYER].stop_attacking();
      }
    }
  }

  pub fn move_by(&mut self, id: usize, dx: i32, dy: i32) {
    let (x, y) = self.objects[id].pos();
    if !self.is_blocked(x + dx, y + dy) {
      self.objects[id].move_by(dx, dy);
    }
  }

  pub fn is_blocked(&self, x: i32, y: i32) -> bool {
    // first test the map tile
    if self.map.tile_at(x, y).blocked {
      return true;
    }

    // now check for any blocking objects
    self
      .objects
      .iter()
      .any(|object| object.blocks && object.pos() == (x, y))
  }

  // Naive method to move Object towards position
  // TODO: use path-finding (e.g. A*)
  pub fn move_towards(&mut self, id: usize, target_x: i32, target_y: i32) {
    // vector from this object to the target, and distance
    let dx = target_x - self.objects[id].x;
    let dy = target_y - self.objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preserving direction), then round it and
    // convert to integer so the movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    self.move_by(id, dx, dy);
  }
}

pub fn render_all(tcod: &mut Tcod, game: &mut Game, fov_recompute: bool) {
  if fov_recompute {
    // recompute FOV if needed (the player moved or something)
    let (x, y) = game.objects[0].pos();
    tcod
      .fov
      .compute_fov(x, y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);
  }

  // go through all tiles, and set their background color
  for y in 0..MAP_HEIGHT {
    for x in 0..MAP_WIDTH {
      let visible = tcod.fov.is_in_fov(x, y);
      let wall = game.map.tile_at(x, y).block_sight;
      let color = match (visible, wall) {
        // outside of field of view:
        (false, true) => COLOR_DARK_WALL,
        (false, false) => COLOR_DARK_GROUND,
        // inside fov:
        (true, true) => COLOR_LIGHT_WALL,
        (true, false) => COLOR_LIGHT_GROUND,
      };

      let mut explored = game.map.tile_at(x, y).explored;

      if visible {
        // since it's visible, explore it
        explored = true;
        game.map.set_explored(x, y);
      }

      if explored {
        // show explored tiles only (any visible tile is explored already)
        tcod
          .con
          .set_char_background(x, y, color, BackgroundFlag::Set);
      }
    }
  }

  let mut to_draw: Vec<_> = game
    .objects
    .iter()
    .filter(|o| tcod.fov.is_in_fov(o.x, o.y))
    .collect();

  // sort so that non-blocking objects come first
  to_draw.sort_by(|o1, o2| o1.blocks.cmp(&o2.blocks));

  // draw all objects in the list
  for object in to_draw {
    object.draw(&mut tcod.con);
  }

  // blit the contents of "con" to the root console
  console::blit(
    &tcod.con,
    (0, 0),
    (MAP_WIDTH, MAP_HEIGHT),
    &mut tcod.root,
    (0, 0),
    1.0,
    1.0,
  );

  // prepare to render the GUI panel
  tcod.panel.set_default_background(colors::BLACK);
  tcod.panel.clear();

  // show the player's stats
  let hp = game.objects[PLAYER].fighter.map_or(0, |f| f.hp);
  let max_hp = game.objects[PLAYER].fighter.map_or(0, |f| f.max_hp);
  render_bar(
    &mut tcod.panel,
    1,
    1,
    BAR_WIDTH,
    "HP",
    hp,
    max_hp,
    colors::LIGHT_RED,
    colors::DARKER_RED,
  );

  game.messages.draw(&mut tcod.panel);

  // display names of objects under the mouse
  tcod.panel.set_default_foreground(colors::LIGHT_GREY);
  tcod.panel.print_ex(
    1,
    0,
    BackgroundFlag::None,
    TextAlignment::Left,
    get_names_under_mouse(tcod.mouse, &game.objects, &tcod.fov),
  );

  // blit the contents of `panel` to the root console
  console::blit(
    &tcod.panel,
    (0, 0),
    (SCREEN_WIDTH, PANEL_HEIGHT),
    &mut tcod.root,
    (0, PANEL_Y),
    1.0,
    1.0,
  );

  tcod.root.flush();
}

fn get_names_under_mouse(mouse: Mouse, objects: &[Object], fov_map: &FovMap) -> String {
  let (x, y) = (mouse.cx as i32, mouse.cy as i32);

  // create a list with the names of all objects at the mouse's coordinates and in FOV
  let names = objects
    .iter()
    .filter(|obj| obj.pos() == (x, y) && fov_map.is_in_fov(obj.x, obj.y))
    .map(|obj| obj.name.clone())
    .collect::<Vec<_>>();

  names.join(", ") // join the names, separated by commas
}

pub fn handle_keys(tcod: &mut Tcod, game: &mut Game) -> bool {
  use tcod::input::KeyCode::*;

  let key = tcod.key;
  match key {
    Key {
      code: Enter,
      alt: true,
      ..
    } => {
      // Alt+Enter: toggle fullscreen
      let fullscreen = tcod.root.is_fullscreen();
      tcod.root.set_fullscreen(!fullscreen);
    }
    Key { code: Escape, .. } => return true, // exit game

    // movement keys
    Key { code: Up, .. } => game.move_by(PLAYER, 0, -1),
    Key { code: Down, .. } => game.move_by(PLAYER, 0, 1),
    Key { code: Left, .. } => game.move_by(PLAYER, -1, 0),
    Key { code: Right, .. } => game.move_by(PLAYER, 1, 0),

    Key { printable: 'w', .. } => game.objects[PLAYER].start_attacking(0, -1),
    Key { printable: 's', .. } => game.objects[PLAYER].start_attacking(0, 1),
    Key { printable: 'a', .. } => game.objects[PLAYER].start_attacking(-1, 0),
    Key { printable: 'd', .. } => game.objects[PLAYER].start_attacking(1, 0),

    _ => {}
  }

  false
}
