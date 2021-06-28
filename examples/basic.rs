use basic_log::BasicLog;
use log::{info, warn, error};

fn main() {
  BasicLog::new().init().expect("Failed to initialise BasicLog");

  info!("Example info message");
  warn!("Example warning");
  error!("Example error");
}