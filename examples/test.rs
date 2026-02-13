#![no_std]
extern crate alloc;

use log::info;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Hello microrps!");
}
