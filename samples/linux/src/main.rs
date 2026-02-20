use domain::net_init;
use pal_linux::Linux;

fn main() {
    net_init::<Linux>();
}
