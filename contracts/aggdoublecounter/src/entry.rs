// Import from `core` instead of from `std` since we are in no-std mode.
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::vec::Vec;
use alloc::collections::btree_set::BTreeSet;

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

	// Retrieve all the group input and group output cell data.
	let group_input_data: Vec<_> = QueryIter::new(load_cell_data, Source::GroupInput).collect();
	let group_output_data: Vec<_> = QueryIter::new(load_cell_data, Source::GroupOutput).collect();

	// Track the group output cells that already been matched.
	let mut group_outputs_matched: BTreeSet<usize> = BTreeSet::new();

	// Loop through all the group input cell data.
	'input_loop: for input_data in group_input_data
	{
		// Load the first input cell data value and convert the data into a u64 value.
		let mut buffer = [0u8; 8];
		buffer.copy_from_slice(&input_data[0..8]);
		let input_value_1 = u64::from_le_bytes(buffer);

		// Load the second input cell data value and convert the data into a u64 value.
		let mut buffer = [0u8; 8];
		buffer.copy_from_slice(&input_data[8..16]);
		let input_value_2 = u64::from_le_bytes(buffer);

		// Loop through all the group output cell data.
		for (o, output_data) in group_output_data.iter().enumerate()
		{
			// If the group output cell has not already matched previously.
			if !group_outputs_matched.contains(&o)
			{
				// Load the first output cell data and convert the data into a u64 value.
				let mut buffer = [0u8; 8];
				buffer.copy_from_slice(&output_data[0..8]);
				let output_value_1 = u64::from_le_bytes(buffer);

				// Load the second output cell data and convert the data into a u64 value.
				let mut buffer = [0u8; 8];
				buffer.copy_from_slice(&output_data[8..16]);
				let output_value_2 = u64::from_le_bytes(buffer);

				// Check if the output is one more than the input.
				if input_value_1 + 1 == output_value_1 && input_value_2 + 2 == output_value_2
				{
					// Mark this output as matched.
					group_outputs_matched.insert(o);

					// Continue to the next group input cell data.
					continue 'input_loop;
				}
			}
		}

		// If no match was found return an error.
		return Err(Error::InvalidCounterValue);
	}

	// Return success if all group input and output cells have been checked and no errors were found.
	Ok(())
}
