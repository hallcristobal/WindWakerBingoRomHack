#![no_std]
#![feature(asm)]

#[macro_use]
extern crate libtww;

use libtww::{controller, items, console, link, system};

static mut item: u8 = 0;

#[no_mangle]
#[inline(never)]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    console::activate();
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    console::write_line1(format!("Hello World: {:02X}", unsafe { item }));

    if controller::is_pressed(controller::DPAD_LEFT) {
        unsafe {
            item -= 1;
        }
    }
    if controller::is_pressed(controller::DPAD_RIGHT) {
        unsafe {
            item += 1;
        }
    }
    if controller::is_pressed(controller::DPAD_DOWN) {
        items::spawn(link::get_position(), unsafe { item });
    }
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
}
