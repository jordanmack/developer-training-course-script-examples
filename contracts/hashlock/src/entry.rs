// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
// use alloc::{vec::Vec};

// Import the Blake2b reference library.
use blake2b_ref::Blake2bBuilder;

// Import CKB syscalls and structures.
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
// use ckb_std::{debug};
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::{prelude::*};
use ckb_std::ckb_types::{bytes::Bytes};
use ckb_std::high_level::{load_script};
use ckb_std::syscalls::load_witness;

use crate::error::Error;

// Constants
const WITNESS_SIZE_LIMIT: usize = 1024;

pub fn main() -> Result<(), Error>
{
	// Load arguments from the current script.
	let script = load_script()?;
	let args: Bytes = script.args().unpack();

	// Verify that the minimum length of the arguments was given.
	if args.len() < 32
	{
		return Err(Error::ArgsLen);
	}

	// Load the hash from the arguments.
	let args_hash = &args[0..32];

	// Load the preimage from the witness.
	let mut witness = [0u8; WITNESS_SIZE_LIMIT];
	let witness_len = load_witness(&mut witness, 0, 0, Source::GroupInput).map_err(|_|Error::WitnessLoadFail)?;

	// Hash the preimage.
	let mut witness_hash = [0u8; 32];
    let mut blake2b = Blake2bBuilder::new(32).build();
    blake2b.update(&witness[0..witness_len]);
    blake2b.finalize(&mut witness_hash);

	// Check if the witness preimage matches the hash from the arguments.
	if args_hash == witness_hash
	{
		return Ok(());
	}

	Err(Error::Unauthorized)
}
