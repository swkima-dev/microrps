use domain::NetStack;
use pal_linux::Linux;

use domain::net_device::NetDeviceType;

fn main() {
    let mut netstack: NetStack<Linux> = NetStack::<Linux>::init();
    netstack.new_device(NetDeviceType::Dummy, 1600, 0, 0, [0; 16]);
    netstack.run();
    netstack.shutdown();
}
