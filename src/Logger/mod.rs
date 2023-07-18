use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::prelude::{DateTime, Utc};

use super::Timer::Timer;

pub const FormatTEXT: usize = 1;
pub const FormatJSON: usize = 2;

pub const LevelINFO: &str = "INFO";
const LevelTIMER: &str = "TIMER";
pub const LevelWARN: &str = "WARN";
pub const LevelERROR: &str = "ERROR";
pub const LevelFATAL: &str = "FATAL";

pub struct Logger{
	pub OutputCMD: bool,
	pub OutputCMDFormat: usize,

	pub OutputFile: bool,
	pub OutputFilePath: String,
	pub OutputFileErase: bool,
	pub OutputFileFormat: usize,
}
impl Logger {
	pub fn new (
		outputCMD: bool,
		outputCMDFormat: usize,
		outputFile: bool,
		outputFilePath: String,
		outputFileErase: bool,
		outputFileFormat: usize,
	) -> Logger {
		let logger =  Logger {
			OutputCMD: outputCMD,
			OutputCMDFormat: outputCMDFormat,
			OutputFile: outputFile,
			OutputFilePath: outputFilePath,
			OutputFileErase: outputFileErase,
			OutputFileFormat: outputFileFormat,
		};
		if logger.OutputFile {
			let mut empty_file = false;
			if !std::path::Path::exists(Path::new(&logger.OutputFilePath)) {
				create_file(&logger.OutputFilePath.clone());
				empty_file = true;
			} else {
				if logger.OutputFileErase {
					delete_file(&logger.OutputFilePath.clone());
					create_file(&logger.OutputFilePath.clone());
					empty_file = true;
				}
			}
			if !empty_file {
				write_string_to_file(&logger.OutputFilePath, "\n\n".to_string());
			}
		}
		return logger;
	}

	pub fn LogString(&mut self, level: &str, string: String) {
		if self.OutputCMD {
			if self.OutputCMDFormat == FormatTEXT {
				println!("{}", format!(
					"{} [{}] {}",
					ToTimeFormat_iso8601(&std::time::SystemTime::now()),
					level, string.clone()
				));
			} else if self.OutputCMDFormat == FormatJSON {
				println!("{}{}{}", "{", format!(
					"\"time_stamp\": \"{}\", \"level\": \"{}\", \"message\": \"{}\"",
					ToTimeFormat_iso8601(&std::time::SystemTime::now()),
					level, string.clone()
				), "}")
			}
		}
		if self.OutputFile {
			if self.OutputFileFormat == FormatTEXT {
				write_string_to_file(&self.OutputFilePath, format!(
					"{} [{}] {}\n",
					ToTimeFormat_iso8601(&std::time::SystemTime::now()),
					level, string.clone()
				));
			}
			if self.OutputFileFormat == FormatJSON {
				write_string_to_file(&self.OutputFilePath, format!(
					"{{\"time_stamp\": \"{}\", \"level\": \"{}\", \"message\": \"{}\"}}\n",
					ToTimeFormat_iso8601(&std::time::SystemTime::now()),
					level, string.clone()
				));
			}
		}
	}
	pub fn LogStr(&mut self, level: &str, str: &str) {
		self.LogString(level, str.to_string());
	}

	pub fn LogTimer(&mut self, timer: &Timer) {self.LogString(LevelTIMER, format!("Duration: {} ms", timer.DurationMilliSecs()));}
	pub fn LogTimerTitled(&mut self, timer: &Timer, title: &str) {self.LogString(LevelTIMER, format!("{}: {} ms", title, timer.DurationMilliSecs()));}
	pub fn LogTimerSecs(&mut self, timer: &Timer) {self.LogString(LevelTIMER, format!("Duration: {} s", timer.DurationSecs()));}
	pub fn LogTimerSecsTitled(&mut self, timer: &Timer, title: &str) {self.LogString(LevelTIMER, format!("{}: {} s", title, timer.DurationSecs()));}
}

pub fn ToTimeFormat_iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}

fn delete_file(file_path: &String) {
	std::fs::remove_file(&file_path).expect("Dvigoon Logger - Unable to delete file");
}
fn create_file(file_path: &String) {
	File::create(&file_path).expect("Dvigoon Logger - Unable to create file");
}
fn write_string_to_file(file_path: &String, string: String) {
	let mut fileRef = OpenOptions::new().append(true).open(file_path).expect("Dvigoon Logger - Unable to open file");
	fileRef.write_all(string.as_bytes()).expect("Dvigoon Logger - Unable to write data");
}
