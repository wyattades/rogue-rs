use bresenham::Bresenham;
use std::cmp;

/*
 * Ported from https://github.com/libtcod/libtcod/blob/master/src/libtcod/fov_circular_raycasting.c
 */

#[derive(Default, Clone, Copy, Debug)]
pub struct Cell {
  pub transparent: bool,
  pub walkable: bool,
  pub fov: bool,
}

#[derive(Debug)]
pub struct FOV {
  w: i32,
  h: i32,
  nbcells: i32,
  pub cells: Vec<Cell>, // TEMP
}

impl FOV {
  pub fn new(w: i32, h: i32) -> Self {
    Self {
      w: w,
      h: h,
      nbcells: w * h,
      cells: vec![Default::default(); (w * h) as usize],
    }
  }

  // Why does `walkable` matter?
  pub fn set(&mut self, x: i32, y: i32, transparent: bool, walkable: bool) {
    let mut cell = &mut self.cells[(x + y * self.w) as usize];
    cell.walkable = walkable;
    cell.transparent = transparent;
    cell.fov = false;
  }

  pub fn is_in_fov(&self, x: i32, y: i32) -> bool {
    self.cells[(x + y * self.w) as usize].fov
  }

  /// circular ray casting
  pub fn compute_fov(&mut self, player_x: i32, player_y: i32, max_radius: i32, light_walls: bool) {
    let mut xmin = 0;
    let mut ymin = 0;
    let mut xmax = self.w;
    let mut ymax = self.h;

    if max_radius > 0 {
      xmin = cmp::max(0, player_x - max_radius);
      ymin = cmp::max(0, player_y - max_radius);
      xmax = cmp::min(self.w, player_x + max_radius + 1);
      ymax = cmp::min(self.h, player_y + max_radius + 1);
    }

    for i in 0..self.nbcells {
      self.cells[i as usize].fov = false;
    }

    let r2 = max_radius * max_radius;
    let mut xo = xmin;
    let mut yo = ymin;
    while xo < xmax {
      self.cast_ray(player_x, player_y, xo, yo, r2, light_walls);
      xo += 1;
    }
    xo = xmax - 1;
    yo = ymin + 1;
    while yo < ymax {
      self.cast_ray(player_x, player_y, xo, yo, r2, light_walls);
      yo += 1;
    }
    xo = xmax - 2;
    yo = ymax - 1;
    while xo >= 0 {
      self.cast_ray(player_x, player_y, xo, yo, r2, light_walls);
      xo -= 1;
    }
    xo = xmin;
    yo = ymax - 2;
    while yo > 0 {
      self.cast_ray(player_x, player_y, xo, yo, r2, light_walls);
      yo -= 1;
    }

    if light_walls {
      // post-processing artefact fix
      self.postproc(xmin, ymin, player_x, player_y, -1, -1);
      self.postproc(player_x, ymin, xmax - 1, player_y, 1, -1);
      self.postproc(xmin, player_y, player_x, ymax - 1, -1, 1);
      self.postproc(player_x, player_y, xmax - 1, ymax - 1, 1, 1);
    }
  }

  fn cast_ray(&mut self, xo: i32, yo: i32, xd: i32, yd: i32, r2: i32, light_walls: bool) {
    let mut inside = false;
    let mut blocked = false;
    let mut offset = xo + yo * self.w;

    if 0 <= offset && offset < self.nbcells {
      inside = true;
      self.cells[offset as usize].fov = true;
    }

    for (curx, cury) in Bresenham::new((xo as isize, yo as isize), (xd as isize, yd as isize)) {
      let curx = curx as i32;
      let cury = cury as i32;

      offset = curx + cury * self.w;
      if r2 > 0 {
        /* check radius */
        let cur_radius = (curx - xo) * (curx - xo) + (cury - yo) * (cury - yo);
        if cur_radius > r2 {
          return;
        };
      }
      if 0 <= offset && offset < self.nbcells {
        inside = true;
        if !blocked && !self.cells[offset as usize].transparent {
          blocked = true;
        } else if blocked {
          return; /* wall */
        }
        if light_walls || !blocked {
          self.cells[offset as usize].fov = true;
        }
      } else if inside {
        return;
      }; /* ray out of map */
    }
  }

  fn postproc(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, dx: i32, dy: i32) {
    for cx in x0..=x1 {
      for cy in y0..=y1 {
        let x2 = cx + dx;
        let y2 = cy + dy;
        let offset = cx + cy * self.w;

        if offset < self.nbcells
          && self.cells[offset as usize].fov
          && self.cells[offset as usize].transparent
        {
          if x2 >= x0 && x2 <= x1 {
            let offset2 = x2 + cy * self.w;
            if offset2 < self.nbcells && !self.cells[offset2 as usize].transparent {
              self.cells[offset2 as usize].fov = true;
            }
          }
          if y2 >= y0 && y2 <= y1 {
            let offset2 = cx + y2 * self.w;
            if offset2 < self.nbcells && !self.cells[offset2 as usize].transparent {
              self.cells[offset2 as usize].fov = true;
            }
          }
          if x2 >= x0 && x2 <= x1 && y2 >= y0 && y2 <= y1 {
            let offset2 = x2 + y2 * self.w;
            if offset2 < self.nbcells && !self.cells[offset2 as usize].transparent {
              self.cells[offset2 as usize].fov = true;
            }
          }
        }
      }
    }
  }
}
