use domain::net_init;
use pal_linux::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    net_init();
}
