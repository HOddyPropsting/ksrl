extern crate tcod;

use tcod::*;
use console::{Root};
use input::KeyPressFlags;


fn main() {
  let (width,height) = (80,80);
  let mut c = Root::initializer()
    .size(width,height)
    .font_dimensions(16,16)
    .font("terminal.png", FontLayout::AsciiInCol)
    .fullscreen(false)
    .init();
  c.clear();
  c.set_default_foreground(colors::CRIMSON);
  c.set_char(40,20, 'r');
  c.horizontal_line(2,2,10,BackgroundFlag::None);
  loop {
    c.flush();
    let k = c.check_for_keypress(KeyPressFlags::all());
    match k {
      None => { }
      _ => { break }
    }
  }
}