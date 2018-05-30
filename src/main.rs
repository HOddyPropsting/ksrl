extern crate tcod;

use tcod::*;
use console::{Root, Offscreen};



fn main() {
    println!("Hello, world!");

    let mut c = Root::initializer().init();
    let (width,height) = (80,30);
    let mut buff = Offscreen::new(width,height);

    while !c.window_closed() {

    }

}

