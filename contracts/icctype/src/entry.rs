// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec::Vec};

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{prelude::*};
use ckb_std::ckb_types::{bytes::Bytes, packed::CellOutput};
use ckb_std::high_level::{load_cell, load_script, QueryIter};

// Import local modules.
use crate::error::Error;

// Constants
const CELLS_REQUIRED_DEFAULT: u64 = 3;

pub fn main() -> Result<(), Error>
{
	// Load arguments from the current script.
	let script = load_script()?;
	let args: Bytes = script.args().unpack();

	// Verify that a valid length of the arguments was given.
	let args_len = args.len();
	if args_len != 0 && args_len != 8 // We expect either nothing, or an 8 byte u64 LE value.
	{
		return Err(Error::ArgsLen);
	}

	// Set the number of input cells required. 
	let mut cells_required = CELLS_REQUIRED_DEFAULT;
	if args_len == 8
	{
		let mut buffer = [0u8; 8];
		buffer.copy_from_slice(&args[0..8]);
		cells_required = u64::from_le_bytes(buffer);
	}

	// Track the number of input cells that exist.
	let mut cell_count = 0;

	// Cycle through all the input cells to count them.
	let input_cells: Vec<CellOutput> = QueryIter::new(load_cell, Source::Input).collect();
	for _ in input_cells.iter()
	{
		cell_count += 1;
	}

	// If our cell count matches the requirement then exit successfully.
	if cell_count == cells_required
	{
		return Ok(());
	}

	// Return an error if the cell count is incorrect.
	Err(Error::Unauthorized)
}
