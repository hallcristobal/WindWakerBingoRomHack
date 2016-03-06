#![no_std]
#![feature(asm)]

#[macro_use]
extern crate libtww;

use libtww::prelude::*;
use libtww::system;
use libtww::Link;
use libtww::game::Console;

#[no_mangle]
#[inline(never)]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    Console::get().setup();
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    let link = Link::get();
    let console = Console::get();
    let mut lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Heart Pieces:   {}", link.heart_pieces);
    let _ = write!(lines[1].begin(), "Heart Quarters: {}", link.heart_quarters);
    let _ = write!(lines[2].begin(), "Rupees:         {}", link.rupees);
    let _ = write!(lines[3].begin(), "Sword ID:       {:02X}", link.sword_id);
    let _ = write!(lines[4].begin(), "Shield ID:      {:02X}", link.shield_id);
    let _ = write!(lines[5].begin(), "Max Magic:      {}", link.max_magic);
    let _ = write!(lines[6].begin(), "Magic:          {}", link.magic);
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
}
