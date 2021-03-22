// Import from `core` instead of from `std` since we are in no-std mode.
use core::result::Result;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::prelude::*;
use ckb_std::high_level::{load_cell, load_cell_data, load_script, QueryIter};

// Import our local error codes.
use crate::error::Error;

// Constants.
const MAX_DATA_SIZE: usize = 10;

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Load the current script.
	let script = load_script()?;

	// Load each cell from the outputs.
	let mut i = 0;
	for cell in QueryIter::new(load_cell, Source::Output)
	{
		// Check if there is a type script, and skip to the next cell if there is not.
		let cell_type_script = &cell.type_();
		if cell_type_script.is_none()
		{
			continue;
		}

		// Convert the scripts to bytes and check if they are the same.
		let cell_type_script = cell_type_script.to_opt().unwrap();
		if *script.as_bytes() == *cell_type_script.as_bytes()
		{
			// Load the cell's data.
			let data = load_cell_data(i, Source::Output)?;

			// If the data is larger than our limit.
			if data.len() > MAX_DATA_SIZE
			{
				// Return a limit exceeded error.
				return Err(Error::DataLimitExceeded);
			}
		}

		// Increment the index to process the next cell.
		i += 1;
	}

	Ok(())
}
