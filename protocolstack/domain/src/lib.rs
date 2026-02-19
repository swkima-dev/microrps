#![no_std]

pub mod pal;
use crate::pal::LogInit;
use log::info;

pub fn net_init<L: LogInit>() {
    L::init();
    info!("network initialization...");
}
