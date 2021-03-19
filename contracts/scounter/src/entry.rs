// Import from `core` instead of from `std` since we are in no-std mode.
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

	// If there isn't an exact 1:1 count, give an error.
	if group_input_count != 1 || group_output_count != 1
	{
		return Err(Error::InvalidTransactionStructure);
	}

	// Load the input cell data and verify that the length is exactly 8, which is the length of a u64.
	let input_data = load_cell_data(0, Source::GroupInput)?;
	if input_data.len() != 8
	{
		return Err(Error::InvalidInputCellData);
	}

	// Load the output cell data and verify that the length is exactly 8, which is the length of a u64.
	let output_data = load_cell_data(0, Source::GroupOutput)?;
	if output_data.len() != 8
	{
		return Err(Error::InvalidOutputCellData);
	}

	// Create a buffer to use for parsing cell data into integers.
	let mut buffer = [0u8; 8];

	// Convert the input cell data to a u64 value.
	buffer.copy_from_slice(&input_data[0..8]);
	let input_value = u64::from_le_bytes(buffer);

	// Convert the output cell data to a u64 value.
	buffer.copy_from_slice(&output_data[0..8]);
	let output_value = u64::from_le_bytes(buffer);

	// Ensure that the output value is always exactly one more that in the input value.
	if input_value + 1 != output_value
	{
		return Err(Error::InvalidCounterValue);
	}

	Ok(())
}
