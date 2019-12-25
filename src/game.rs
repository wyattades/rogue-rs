use pcg_rand::{seeds::PcgSeeder, Pcg32Basic};
use rand::{Rng, SeedableRng};

use crate::ai::Ai;
use crate::colors;
use crate::config::*;
use crate::draw::Tcod;
use crate::fov::FOV;
use crate::map::Map;
use crate::mem::mut_two;
use crate::object::{Fighter, Object};
use crate::rect::Rect;
use crate::ui::{render_bar, Messages};

fn place_objects<R: Rng>(rng: &mut R, room: &Rect, objects: &mut Vec<Object>) {
  // choose random number of monsters
  let num_monsters = rng.gen_range(0, MAX_ROOM_MONSTERS + 1);

  for _ in 0..num_monsters {
    // choose random spot for this monster
    let x = rng.gen_range(room.x1 + 1, room.x2);
    let y = rng.gen_range(room.y1 + 1, room.y2);

    let mut monster = if rng.gen::<f32>() < 0.8 {
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
  pub rng: Pcg32Basic,
  pub map: Map,
  pub messages: Messages,
  pub fov: FOV,
  pub objects: Vec<Object>,
  pub player: Player,
  pub tick: u64,
}

impl Game {
  pub fn new(seed: u64) -> Self {
    // random number generator
    let mut rng = Pcg32Basic::from_seed(PcgSeeder::seed(seed));

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
      map: Map::new(&mut rng),
      messages: Messages::new(),
      fov: FOV::new(MAP_WIDTH, MAP_HEIGHT),
      objects: vec![player],
      rng: rng,
      tick: 0,
      player: Player {
        prev_position: (-1, -1),
        attack_ticks: 0,
      },
    };

    // populate the FOV map, according to the generated map
    for y in 0..MAP_HEIGHT {
      for x in 0..MAP_WIDTH {
        game.fov.set(
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
        place_objects(&mut game.rng, room, &mut game.objects);
      }
    }

    // a warm welcoming message!
    game.messages.add(
      "Welcome stranger! Prepare to perish in the Tombs of the Ancient Kings.",
      colors::RED,
    );

    game
  }

  pub fn update(&mut self) {
    // recompute FOV if needed (the player moved or something)
    if self.player.prev_position != self.objects[PLAYER].pos() {
      let (x, y) = self.objects[PLAYER].pos();

      self.fov.compute_fov(x, y, TORCH_RADIUS, FOV_LIGHT_WALLS);
    }
    self.player.prev_position = self.objects[PLAYER].pos();

    // let monsters take their turn
    if self.objects[PLAYER].alive {
      for id in 1..self.objects.len() {
        // TODO: how to achieve this without Copy?
        if let Some(ai) = self.objects[id].ai {
          ai.action(id, self);
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

          if dx == 0 && dy == 0 {
            // if (dx == 0 && dy.abs() <= 1) || (dx.abs() <= 1 && dy == 0) {
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

    self.tick += 1;
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

  pub fn handle_keys(&mut self, key_code: i32) -> bool {
    if !self.objects[PLAYER].alive {
      return false;
    };

    match key_code {
      38 => self.objects[PLAYER].start_attacking(0, -1),
      40 => self.objects[PLAYER].start_attacking(0, 1),
      37 => self.objects[PLAYER].start_attacking(-1, 0),
      39 => self.objects[PLAYER].start_attacking(1, 0),

      87 => self.move_by(PLAYER, 0, -1),
      83 => self.move_by(PLAYER, 0, 1),
      65 => self.move_by(PLAYER, -1, 0),
      68 => self.move_by(PLAYER, 1, 0),
      _ => {}
    }

    false
  }

  // pub fn handle_keys(&mut self, key_codes: &[i32]) {
  //   for key_code in key_codes {
  //     match key_code {
  //       _ => {}
  //     }
  //   }
  // }

  pub fn render(&mut self, tcod: &mut Tcod, mouse: (i32, i32)) {
    tcod.background(colors::BLACK);
    tcod.clear_chars();

    // go through all tiles, and set their background color
    for y in 0..MAP_HEIGHT {
      for x in 0..MAP_WIDTH {
        let visible = self.fov.is_in_fov(x, y);

        let mut explored = self.map.tile_at(x, y).explored;

        if visible {
          // since it's visible, explore it
          explored = true;
          // TODO: shouldn't this be in `update`?
          self.map.set_explored(x, y);
        }

        if explored {
          let wall = self.map.tile_at(x, y).block_sight;
          let color = match (visible, wall) {
            // outside of field of view:
            (false, true) => COLOR_DARK_WALL,
            (false, false) => COLOR_DARK_GROUND,
            // inside fov:
            (true, true) => COLOR_LIGHT_WALL,
            (true, false) => COLOR_LIGHT_GROUND,
          };

          // show explored tiles only (any visible tile is explored already)
          tcod.put_char_background(x, y, color);
        }
      }
    }

    let mut to_draw: Vec<_> = self
      .objects
      .iter()
      .filter(|o| self.fov.is_in_fov(o.x, o.y))
      .collect();

    // sort so that non-blocking objects come first
    to_draw.sort_by(|o1, o2| {
      let a = o1.alive.cmp(&o2.alive);
      if a == std::cmp::Ordering::Equal {
        o1.blocks.cmp(&o2.blocks)
      } else {
        a
      }
    });

    // draw all objects in the list
    for object in to_draw {
      object.draw(tcod);
    }

    // // prepare to render the GUI panel
    // tcod.panel.set_default_background(colors::BLACK);
    // tcod.panel.clear();

    // show the player's stats
    let hp = self.objects[PLAYER].fighter.map_or(0, |f| f.hp);
    let max_hp = self.objects[PLAYER].fighter.map_or(0, |f| f.max_hp);
    render_bar(
      tcod,
      1,
      1,
      BAR_WIDTH,
      "HP",
      hp,
      max_hp,
      colors::LIGHT_RED,
      colors::DARKER_RED,
    );

    self.messages.draw(tcod);

    // display names of objects under the mouse
    tcod.stroke(colors::LIGHT_GREY);
    tcod.print_rect(
      &self.get_names_at(mouse),
      1,
      PANEL_Y + PANEL_HEIGHT - 2,
      BAR_WIDTH,
      0,
    );
  }

  fn get_names_at(&self, (x, y): (i32, i32)) -> String {
    // create a list with the names of all objects at the mouse's coordinates and in FOV
    let names = self
      .objects
      .iter()
      .filter(|obj| obj.pos() == (x, y) && self.fov.is_in_fov(obj.x, obj.y))
      .map(|obj| obj.name.clone())
      .collect::<Vec<_>>();

    names.join(", ") // join the names, separated by commas
  }
}
