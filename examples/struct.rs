use basic_log::{BasicLog, LoggerSettings, trace, debug};

fn main() {
  let log_settings = LoggerSettings::new()
    .enable_debug()
    .enable_trace();

  BasicLog::new_with_struct(log_settings)
    .init()
    .expect("Failed to initialise BasicLog");

  trace!("Example trace message");
  debug!("Example debug message");
}