use super::*;
use ckb_testtool::{context::Context};
use ckb_testtool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_testtool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_ICCLOCK_UNAUTHORIZED: i8 = 5;
const ERROR_ICCLOCK_ARGSLEN: i8 = 6;

#[test]
fn test_icclock_minimum_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("icclock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point, Bytes::from(50_000_000_000u64.to_le_bytes().to_vec())).expect("script");
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
	assert_script_error(err, ERROR_ICCLOCK_UNAUTHORIZED);
}

#[test]
fn test_icclock_exact_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("icclock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context
	.build_script(&out_point, Bytes::from(50_000_000_000u64.to_le_bytes().to_vec())).expect("script");
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
fn test_icclock_multi_cell_exact_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("icclock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context
	.build_script(&out_point, Bytes::from(50_000_000_000u64.to_le_bytes().to_vec())).expect("script");
	let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(50_000_000_000u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
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
fn test_icclock_over_capacity()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("icclock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let lock_script = context
	.build_script(&out_point, Bytes::from(50_000_000_000u64.to_le_bytes().to_vec())).expect("script");
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
	assert_script_error(err, ERROR_ICCLOCK_UNAUTHORIZED);
}

#[test]
fn test_icclock_no_args()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("icclock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let args = vec!();
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
	assert_script_error(err, ERROR_ICCLOCK_ARGSLEN);
}

#[test]
fn test_icclock_wrong_args()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let binary: Bytes = Loader::default().load_binary("icclock");
	let out_point = context.deploy_cell(binary);

	// Prepare Scripts
	let mut args = vec!();
	let mut amount = 50_000_000_000u64.to_le_bytes().to_vec();
	let mut zero = 0u64.to_le_bytes().to_vec();
	args.append(&mut amount);
	args.append(&mut zero);
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
	assert_script_error(err, ERROR_ICCLOCK_ARGSLEN);
}
