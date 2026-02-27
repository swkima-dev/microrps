pub mod builder;

use crate::util;
use alloc::string::String;
use bitflags::bitflags;
use core::fmt;

#[allow(dead_code)]
pub struct NetDevice {
    index: usize,
    name: String,
    device_type: NetDeviceType,
    mtu: u16,
    header_len: u16,
    address_len: u16,
    addr: [u8; 16],
    flags: NetDeviceFlags,
}

pub enum NetDeviceType {
    Dummy,
    LoopBack,
    Ethernet,
}

bitflags! {
    pub struct NetDeviceFlags: u16 {
        const UP = 0x0001;
        const LOOPBACK = 0x0010;
        const BROADCAST = 0x0020;
        const P2P = 0x0040;
        const NEED_ARP = 0x0100;
    }
}

impl NetDevice {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn enable(&mut self) -> Result<(), NetDeviceError> {
        if self.is_up() {
            return Err(NetDeviceError::AlreadyUp);
        }
        self.flags.insert(NetDeviceFlags::UP);
        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), NetDeviceError> {
        if !self.is_up() {
            return Err(NetDeviceError::AlreadyDown);
        }
        self.flags.remove(NetDeviceFlags::UP);
        Ok(())
    }

    pub fn is_up(&self) -> bool {
        self.flags.contains(NetDeviceFlags::UP)
    }

    #[allow(unused_variables)]
    pub fn output(&self, protocol_type: u16, data: &[u8], dst: ()) -> Result<(), NetDeviceError> {
        util::debugdump(data);
        if !self.is_up() {
            return Err(NetDeviceError::DeviceDown);
        }
        let mtu_usize = self.mtu as usize;
        if mtu_usize < data.len() {
            return Err(NetDeviceError::PacketTooLong);
        }
        Ok(())
    }
}

pub enum NetDeviceError {
    AlreadyUp,
    AlreadyDown,
    DeviceDown,
    PacketTooLong,
}

#[derive(Default)]
pub struct Builder {
    index: Option<usize>,
    name: Option<String>,
    device_type: Option<NetDeviceType>,
    mtu: Option<u16>,
    header_len: Option<u16>,
    address_len: Option<u16>,
    addr: Option<[u8; 16]>,
    flags: Option<NetDeviceFlags>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<NetDevice, BuildError> {
        Ok(NetDevice {
            index: self.index.ok_or(BuildError::MissingIndex)?,
            name: self.name.ok_or(BuildError::MissingName)?,
            device_type: self.device_type.ok_or(BuildError::MissingDeviceType)?,
            mtu: self.mtu.ok_or(BuildError::MissingMtu)?,
            header_len: self.header_len.ok_or(BuildError::MissingHeaderLen)?,
            address_len: self.address_len.ok_or(BuildError::MissingAddressLen)?,
            addr: self.addr.ok_or(BuildError::MissingAddr)?,
            flags: self.flags.ok_or(BuildError::MissingFlags)?,
        })
    }

    pub fn index(mut self, value: usize) -> Self {
        self.index = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn device_type(mut self, value: NetDeviceType) -> Self {
        self.device_type = Some(value);
        self
    }

    pub fn mtu(mut self, value: u16) -> Self {
        self.mtu = Some(value);
        self
    }

    pub fn header_len(mut self, value: u16) -> Self {
        self.header_len = Some(value);
        self
    }

    pub fn address_len(mut self, value: u16) -> Self {
        self.address_len = Some(value);
        self
    }

    pub fn addr(mut self, value: [u8; 16]) -> Self {
        self.addr = Some(value);
        self
    }

    pub fn flags(mut self, value: NetDeviceFlags) -> Self {
        self.flags = Some(value);
        self
    }
}

#[derive(Debug)]
pub enum BuildError {
    MissingAddressLen,
    MissingAddr,
    MissingDeviceType,
    MissingFlags,
    MissingHeaderLen,
    MissingIndex,
    MissingMtu,
    MissingName,
}

impl BuildError {
    pub fn as_str(&self) -> &'static str {
        use BuildError::*;
        match self {
            MissingAddressLen => "missing address len",
            MissingAddr => "missing address",
            MissingDeviceType => "missing device type",
            MissingFlags => "missing flags",
            MissingHeaderLen => "missing header len",
            MissingIndex => "missing index",
            MissingMtu => "missing mtu",
            MissingName => "missing name",
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
