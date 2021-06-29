use basic_log::{BasicLog, trace, debug};

fn main() {
  BasicLog::new()
    .enable_debug()
    .init()
    .expect("Failed to initialise BasicLog");

  trace!("Example trace message");
  debug!("Example debug message");
}