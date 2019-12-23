use tcod::colors::{self, Color};
use tcod::console::{BackgroundFlag, Console, Offscreen, TextAlignment};

use crate::config::*;

pub struct Messages {
  messages: Vec<(String, Color)>,
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
  pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, Color)> {
    self.messages.iter()
  }

  pub fn draw(&self, panel: &mut Offscreen) {
    // print the game messages, one line at a time
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in self.iter().rev() {
      let msg_height = panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
      y -= msg_height;
      if y < 0 {
        break;
      }
      panel.set_default_foreground(color);
      panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }
  }
}

pub fn render_bar(
  panel: &mut Offscreen,
  x: i32,
  y: i32,
  total_width: i32,
  name: &str,
  value: i32,
  maximum: i32,
  bar_color: Color,
  back_color: Color,
) {
  // render a bar (HP, experience, etc). First calculate the width of the bar
  let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

  // render the background first
  panel.set_default_background(back_color);
  panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

  // now render the bar on top
  panel.set_default_background(bar_color);
  if bar_width > 0 {
    panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
  }

  // finally, some centered text with the values
  panel.set_default_foreground(colors::WHITE);
  panel.print_ex(
    x + total_width / 2,
    y,
    BackgroundFlag::None,
    TextAlignment::Center,
    &format!("{}: {}/{}", name, value, maximum),
  );
}
