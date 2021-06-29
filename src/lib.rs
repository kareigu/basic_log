use log::{LevelFilter, Record, Log, Metadata, Level, SetLoggerError};
use colored::*;
use chrono::Local;
pub use log::{info, warn, error, debug, trace};

pub struct BasicLog {
  output_level: LevelFilter,
}

impl BasicLog {
  #[must_use = "Must initialise logger: .init()"]
  pub fn new() -> Self {
    Self {
      output_level: LevelFilter::Info,
    }
  }

  #[must_use = "Must initialise logger: .init()"]
  pub fn enable_debug(mut self) -> BasicLog {
    self.output_level = LevelFilter::Debug;
    self
  }

  #[must_use = "Must initialise logger: .init()"]
  pub fn enable_trace(mut self) -> BasicLog {
    self.output_level = LevelFilter::Trace;
    self
  }

  pub fn init(self) -> Result<(), SetLoggerError> {
    log::set_max_level(self.output_level);
    log::set_boxed_logger(Box::new(self))?;
    Ok(())
  }
}

impl Default for BasicLog {
  fn default() -> Self {
    BasicLog::new()
  }
}

impl Log for BasicLog {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level().to_level_filter() <= self.output_level
  }

  fn log(&self, rec: &Record) {
    use Level::*;
    let log_type = match rec.level() {
      Error => "ERROR".bright_red(),
      Warn => "WARN".bright_yellow(),
      Info => "INFO".bright_blue(),
      Debug => "DEBUG".green(),
      Trace => "trace".bright_purple(),
    };

    let time = Local::now()
      .format("%d %B %Y %H:%M:%S,%3f");

    println!("{log_type:<5} | {time} - {msg}",
      log_type = log_type,
      time = time,
      msg = rec.args()
    )
  }

  fn flush(&self) {
      
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
