use domain::NetStack;
use pal_linux::Linux;

fn main() {
    let mut netstack: NetStack<Linux> = NetStack::<Linux>::init();
}
