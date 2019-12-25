use rand::Rng;
use std::cmp;

use crate::config::*;
use crate::rect::Rect;

// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
pub struct Tile {
  pub blocked: bool,
  pub explored: bool,
  pub block_sight: bool,
}

impl Tile {
  pub fn empty() -> Self {
    Tile {
      blocked: false,
      explored: false,
      block_sight: false,
    }
  }

  pub fn wall() -> Self {
    Tile {
      blocked: true,
      explored: false,
      block_sight: true,
    }
  }
}

pub struct Map {
  tiles: Vec<Vec<Tile>>,
  pub rooms: Vec<Rect>,
}

impl Map {
  pub fn new<R: Rng>(rng: &mut R) -> Self {
    // fill map with "blocked" tiles
    let mut map = Map {
      tiles: vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize],
      rooms: vec![],
    };

    for _ in 0..MAX_ROOMS {
      // random width and height
      let w = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
      let h = rng.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
      // random position without going out of the boundaries of the map
      let x = rng.gen_range(0, MAP_WIDTH - w);
      let y = rng.gen_range(0, MAP_HEIGHT - h);

      let new_room = Rect::new(x, y, w, h);

      // run through the other rooms and see if they intersect with this one
      let failed = map
        .rooms
        .iter()
        .any(|other_room| new_room.intersects_with(other_room));

      if !failed {
        // this means there are no intersections, so this room is valid

        // "paint" it to the map's tiles
        map.create_room(new_room);

        // center coordinates of the new room, will be useful later
        let (new_x, new_y) = new_room.center();

        if !map.rooms.is_empty() {
          // all rooms after the first:

          // connect it to the previous room with a tunnel

          // center coordinates of the previous room
          let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

          // toss a coin (random bool value -- either true or false)
          if rng.gen::<bool>() {
            // first move horizontally, then vertically
            map.create_h_tunnel(prev_x, new_x, prev_y);
            map.create_v_tunnel(prev_y, new_y, new_x);
          } else {
            // first move vertically, then horizontally
            map.create_v_tunnel(prev_y, new_y, prev_x);
            map.create_h_tunnel(prev_x, new_x, new_y);
          }
        }

        // finally, append the new room to the list
        map.rooms.push(new_room);
      }
    }

    map
  }

  fn create_room(&mut self, room: Rect) {
    // go through the tiles in the rectangle and make them passable
    // for (x, y) in room.iter_points() {
    for x in (room.x1 + 1)..room.x2 {
      for y in (room.y1 + 1)..room.y2 {
        self.tiles[x as usize][y as usize] = Tile::empty();
      }
    }
  }

  fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
      self.tiles[x as usize][y as usize] = Tile::empty();
    }
  }

  fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
      self.tiles[x as usize][y as usize] = Tile::empty();
    }
  }

  pub fn tile_at(&self, x: i32, y: i32) -> &Tile {
    &self.tiles[x as usize][y as usize]
  }

  pub fn set_explored(&mut self, x: i32, y: i32) {
    self.tiles[x as usize][y as usize].explored = true;
  }
}
