//! instruction_read_tester will parse entire Chip8 ROM files using the
//! parser logic.
//!
//! The tester is provided as a means to evaluate exhaustive and
//! complete machine language parsing, but does not ensure the parsing
//! is correct (that would require a comparable ROM with structured
//! data).

// FIXME(joey): This file actually makes no sense! This is because
// developers can intertwine data with code, meaning not everything
// should be a parsable instruction.

use clap::{App, Arg};
use env_logger;
use log::LevelFilter;
use rustyemulator::chip8::instruction::{parse_instruction, Instruction, InstructionError};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
#[macro_use]
extern crate log;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstructionReadTesterError {
  #[error("Parse error")]
  ParseError {
    file: String,
    source: InstructionError,
  },

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

fn main() -> std::io::Result<()> {
  let matches = App::new("Chip8 instruction read tester")
    .version("1.0")
    .author("Joey Pereira <joey@pereira.io>")
    .about("Parses the instructions from Chip8 ROMs")
    .arg(
      Arg::with_name("INPUT")
        .help("Sets the input files o parse")
        .required(true)
        .multiple(true)
        .last(true),
    )
    .arg(
      Arg::with_name("log-level")
        .short("l")
        .takes_value(true)
        .possible_value("off")
        .possible_value("error")
        .possible_value("warn")
        .possible_value("info")
        .possible_value("debug")
        .possible_value("trace")
        .default_value("warn")
        .help("Sets the level of verbosity"),
    )
    .get_matches();

  let verbosity = match matches.value_of("log-level").unwrap() {
    "off" => LevelFilter::Off,
    "error" => LevelFilter::Error,
    "warn" => LevelFilter::Warn,
    "info" => LevelFilter::Info,
    "debug" => LevelFilter::Debug,
    "trace" => LevelFilter::Trace,
    unhandled_level => unimplemented!(
      "Unexpected: provided verbosity level that was unsupporte: {}",
      unhandled_level
    ),
  };

  env_logger::Builder::new().filter_level(verbosity).init();

  println!(
    "Initializing instruction read parsers (verbosity={})",
    verbosity
  );

  let input_files: Vec<_> = matches.values_of("INPUT").unwrap().collect();
  let cwd = env::current_dir()?;
  let source_file_paths = input_files.iter().map(|file| cwd.join(Path::new(file)));

  for source_file_path in source_file_paths {
    read_and_parse_file(source_file_path.as_path()).map_err(|source| source)?;
  }

  Ok(())
}

fn read_and_parse_file(path: &Path) -> std::io::Result<Vec<Instruction>> {
  let file = File::open(path)?;
  let attr = std::fs::metadata(path)?;
  let filename = path.file_name().unwrap().to_str().unwrap();
  if !attr.is_file() {
    error!("ROM file {} was not a file.", filename,)
  }
  if attr.len() % 2 == 1 {
    error!(
      "ROM file {} had an odd byte-length, but Chip8 ROMS should have event byte lengths.",
      filename,
    );
  }
  let mut reader = BufReader::new(file);
  let mut data = Vec::new();
  reader.read_to_end(&mut data)?;

  let instructions = data
    .chunks_exact(2)
    .enumerate()
    .map(|(idx, chunk)| {
      let instr_bytes = ((chunk[0] as u16) << 8) | chunk[1] as u16;
      parse_instruction(instr_bytes)
        .map_err(|source| {
          panic!(
            "Error on instruction #{}: {:X?}",
            idx,
            InstructionReadTesterError::ParseError {
              file: String::from(filename),
              source: source,
            }
          )
        })
        .unwrap()
    })
    .collect();

  Ok(instructions)
}
