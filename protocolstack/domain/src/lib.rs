#![no_std]
extern crate alloc;

pub mod net_device;
pub mod pal;

use crate::pal::Platform;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;
use log::info;
use net_device::{NetDevice, NetDeviceFlags, NetDeviceType};

pub struct NetStack<P: Platform> {
    devices: Vec<NetDevice>,
    _platform: PhantomData<P>,
}

impl<P: Platform> NetStack<P> {
    pub fn init() -> Self {
        P::init();
        info!("network initialization...");
        info!("success");
        Self {
            devices: Vec::new(),
            _platform: PhantomData,
        }
    }

    pub fn new_device(
        &mut self,
        device_type: NetDeviceType,
        mtu: u16,
        header_len: u16,
        address_len: u16,
        addr: [u8; 16],
    ) {
        info!("Register new device...");
        let index_size = self.devices.len();
        let mut exist_index = vec![false; index_size + 1];
        for device in &self.devices {
            exist_index[device.get_index()] = true;
        }
        for (i, val) in exist_index.iter().enumerate() {
            if !val {
                let new_device_name = String::from("net") + &i.to_string();
                NetDevice::new(
                    i,
                    new_device_name.clone(),
                    device_type,
                    mtu,
                    header_len,
                    address_len,
                    addr,
                    NetDeviceFlags::empty(),
                );
                info!("success, dev={}", &new_device_name);
                break;
            }
        }
    }
}
