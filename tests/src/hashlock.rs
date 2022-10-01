use super::*;
use ckb_testtool::{context::Context};
use ckb_testtool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_testtool::ckb_types::core::{TransactionBuilder};
use blake2b_ref::Blake2bBuilder;

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_HASHLOCK_UNAUTHORIZED: i8 = 5;
const ERROR_HASHLOCK_ARGSLEN: i8 = 6;
const ERROR_HASHLOCK_WITNESSLOAD: i8 = 7;

#[test]
fn test_hashlock_valid()
{
	// Setup Preimage and Hash
	let preimage = "Open Sesame".as_bytes();
	let mut hash = [0u8; 32];
	let mut blake2b = Blake2bBuilder::new(32).build();
	blake2b.update(&preimage);
	blake2b.finalize(&mut hash);

	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("hashlock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let args = hash.to_vec();
	let lock_script = context.build_script(&out_point, Bytes::from(args)).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Prepare Witnesses
	let witnesses = vec!(Bytes::from(preimage));

	// Build Transaction
	let tx = TransactionBuilder::default().input(input).outputs(outputs).outputs_data(outputs_data.pack()).cell_dep(lock_script_dep).witnesses(witnesses.pack()).build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_hashlock_no_witness()
{
	// Setup Preimage and Hash
	let preimage = "Open Sesame".as_bytes();
	let mut hash = [0u8; 32];
	let mut blake2b = Blake2bBuilder::new(32).build();
	blake2b.update(&preimage);
	blake2b.finalize(&mut hash);

	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("hashlock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let args = hash.to_vec();
	let lock_script = context.build_script(&out_point, Bytes::from(args)).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Build Transaction
	let tx = TransactionBuilder::default().input(input).outputs(outputs).outputs_data(outputs_data.pack()).cell_dep(lock_script_dep).build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_HASHLOCK_WITNESSLOAD);
}

#[test]
fn test_hashlock_no_args()
{
	// Setup Preimage and Hash
	let preimage = "Open Sesame".as_bytes();
	let mut hash = [0u8; 32];
	let mut blake2b = Blake2bBuilder::new(32).build();
	blake2b.update(&preimage);
	blake2b.finalize(&mut hash);

	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("hashlock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point, Bytes::new()).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Prepare Witnesses
	let witnesses = vec!(Bytes::from(preimage));

	// Build Transaction
	let tx = TransactionBuilder::default().input(input).outputs(outputs).outputs_data(outputs_data.pack()).cell_dep(lock_script_dep).witnesses(witnesses.pack()).build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_HASHLOCK_ARGSLEN);
}

#[test]
fn test_hashlock_incorrect()
{
	// Setup Preimage and Hash
	let preimage = "Open Sesame".as_bytes();
	let mut hash = [0u8; 32];
	let mut blake2b = Blake2bBuilder::new(32).build();
	blake2b.update(&preimage);
	blake2b.finalize(&mut hash);

	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("hashlock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let args = hash.to_vec();
	let lock_script = context.build_script(&out_point, Bytes::from(args)).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Prepare Witnesses
	let preimage = "Guacamole".as_bytes();
	let witnesses = vec!(Bytes::from(preimage));

	// Build Transaction
	let tx = TransactionBuilder::default().input(input).outputs(outputs).outputs_data(outputs_data.pack()).cell_dep(lock_script_dep).witnesses(witnesses.pack()).build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_HASHLOCK_UNAUTHORIZED);
}
