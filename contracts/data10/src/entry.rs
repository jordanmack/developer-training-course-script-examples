// Import from `core` instead of from `std` since we are in no-std mode.
use core::result::Result;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_data, QueryIter};

// Import our local error codes.
use crate::error::Error;

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Load the cell data from each cell.
	for data in QueryIter::new(load_cell_data, Source::GroupOutput)
	{
		if data.len() as u64 > 10u64
		{
			return Err(Error::DataLimitExceeded);
		}
	}

	Ok(())
}
