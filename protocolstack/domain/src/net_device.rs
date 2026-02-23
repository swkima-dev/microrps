use alloc::string::String;
use bitflags::bitflags;

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

    pub fn get_index(&self) -> usize {
        self.index
    }
}
