use domain::pal::Platform;
use simple_logger::SimpleLogger;
pub struct Linux;

impl Platform for Linux {
    type Logger = SimpleLogger;

    fn log_init(logger: Self::Logger) {
        logger.init().expect("Logger initialization failed.");
    }

    fn create_logger() -> Self::Logger {
        Self::Logger::new()
    }
}
