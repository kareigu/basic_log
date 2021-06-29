use basic_log::{BasicLog, trace, debug};

fn main() {
  BasicLog::new_with_settings(
    |s| 
      s
      .enable_debug()
      .enable_trace()
    )
    .init()
    .expect("Failed to initialise BasicLog");

  trace!("Example trace message");
  debug!("Example debug message");
}