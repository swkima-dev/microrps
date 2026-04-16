#![no_std]
extern crate alloc;

pub mod net_device;
pub mod pal;
pub mod util;

use crate::net_device::DeviceDriver;
use crate::net_device::loopback::LoopBackDriver;
use crate::pal::Platform;
use crate::util::debugdump;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::marker::PhantomData;
use log::{debug, info, warn};
use net_device::{Builder, NetDevice, NetDeviceError, NetDeviceFlags, NetDeviceType};

const LOOPBACK_MTU: u16 = u16::MAX;

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

    pub fn register_device(
        &mut self,
        device_type: NetDeviceType,
        mtu: u16,
        header_len: u16,
        address_len: u16,
        addr: [u8; 16],
        driver: Option<Box<dyn DeviceDriver>>,
    ) -> usize {
        info!("Register new device...");
        let index_size = self.devices.len();
        let new_device_name = String::from("net") + &index_size.to_string();
        let new_device = Builder::new()
            .index(index_size)
            .name(new_device_name.clone())
            .device_type(device_type)
            .mtu(mtu)
            .header_len(header_len)
            .address_len(address_len)
            .addr(addr)
            .flags(NetDeviceFlags::empty())
            .driver(driver)
            .build()
            .expect("All fields are provided by new_device");
        self.devices.push(new_device);
        info!("success, dev={}", &new_device_name);
        index_size
    }

    pub fn loopback_init(&mut self, driver: LoopBackDriver) -> usize {
        info!("Register new loopback...");
        let index = self.register_device(
            NetDeviceType::LoopBack,
            LOOPBACK_MTU,
            0,
            0,
            [0u8; 16],
            Some(Box::new(driver)),
        );
        index
    }

    pub fn input(
        &self,
        protocol_type: u16,
        data: &[u8],
        index: usize,
    ) -> Result<(), NetStackError> {
        let Some(device) = self.devices.get(index) else {
            warn!("target device not found");
            return Err(NetStackError::DeviceNotFound);
        };
        debug!(
            "dev={}, type={}, len={}",
            device.name(),
            protocol_type,
            data.len()
        );
        debugdump(data);
        Ok(())
    }

    pub fn output(
        &mut self,
        index: usize,
        protocol_type: u16,
        data: &[u8],
    ) -> Result<(), NetStackError> {
        let Some(device) = self.devices.get_mut(index) else {
            warn!("target device not found");
            return Err(NetStackError::DeviceNotFound);
        };
        device.output(protocol_type, data, ())?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum NetStackError {
    DeviceNotFound,
    Device(NetDeviceError),
}

impl From<NetDeviceError> for NetStackError {
    fn from(err: NetDeviceError) -> NetStackError {
        NetStackError::Device(err)
    }
}
