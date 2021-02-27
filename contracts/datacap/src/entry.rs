// Import from `core` instead of from `std` since we are in no-std mode.
use core::result::Result;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};
use ckb_std::high_level::{load_cell_data, load_script, QueryIter};

// Import our local error codes.
use crate::error::Error;

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Load arguments from the current script.
	let script = load_script()?;
	let args: Bytes = script.args().unpack();

	// Verify that correct length of arguments was given.
	if args.len() != 8
	{
		return Err(Error::ArgsLen);
	}

	// Load the cell_data_limit from the script args.
	let mut buffer = [0u8; 8];
	buffer.copy_from_slice(&args[0..8]);
	let cell_data_limit = u64::from_le_bytes(buffer);

	// Load the cell data from each cell.
	for data in QueryIter::new(load_cell_data, Source::GroupOutput)
	{
		if data.len() as u64 > cell_data_limit
		{
			return Err(Error::DataLimitExceeded);
		}
	}

	Ok(())
}
