#![no_std]

pub mod pal;
use crate::pal::Platform;
use log::info;

pub fn net_init<P: Platform>() {
    P::init();
    info!("network initialization...");
    // implementation of execution of network protocol stack;
    info!("network shutdown...");
}
