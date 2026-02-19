use domain::net_init;
use pal_linux::StdLogger;

fn main() {
    net_init::<StdLogger>();
}
