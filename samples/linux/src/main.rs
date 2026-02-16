use domain;
use pal_linux::LinuxConsole;

fn main() {
    domain::net_init::<LinuxConsole>();
}
