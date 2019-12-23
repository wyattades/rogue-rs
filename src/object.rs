use tcod::colors::{self, Color};
use tcod::console::{BackgroundFlag, Console};

use crate::ai::*;
use crate::messages::Messages;

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
pub struct Object {
  pub x: i32,
  pub y: i32,
  char: char,
  color: Color,
  pub name: String,
  pub blocks: bool,
  pub alive: bool,
  pub fighter: Option<Fighter>,
  pub ai: Option<Ai>, // TODO: how to use duck module here?
  pub attacking: Option<(i32, i32)>,
}

// combat-related properties and methods (monster, player, NPC).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
  pub max_hp: i32,
  pub hp: i32,
  pub defense: i32,
  pub power: i32,
  pub mtype: i32,
}

impl Fighter {
  pub fn on_death(&self, object: &mut Object, messages: &mut Messages) {
    if self.mtype == 0 {
      // the game ended!
      messages.add("You died!", colors::RED);

      // for added effect, transform the player into a corpse!
      object.char = '%';
      object.color = colors::DARK_RED;
    } else {
      // transform it into a nasty corpse! it doesn't block, can't be
      // attacked and doesn't move
      messages.add(format!("{} is dead!", object.name), colors::ORANGE);

      object.char = '%';
      object.color = colors::DARK_RED;
      object.blocks = false;
      object.fighter = None;
      object.ai = None;
      object.name = format!("remains of {}", object.name);
    }
  }
}

impl Object {
  pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
    Object {
      x,
      y,
      char,
      color,
      name: name.into(),
      blocks: blocks,
      alive: false,
      ai: None,
      fighter: None,
      attacking: None,
    }
  }

  /// move by the given amount, if the destination is not blocked
  pub fn move_by(&mut self, dx: i32, dy: i32) {
    self.x += dx;
    self.y += dy;
  }

  /// set the color and then draw the character that represents this object at its position
  pub fn draw(&self, con: &mut dyn Console) {
    con.set_default_foreground(self.color);
    con.put_char(self.x, self.y, self.char, BackgroundFlag::None);

    if let Some((dx, dy)) = self.attacking {
      con.set_default_foreground(colors::CYAN);
      let char = match (dx, dy) {
        (0, 1) => '|',
        (0, -1) => '|',
        (1, 0) => '-',
        // (-1, 0) => '-',
        _ => '-',
      };
      con.put_char(self.x + dx, self.y + dy, char, BackgroundFlag::None);
    }
  }

  pub fn set_pos(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }

  pub fn pos(&self) -> (i32, i32) {
    (self.x, self.y)
  }

  /// return the distance to another object
  pub fn distance_to(&self, other: &Object) -> f32 {
    let (dx, dy) = self.delta_to(other);
    ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
  }

  /// return delta x and y to another object
  pub fn delta_to(&self, other: &Object) -> (i32, i32) {
    (other.x - self.x, other.y - self.y)
  }

  /// heal by the given amount, without going over the maximum
  pub fn heal(&mut self, amount: i32) {
    if let Some(ref mut fighter) = self.fighter {
      fighter.hp += amount;
      if fighter.hp > fighter.max_hp {
        fighter.hp = fighter.max_hp;
      }
    }
  }

  pub fn take_damage(&mut self, damage: i32, messages: &mut Messages) {
    // apply damage if possible
    if let Some(fighter) = self.fighter.as_mut() {
      if damage > 0 {
        fighter.hp -= damage;
      }
    }
    // check for death, call the death function
    if let Some(fighter) = self.fighter {
      if fighter.hp <= 0 {
        self.alive = false;
        fighter.on_death(self, messages);
      }
    }
  }

  pub fn start_attacking(&mut self, dx: i32, dy: i32) {
    if self.attacking.is_none() {
      self.attacking = Some((dx, dy));
    }
  }

  pub fn stop_attacking(&mut self) {
    self.attacking = None;
  }

  pub fn attack(&mut self, target: &mut Object, messages: &mut Messages) {
    // a simple formula for attack damage
    let damage = self.fighter.map_or(0, |f| f.power) - target.fighter.map_or(0, |f| f.defense);
    if damage > 0 {
      // make the target take some damage
      messages.add(
        format!(
          "{} attacks {} for {} hit points.",
          self.name, target.name, damage
        ),
        colors::WHITE,
      );
      target.take_damage(damage, messages);
    } else {
      messages.add(
        format!(
          "{} attacks {} but it has no effect!",
          self.name, target.name
        ),
        colors::WHITE,
      );
    }
  }
}
