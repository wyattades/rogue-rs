use std::iter::FromIterator;
use web_sys::CanvasRenderingContext2d;

use crate::colors::{self, Color};
use crate::config::*;

/// Rendering engine
/// Named "Tcod" after the rogulelike "tcod" project: https://pypi.org/project/tcod/
pub struct Tcod {
  pub w: i32,
  pub h: i32,
  fill: Option<Color>,
  stroke: Option<Color>,
  bg: Vec<Vec<Option<Color>>>,
  chars: Vec<Vec<Option<(char, Color)>>>,
}

pub enum TextAlignment {
  Left,
  Center,
  Right,
}

impl Tcod {
  pub fn new(w: i32, h: i32) -> Self {
    Self {
      w: w,
      h: h,
      fill: None,
      stroke: None,
      bg: vec![vec![None; h as usize]; w as usize],
      // fg: vec![vec![None; h as usize]; w as usize],
      chars: vec![vec![None; h as usize]; w as usize],
    }
  }

  pub fn clear_chars(&mut self) {
    for x in 0..self.w {
      for y in 0..self.h {
        self.chars[x as usize][y as usize] = None;
      }
    }
  }

  pub fn fill(&mut self, color: Color) {
    self.fill = Some(color);
  }

  pub fn stroke(&mut self, color: Color) {
    self.stroke = Some(color);
  }

  pub fn background(&mut self, color: Color) {
    // self.panel.set_default_background(color);
    // self.panel.clear();
    self.fill(color);
    self.rect(0, 0, self.w, self.h);
  }

  pub fn put_char_background(&mut self, x: i32, y: i32, color: Color) {
    // self
    //   .con
    //   .set_char_background(x, y, color, BackgroundFlag::Set);

    self.bg[x as usize][y as usize] = Some(color);
  }

  pub fn put_char(&mut self, x: i32, y: i32, char: char) {
    // self.fg[x as usize][y as usize] = self.stroke;
    self.chars[x as usize][y as usize] = Some((char, self.stroke.unwrap_or(colors::WHITE)));
  }

  pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
    for ix in x..(x + w) {
      for iy in y..(y + h) {
        self.bg[ix as usize][iy as usize] = self.fill;
      }
    }
  }

  pub fn print_ex(&mut self, text: &String, x: i32, y: i32, align: TextAlignment) {
    // self.panel.print_ex(
    //   x,
    //   y,
    //   BackgroundFlag::None,
    //   TextAlignment::Left,
    //   text,
    // );
    use TextAlignment::*;
    let x = x
      + (match align {
        Left => 0,
        Right => text.len() as i32,
        Center => (text.len() as f32 * -0.5) as i32,
      });
    for (i, char) in text.char_indices() {
      let char_x = x + i as i32;
      if char_x >= self.w {
        break;
      }
      self.put_char(char_x, y, char);
    }
  }

  /// returns the number of lines that `text` takes up
  pub fn get_height_rect(&mut self, text: &String, _x: i32, _y: i32, w: i32, _h: i32) -> i32 {
    (text.len() as i32 / w) + 1
  }

  /// print text inside a rectangle
  pub fn print_rect(&mut self, text: &String, x: i32, y: i32, w: i32, _h: i32) {
    // TODO: what does `h` do?
    // self.panel.print_rect(
    //   x,
    //   y,
    //   w,
    //   h,
    //   text,
    //   false,
    //   ....
    // );

    if w == 0 {
      return;
    }

    for (i, char) in text.char_indices() {
      let char_x = x + i as i32 % w;
      let char_y = y + i as i32 / w;
      if char_y >= self.h {
        break;
      }
      if char_x < self.w {
        self.put_char(char_x, char_y, char);
      }
    }
  }

  pub fn render_to_string(&self) -> String {
    let mut chars = vec![' '; ((self.w + 1) * self.h) as usize];

    for y in 0..self.h {
      chars[((y * (self.w + 1)) + self.w) as usize] = '\n';
      for x in 0..self.w {
        chars[((y * (self.w + 1)) + x) as usize] =
          if let Some((obj, _color)) = self.chars[x as usize][y as usize] {
            obj
          } else if let Some(wall) = self.bg[x as usize][y as usize] {
            match wall {
              colors::BLACK => '█',
              COLOR_DARK_GROUND => '▒',
              COLOR_LIGHT_GROUND => ' ',
              COLOR_DARK_WALL => '▓',
              COLOR_LIGHT_WALL => '░',
              _ => ' ',
            }
          } else {
            ' '
          };
      }
    }

    String::from_iter(chars)
  }

  pub fn render_to_canvas(&self, ctx: CanvasRenderingContext2d, scale_x: i32, scale_y: i32) {
    for y in 0..self.h {
      for x in 0..self.w {
        if let Some(ref bg) = self.bg[x as usize][y as usize] {
          ctx.set_fill_style(&&bg.to_hex_str().into());
          ctx.fill_rect(
            (x * scale_x).into(),
            (y * scale_y).into(),
            scale_x.into(),
            scale_y.into(),
          );
        }
      }
    }

    for y in 0..self.h {
      for x in 0..self.w {
        if let Some((obj, ref color)) = self.chars[x as usize][y as usize] {
          ctx.set_fill_style(&color.to_hex_str().into());
          // TODO: render real sprites instead of text?
          ctx
            .fill_text(
              &obj.to_string(),
              (x * scale_x).into(),
              (y * scale_y + 1).into(), // add a 1px offset
            )
            .unwrap();
        }
      }
    }
  }

  pub fn fill_render_buffer(&self, render_buffer: &mut [u8]) {
    let mut i: usize = 0;
    for y in 0..self.h {
      for x in 0..self.w {
        let bg_color = &self.bg[x as usize][y as usize].unwrap_or(colors::BLACK);
        let (char, ref char_color) =
          self.chars[x as usize][y as usize].unwrap_or(('\0', colors::BLACK));

        render_buffer[i] = bg_color.r;
        render_buffer[i + 1] = bg_color.g;
        render_buffer[i + 2] = bg_color.b;
        render_buffer[i + 3] = char_color.r;
        render_buffer[i + 4] = char_color.g;
        render_buffer[i + 5] = char_color.b;
        render_buffer[i + 6] = char as u8;

        i += 7;
      }
    }
  }
}
