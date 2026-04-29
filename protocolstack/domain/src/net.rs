use crate::net_device::NetDevice;
use alloc::vec::Vec;
use log::info;
use spin::Mutex;

pub const PROTOCOL_TYPE_IP: u16 = 0x0800;
pub const PROTOCOL_TYPE_ARP: u16 = 0x0806;
pub const PROTOCOL_TYPE_IPV6: u16 = 0x86dd;

pub type ProtocolHandler = fn(data: &[u8], dev: &NetDevice);

static PROTOCOLS: Mutex<Vec<Protocol>> = Mutex::new(Vec::new());

pub struct Protocol {
    ty: u16,
    handler: ProtocolHandler,
}

pub fn register_protocol(ty: u16, handler: ProtocolHandler) -> Result<(), ProtocolError> {
    let mut protocols = PROTOCOLS.lock();
    if protocols.iter().any(|p| p.ty == ty) {
        return Err(ProtocolError::AlreadyRegistered);
    };

    protocols.push(Protocol { ty, handler });
    info!("registered, type=0x{:04x}", ty);
    Ok(())
}

#[derive(Debug)]
pub enum ProtocolError {
    AlreadyRegistered,
}
