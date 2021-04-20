// Import from core instead of from std since we are in no-std mode.
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_cell_data, QueryIter};

// Import local modules.
use crate::error::Error;

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Count on the number of group input and groupt output cells.
	let group_input_count = QueryIter::new(load_cell, Source::GroupInput).count();
	let group_output_count = QueryIter::new(load_cell, Source::GroupOutput).count();

	// If there are no inputs, skip validation.
	if group_input_count == 0
	{
		return Ok(());
	}

	// If there isn't an exact 1:1 ratio, give an error.
	if group_input_count != group_output_count
	{
		return Err(Error::InvalidTransactionStructure);
	}

	// Loop through all the group input cell data.
	for i in 0..group_input_count
	{
		// Load the input and output data at the current index.
		let input_data = load_cell_data(i, Source::GroupInput)?;
		let output_data = load_cell_data(i, Source::GroupOutput)?;

		// Convert the input cell data into a u64 value.
		let mut buffer = [0u8; 8];
		buffer.copy_from_slice(&input_data[0..8]);
		let input_value = u64::from_le_bytes(buffer);

		// Convert the output cell data into a u64 value.
		let mut buffer = [0u8; 8];
		buffer.copy_from_slice(&output_data[0..8]);
		let output_value = u64::from_le_bytes(buffer);

		// Check if the output is one more than the input.
		if input_value + 1 != output_value
		{
			// If no match was found return an error.
			return Err(Error::InvalidCounterValue);
		}
	}

	// Return success if all group input and output cells have been checked and no errors were found.
	Ok(())
}
