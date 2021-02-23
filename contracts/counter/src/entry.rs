// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
// use alloc::vec;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
// use ckb_std::{debug};
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_data, load_cell_type_hash};
use ckb_std::syscalls::SysError;

// Local module imports.
use crate::error::Error;

// The modes of operation for the script. 
enum Mode
{
	Burn, // Consume an existing counter cell.
	Create, // Create a new counter cell.
	Transfer, // Transfer (update) a counter cell and increase its value.
}

// Determines the mode of operation for the currently executing script.
fn determine_mode() -> Result<Mode, Error>
{
	// Keep track of the matching cells.
	let mut matching_group_input_count = 0;
	let mut matching_group_output_count = 0;

	// Tally the number of cells with matching type scripts on the inputs and outputs.
	for source in &[Source::GroupInput, Source::GroupOutput]
	{
		// Loop through all the cells.
		let mut i = 0;
		loop
		{
			// Get the type script hash. We could use multiple system calls, but we use this because it can be used to test if a type script exists.
			let type_script = load_cell_type_hash(i, *source);

			// Check if load_cell_type_hash() executed successfully.
			if let Ok(ref type_script) = type_script
			{
				// If the type script exists. 
				if type_script.is_some()
				{
					// Increment the variable based on which input source is being used.
					if source == &Source::GroupInput
					{
						matching_group_input_count += 1;
					}
					else
					{
						matching_group_output_count += 1;
					}
				}
				// If a type script doesn't exist, there is a problem. This script might be being used as a lock script.
				else
				{
					return Err(Error::InvalidTransactionStructure);
				}
			}

			// Check if a syscall error was received.
			if let Err(error) = type_script
			{
				// If we get an out of bounds error, we are at the end of the source array. This is expected.
				if error == SysError::IndexOutOfBound
				{
					break;
				}
				// We received some kind of unexpected syscall error.
				else
				{
					return Err(error.into());
				}
			}
	
			i += 1;
		}
	}

	// Detect the operation based on the cell count.
	if matching_group_input_count == 1 && matching_group_output_count == 0
	{
		return Ok(Mode::Burn);
	}
	if matching_group_input_count == 0 && matching_group_output_count == 1
	{
		return Ok(Mode::Create);
	}
	if matching_group_input_count == 1 && matching_group_output_count == 1
	{
		return Ok(Mode::Transfer);
	}

	// If no known code structure was used, return an error.
	Err(Error::InvalidTransactionStructure)
}

// Validate a transaction to create a counter cell.
fn validate_create() -> Result<(), Error>
{
	// Load the output cell data and verify that the value is 0u64.
	let cell_data = load_cell_data(0, Source::GroupOutput)?;
	if cell_data != 0u64.to_le_bytes().to_vec()
	{
		return Err(Error::InvalidOutputCellData);	
	}

	Ok(())
}

// Validate a transaction to transfer (update) a counter cell and increase its value.
fn validate_transfer() -> Result<(), Error>
{
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

	// Check for an overflow scenario.
	if input_value == u64::MAX
	{
		return Err(Error::CounterValueOverflow);
	}

	// Ensure that the output value is always exactly one more that in the input value.
	if input_value + 1 != output_value
	{
		return Err(Error::InvalidCounterValue);
	}

	Ok(())
}

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Determine the mode and validate as needed.
	match determine_mode()
	{
		Ok(Mode::Burn) => return Ok(()),
		Ok(Mode::Create) => validate_create()?,
		Ok(Mode::Transfer) => validate_transfer()?,
		Err(e) => return Err(e),
	}

	Ok(())
}
