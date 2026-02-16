#![no_std]

pub mod pal;

use pal::Console;

pub fn net_init<C: Console>() {
    C::strout("network initialization...");
}
