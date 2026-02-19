use log::Log;

pub trait Platform {
    type Logger: Log;

    fn init() {
        Self::log_init(Self::create_logger());
    }
    fn log_init(logger: Self::Logger);

    fn create_logger() -> Self::Logger;
}
