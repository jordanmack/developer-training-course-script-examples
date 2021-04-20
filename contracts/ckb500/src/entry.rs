// Import from core instead of from std since we are in no-std mode.
use core::result::Result;

// Import heap related library from alloc.
// https://doc.rust-lang.org/alloc/index.html
use alloc::vec::Vec;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{prelude::*};
use ckb_std::ckb_types::{packed::CellOutput};
use ckb_std::high_level::{load_cell, QueryIter};

use crate::error::Error;

pub fn main() -> Result<(), Error>
{
	let mut total_input_capacity = 0;
	
	let input_cells: Vec<CellOutput> = QueryIter::new(load_cell, Source::Input).collect();
	for cell in input_cells.iter()
	{
		total_input_capacity += cell.capacity().unpack();
	}

	if total_input_capacity == 50_000_000_000
	{
		Ok(())
	}
	else
	{
		Err(Error::Unauthorized)
	}
}

