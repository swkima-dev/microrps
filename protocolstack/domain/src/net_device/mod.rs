use crate::util;
use alloc::string::String;
use bitflags::bitflags;

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
    pub fn new(
        index: usize,
        name: String,
        device_type: NetDeviceType,
        mtu: u16,
        header_len: u16,
        address_len: u16,
        addr: [u8; 16],
        flags: NetDeviceFlags,
    ) -> Self {
        NetDevice {
            index,
            name,
            device_type,
            mtu,
            header_len,
            address_len,
            addr,
            flags,
        }
    }

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
