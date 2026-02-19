use domain::pal::LogInit;
use simple_logger::SimpleLogger;
pub struct StdLogger;

impl LogInit for StdLogger {
    fn init() {
        SimpleLogger::new()
            .init()
            .expect("Logger initialization has been failed.");
    }
}
