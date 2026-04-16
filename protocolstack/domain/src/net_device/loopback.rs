use crate::{
    net_device::{DeviceDriver, DeviceDriverError},
    util::debugdump,
};
use alloc::{collections::vec_deque::VecDeque, vec::Vec};
use log::debug;

pub struct LoopBackDriver {
    rx_queue: VecDeque<(u16, Vec<u8>)>,
}

impl DeviceDriver for LoopBackDriver {
    #[allow(unused_variables)]
    fn output(
        &mut self,
        data: &[u8],
        len: u16,
        driver_type: u16,
        dst: Option<&[u8]>, // A device tracking a loopback does not need to use the destination address.
    ) -> Result<(), super::DeviceDriverError> {
        debug!("dev: loopback, type: {}, len: {}", driver_type, len);
        debugdump(data);
        self.rx_queue.push_back((driver_type, data.to_vec()));
        Ok(())
    }

    fn poll(&mut self) -> Result<Option<(u16, Vec<u8>)>, DeviceDriverError> {
        Ok(self.rx_queue.pop_front())
    }
}
