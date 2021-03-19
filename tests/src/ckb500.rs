use super::*;
// use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_testtool::{context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use ckb_tool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_tool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_CKB500_UNAUTHORIZED: i8 = 5;

#[test]
fn test_ckb500_minimum_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("ckb500");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point, Default::default()).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(6_100_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(6_100_000_000u64.pack()).lock(lock_script.clone()).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Build Transaction
	let tx = TransactionBuilder::default().input(input).outputs(outputs).outputs_data(outputs_data.pack()).cell_dep(lock_script_dep).build();
	let tx = context.complete_tx(tx);

	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_CKB500_UNAUTHORIZED).input_lock_script(0));
}

#[test]
fn test_ckb500_exact_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("ckb500");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point, Default::default()).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(50_000_000_000u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(50_000_000_000u64.pack()).lock(lock_script.clone()).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Build Transaction
	let tx = TransactionBuilder::default().input(input).outputs(outputs).outputs_data(outputs_data.pack()).cell_dep(lock_script_dep).build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_ckb500_multi_cell_exact_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("ckb500");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point, Default::default()).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(50_000_000_000u64.pack()).lock(lock_script.clone()).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Build Transaction
	let tx = TransactionBuilder::default().input(input.clone()).input(input.clone()).input(input.clone()).input(input.clone()).input(input).outputs(outputs).outputs_data(outputs_data.pack()).cell_dep(lock_script_dep).build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_ckb500_over_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("ckb500");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point, Default::default()).expect("script");
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
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_CKB500_UNAUTHORIZED).input_lock_script(0));
}
