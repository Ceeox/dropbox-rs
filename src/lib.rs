// License

// extern crates
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;
// extern mods
// intern mods
mod tests;

pub mod models;
pub mod error;
pub mod connection;
#[macro_use] mod macros;

pub mod files;
// std uses
// crate uses
use log::{ LogRecord, LogLevel, LogMetadata, LogLevelFilter };
// intern uses
use ::error::*;
// consts or statics
static BASE_URL: &'static str = "https://api.dropboxapi.com";
static UPLOAD_URL: &'static str = "https://content.dropboxapi.com";
static API_VERSION: &'static str = "/2";
// etc
pub struct Logger;

impl log::Log for Logger
{
	fn enabled(&self, metadata: &LogMetadata)
	-> bool
	{
		metadata.level() <= LogLevel::Trace
	}

	fn log(&self, record: &LogRecord)
	{
		if self.enabled(record.metadata())
		{
			println!("[{}] {}", record.level(), record.args())
		}
	}
}

pub fn init(log_level: LogLevelFilter)
-> Result<()>
{
	let _ = log::set_logger(|max_log_level|
	{
		max_log_level.set(log_level);
		Box::new(Logger)
    });
	Ok(())
}
