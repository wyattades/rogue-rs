use crate::colors::{self, Color};
use crate::config::*;
use crate::draw::{Tcod, TextAlignment};

pub struct Messages {
  messages: Vec<(String, Color)>,
}

macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

impl Messages {
  pub fn new() -> Self {
    Self { messages: vec![] }
  }

  /// add the new message as a tuple, with the text and the color
  pub fn add<T: Into<String>>(&mut self, message: T, color: Color) {
    self.messages.push((message.into(), color));
  }

  /// Create a `DoubleEndedIterator` over the messages
  // pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
  //   self.messages.iter()
  // }

  pub fn draw(&self, tcod: &mut Tcod) {
    // print the game messages, one line at a time
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in self.messages.iter().rev() {
      let msg_height = tcod.get_height_rect(msg, MSG_X, y, MSG_WIDTH, 0);
      y -= msg_height;
      if y < 0 {
        break;
      }
      tcod.stroke(color);
      tcod.print_rect(msg, MSG_X, PANEL_Y + y, MSG_WIDTH, 0);
    }
  }
}

pub fn render_bar(
  tcod: &mut Tcod,
  x: i32,
  y: i32,
  total_width: i32,
  name: &str,
  value: i32,
  maximum: i32,
  bar_color: Color,
  back_color: Color,
) {
  let y = PANEL_Y + y;

  // render a bar (HP, experience, etc). First calculate the width of the bar
  let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

  // render the background first
  tcod.fill(back_color);
  tcod.rect(x, y, total_width, 1);

  // now render the bar on top
  tcod.fill(bar_color);
  if bar_width > 0 {
    tcod.rect(x, y, bar_width, 1);
  }

  // finally, some centered text with the values
  tcod.stroke(colors::WHITE);
  tcod.print_ex(
    &format!("{}: {}/{}", name, value, maximum),
    x + total_width / 2,
    y,
    TextAlignment::Center,
  );
}
