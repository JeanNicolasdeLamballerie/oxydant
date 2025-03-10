// use log::{debug, error, info, trace, warn};
use std::{env, time::SystemTime};

#[cfg(debug_assertions)]
fn use_level() -> log::LevelFilter {
    if let Ok(level) = env::var("OXYDANT_LOG_LEVEL") {
        let envlevel: &str = &level.to_uppercase();
        match envlevel {
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "ERROR" => log::LevelFilter::Error,
            "WARN" => log::LevelFilter::Warn,
            "OFF" => log::LevelFilter::Off,
            _ => log::LevelFilter::Debug,
        }
    } else {
        log::LevelFilter::Debug
    }
}
#[cfg(not(debug_assertions))]
fn use_level() -> log::LevelFilter {
    if let Ok(level) = env::var("OXYDANT_LOG_LEVEL") {
        let envlevel: &str = &level.to_uppercase();
        match envlevel {
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "ERROR" => log::LevelFilter::Error,
            "WARN" => log::LevelFilter::Warn,
            "OFF" => log::LevelFilter::Off,
            _ => log::LevelFilter::Debug,
        }
    } else {
        log::LevelFilter::Error
    }
}
pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_micros(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(use_level())
        .chain(std::io::stdout())
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{} {} {}] {}",
                        humantime::format_rfc3339_micros(SystemTime::now()),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file("output.log")?),
        )
        .apply()?;
    Ok(())
}
