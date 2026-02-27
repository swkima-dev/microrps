#![no_std]
extern crate alloc;

pub mod net_device;
pub mod pal;
pub mod util;

use crate::pal::Platform;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::marker::PhantomData;
use log::{info, warn};
use net_device::{NetDevice, NetDeviceError, NetDeviceFlags, NetDeviceType, builder};

pub struct NetStack<P: Platform> {
    devices: Vec<NetDevice>,
    _platform: PhantomData<P>,
}

impl<P: Platform> NetStack<P> {
    pub fn init() -> Self {
        P::init();
        info!("network initialization success");
        Self {
            devices: Vec::new(),
            _platform: PhantomData,
        }
    }

    pub fn run(&mut self) {
        info!("startup...");
        for device in &mut self.devices {
            match device.enable() {
                Ok(()) => info!("{} is enabled", device.name()),
                Err(NetDeviceError::AlreadyUp) => warn!("{} is already Up", device.name()),
                Err(_) => unreachable!(),
            }
        }
        info!("success");
    }

    pub fn shutdown(&mut self) {
        info!("shutting down...");
        for device in &mut self.devices {
            match device.disable() {
                Ok(()) => info!("{} is disabled", device.name()),
                Err(NetDeviceError::AlreadyDown) => warn!("{} is already Down", device.name()),
                Err(_) => unreachable!(),
            }
        }
        info!("success");
    }

    pub fn new_device(
        &mut self,
        device_type: NetDeviceType,
        mtu: u16,
        header_len: u16,
        address_len: u16,
        addr: [u8; 16],
    ) -> usize {
        info!("Register new device...");
        let index_size = self.devices.len();
        let new_device_name = String::from("net") + &index_size.to_string();
        let new_device = builder::Builder::new()
            .index(index_size)
            .name(new_device_name.clone())
            .device_type(device_type)
            .mtu(mtu)
            .header_len(header_len)
            .address_len(address_len)
            .addr(addr)
            .flags(NetDeviceFlags::empty())
            .build()
            .expect("All fields are provided by new_device");
        self.devices.push(new_device);
        info!("success, dev={}", &new_device_name);
        index_size
    }

    pub fn output(&self, index: usize, protocol_type: u16, data: &[u8]) {
        match self.devices[index].output(protocol_type, data, ()) {
            Err(NetDeviceError::DeviceDown) => warn!("target device is down"),
            Err(NetDeviceError::PacketTooLong) => warn!("packet is too long"),
            Err(_) => unreachable!(),
            Ok(()) => info!("output success"),
        };
    }
}
