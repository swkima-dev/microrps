use domain::net_init;
use pal_linux::LinuxConsole;

fn main() {
    net_init::<LinuxConsole>();
}
