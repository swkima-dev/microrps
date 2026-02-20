#![no_std]
extern crate alloc;

pub mod net_device;
pub mod pal;

use core::marker::PhantomData;

use crate::pal::Platform;
use alloc::vec::Vec;
use log::info;
use net_device::NetDevice;

// pub fn net_init<P: Platform>() {
//     P::init();
//     info!("network initialization...");
//     // implementation of execution of network protocol stack;
//     info!("network shutdown...");
// }

pub struct NetStack<P: Platform> {
    devices: Vec<NetDevice>,
    _platform: PhantomData<P>,
}

impl<P: Platform> NetStack<P> {
    pub fn init() -> Self {
        P::init();
        Self {
            devices: Self::devices_new(),
            _platform: PhantomData,
        }
    }

    fn devices_new() -> Vec<NetDevice> {
        Vec::new()
    }
}
