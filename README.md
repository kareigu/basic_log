# basic_log
![crates.io](https://img.shields.io/crates/v/basic_log?style=flat-square)
![License badge](https://img.shields.io/github/license/mxrr/basic_log?label=License&style=flat-square)
![Latest commit](https://img.shields.io/github/last-commit/mxrr/basic_log?label=Latest%20commit&style=flat-square)
## A basic logging crate for Rust.

--- 
Focuses on having sensible defaults for basic logging to accomplish great out of the box functionality.


## Usage

To use the default settings, simply create and initialise the logger  
The crate exposes the logging macros provided by log for easy access

```rs
use basic_log::{BasicLog, info, warn, error};

fn main() {
  BasicLog::new()
    .init()
    .expect("Failed to initialise BasicLog");

  info!("Example info message");
  warn!("Example warning");
  error!("Example error");
}
```

To change logger behaviour you can use a closure or a settings struct

```rs
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
```

```rs
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
```