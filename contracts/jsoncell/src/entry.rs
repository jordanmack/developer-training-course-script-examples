// Import from core instead of from std since we are in no-std mode.
use core::result::Result;
use core::str;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_data, QueryIter};

// Import the lite-json library for JSON parsing/validation.
use lite_json::json_parser::parse_json;

// Import our local error codes.
use crate::error::Error;

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Load the cell data from each cell.
	for data in QueryIter::new(load_cell_data, Source::GroupOutput)
	{
		// Parse the cell data into a UTF-8 string.
		let json_str = str::from_utf8(&data).map_err(|_|Error::InvalidStringData)?;

		// Validate the string as JSON by parsing it.
		parse_json(json_str).map_err(|_|Error::InvalidJson)?;
	}

	Ok(())
}
