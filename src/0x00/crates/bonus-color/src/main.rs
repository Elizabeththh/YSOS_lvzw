use colored::Colorize;
use log::*;
struct LvzwLogger;

impl Log for LvzwLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if self.enabled(metadata) {
            match metadata.level() {
                Level::Error => println!("[{}]: {}", record.level().as_str().red(), record.args()),
                Level::Warn => {
                    println!("[{}]: {}", record.level().as_str().yellow(), record.args())
                }
                Level::Info => println!("[{}]: {}", record.level().as_str().green(), record.args()),
                Level::Debug => println!("[{}]: {}", record.level().as_str().blue(), record.args()),
                Level::Trace => print!("[{}], {}", record.level().as_str().purple(), record.args()),
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: LvzwLogger = LvzwLogger;

impl LvzwLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_max_level(level);
        log::set_logger(&LOGGER)
    }
}

fn main() {
    LvzwLogger::init(LevelFilter::Trace).expect("Failed to initialize logger");
    error!("an error message");
    warn!("a warn message");
    info!("a info message");
    debug!("a debug message");
    trace!("a trace message");
}
