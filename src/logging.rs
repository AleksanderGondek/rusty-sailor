use std::fs::File;
use std::io::{Error, ErrorKind};

use simplelog::{
  CombinedLogger,
  ConfigBuilder,
  LevelFilter,
  SimpleLogger,
  WriteLogger
};

use crate::config::Settings;

pub fn init_logger(cfg: &Settings) -> Result<(), Error> {
  let log_level = match cfg.debug {
    true => LevelFilter::Trace,
    false => LevelFilter::Info
  };

  let logger_config = ConfigBuilder::new()
    .set_target_level(LevelFilter::Off)  
    .set_thread_level(LevelFilter::Off)
    // Possible bug, see: https://github.com/Drakulix/simplelog.rs/issues/66
    // In future, only Errors should have location
    .set_location_level(LevelFilter::Error)
    .set_time_format_str("%+")
    .build();

  let terminal_logger = SimpleLogger::new(
    log_level,
    logger_config.clone()
  );

  let logger_file = File::create("rusty-sailor.log")?;
  let file_logger = WriteLogger::new(
    log_level,
    logger_config,
    logger_file
  );

  CombinedLogger::init(
    vec![
      terminal_logger,
      file_logger
    ]
  ).map_or_else(
    |e| Err(Error::new(ErrorKind::Other, e.to_string())),
    |_| Ok(())
  )
}
