// Import from core instead of from std since we are in no-std mode.
use core::result::Result;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};
use ckb_std::high_level::{load_script, load_cell_lock_hash, load_cell_data, QueryIter};

// Import our local error codes.
use crate::error::Error;

// Constants
const LOCK_HASH_LEN: usize = 32; // Number of bytes for a lock hash. (Blake2b 256-bit 32 bytes)
const SUDT_DATA_LEN: usize = 16; // SUDT uses a u128, which is 16 bytes.

/// Determine if owner mode is enabled.
fn check_owner_mode(args: &Bytes) -> Result<bool, Error>
{
	// Verify the the arguments length matches the length of a Blake2b hash.
	if args.len() != LOCK_HASH_LEN
	{
		return Err(Error::ArgsLength);
	}

	// Compare the Lock Script Hash from the script args with the Lock Scripts
	// of each input cell to determine if a match exists.
	let is_owner_mode = QueryIter::new(load_cell_lock_hash, Source::Input)
		.find(|lock_hash|args[..]==lock_hash[..]).is_some();

	// Return the owner mode status.
	Ok(is_owner_mode)
}

/// Count the number of tokens in the specified source. Source should be either GroupInput or GroupOutput.
fn determine_token_amount(source: Source) -> Result<u128, Error>
{
	// Track the number of tokens that are counted.
	let mut total_token_amount = 0;

	// Cycle through the data in each cell within the specified source.
	let cell_data = QueryIter::new(load_cell_data, source);
	for data in cell_data
	{
		// Check that the length of the data is >= 16 bytes, the size of a u128.
		if data.len() >= SUDT_DATA_LEN
		{
			// Convert the binary data in the cell to a u128 value.
			let mut buffer = [0u8; SUDT_DATA_LEN];
			buffer.copy_from_slice(&data[0..SUDT_DATA_LEN]);
			let amount = u128::from_le_bytes(buffer);

			// Add the amount of tokens in the cell to the total amount of tokens.
			total_token_amount += amount;
		}
		// If the data is less than 16 bytes, then return an encoding error.
		else
		{
			return Err(Error::Encoding);
		}
	}

	// Return the total amount of tokens found in the specified source.
	Ok(total_token_amount)
}


// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Load the currently executing script and get the args.
	let script = load_script()?;
	let args: Bytes = script.args().unpack();

	// Check if the script is being run by the owner and immediately return success if true.
	if check_owner_mode(&args)?
	{
		return Ok(());
	}

	// Count the number of tokens in the GroupInput and GroupOutput.
	let input_token_amount = determine_token_amount(Source::GroupInput)?;
	let output_token_amount = determine_token_amount(Source::GroupOutput)?;

	// If the amount of input tokens is less than the amount of output tokens, return an error.   
	if input_token_amount < output_token_amount
	{
		return Err(Error::Amount);
	}

	// No errors were found during validation. Return success.
	Ok(())
}
