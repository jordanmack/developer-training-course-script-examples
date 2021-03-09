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

use crate::error::Error;

pub fn main() -> Result<(), Error>
{
	// Load arguments from the current script.
	let script = load_script()?;
	let args: Bytes = script.args().unpack();

	// Verify that the minimum length of the arguments was given.
	if args.len() < 16
	{
		return Err(Error::ArgsLen);
	}

	// Load the output capacity amount.
	let mut buffer = [0u8; 8];
	buffer.copy_from_slice(&args[0..8]);
	let amount = u64::from_le_bytes(buffer);

	// Load the count of the number of output cells that must match the amount.
	let mut buffer2 = [0u8; 8];
	buffer2.copy_from_slice(&args[8..16]);
	let count = u64::from_le_bytes(buffer2);

	let mut found = 0;

	let output_cells: Vec<CellOutput> = QueryIter::new(load_cell, Source::Output).collect();
	for cell in output_cells.iter()
	{
		let cell_capacity: u64 = cell.capacity().unpack();
		if cell_capacity == amount
		{
			found += 1;

			if found >= count
			{
				return Ok(());
			}
		}
	}

	Err(Error::Unauthorized)
}
