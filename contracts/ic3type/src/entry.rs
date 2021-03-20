// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::load_cell;
use ckb_std::error::SysError;

// Import local modules.
use crate::error::Error;

// Constants.
const CELLS_REQUIRED: u64 = 3;

// Main entry point.
pub fn main() -> Result<(), Error>
{
	// Track the number of input cells that exist.
	let mut cell_count = 0;

	// Cycle through all the input cells to count them.
	let mut i = 0;
	loop
	{
		match load_cell(i, Source::Input)
		{
			Ok(_cell) => cell_count += 1,
			Err(SysError::IndexOutOfBound) => break,
			Err(e) => return Err(e.into())
		}
		i += 1;
	}

	// If our cell count matches the requirement then exit successfully.
	if cell_count == CELLS_REQUIRED
	{
		return Ok(());
	}

	// Return an error if the cell count is incorrect.
	Err(Error::Unauthorized)
}
