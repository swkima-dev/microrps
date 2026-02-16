use domain::pal::Console;

pub struct LinuxConsole;

impl Console for LinuxConsole {
    fn strout(s: &str) {
        println!("{}", s);
    }
}
