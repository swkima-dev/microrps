use crate::{
    net_device::{DeviceDriver, DeviceDriverError},
    util::debugdump,
};
use alloc::{collections::vec_deque::VecDeque, vec::Vec};
use log::debug;

pub struct LoopBackDriver {
    rx_queue: VecDeque<(u16, Vec<u8>)>,
}

impl LoopBackDriver {
    pub fn new() -> Self {
        Self {
            rx_queue: VecDeque::new(),
        }
    }
}

impl Default for LoopBackDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceDriver for LoopBackDriver {
    #[allow(unused_variables)]
    fn output(
        &mut self,
        data: &[u8],
        driver_type: u16,
        dst: Option<&[u8]>, // A device tracking a loopback does not need to use the destination address.
    ) -> Result<(), super::DeviceDriverError> {
        debug!(
            "OUTPUT dev: loopback, type: {}, len: {}",
            driver_type,
            data.len()
        );
        debugdump(data);
        self.rx_queue.push_back((driver_type, data.to_vec()));
        Ok(())
    }

    fn poll(&mut self) -> Result<Option<(u16, Vec<u8>)>, DeviceDriverError> {
        Ok(self.rx_queue.pop_front())
    }
}
