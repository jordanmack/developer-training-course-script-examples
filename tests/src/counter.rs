use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_testtool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_testtool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_UNKNOWN: i8 = -1;
const ERROR_COUNTER_INVALID_TRANSACTION_STRUCTURE: i8 = 5;
const ERROR_COUNTER_INVALID_COUNTER_VALUE: i8 = 6;

#[test]
fn test_counter_burn()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(vec![0u8; 8]));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![];
	let outputs_data: Vec<Bytes> = vec![];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_COUNTER_INVALID_TRANSACTION_STRUCTURE);
}

#[test]
fn test_counter_burn_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(vec![0u8; 8]));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let inputs = vec![input.clone(), input];
	let outputs = vec![];
	let outputs_data: Vec<Bytes> = vec![];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_COUNTER_INVALID_TRANSACTION_STRUCTURE);
}

#[test]
fn test_counter_create()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build()];
	let outputs_data = vec![Bytes::from(vec![0u8; 8])];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_counter_create_no_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build()];
	let outputs_data = vec![Bytes::new(); outputs.len()];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_counter_create_invalid_output_data_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build()];
	let outputs_data = vec![Bytes::from(vec![1])];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_counter_create_invalid_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build()];
	let outputs_data = vec![Bytes::from(vec![0, 0, 0, 0, 0, 0, 0])];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_counter_transfer()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(vec![0u8; 8]));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(1u64.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_counter_transfer_high_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(1_000_000_000u64.to_le_bytes().to_vec()));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(1_000_000_001u64.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_counter_transfer_plus_2()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(1u64.to_le_bytes().to_vec()));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(3u64.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_COUNTER_INVALID_COUNTER_VALUE);
}

#[test]
fn test_counter_transfer_plus_9000()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(1u64.to_le_bytes().to_vec()));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(9001u64.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_COUNTER_INVALID_COUNTER_VALUE);
}

#[test]
fn test_counter_transfer_minus_1()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(9001u64.to_le_bytes().to_vec()));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(9000u64.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_COUNTER_INVALID_COUNTER_VALUE);
}

#[test]
fn test_counter_transfer_overflow_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(u64::MAX.to_le_bytes().to_vec()));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(0u64.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_UNKNOWN);
}

#[test]
fn test_counter_transfer_invalid_input_data_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(0u32.to_le_bytes().to_vec()));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(1u64.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_UNKNOWN);
}

#[test]
fn test_counter_transfer_invalid_output_data_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_counter = context.deploy_cell(Loader::default().load_binary("counter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let counter_dep = CellDep::new_builder().out_point(out_point_counter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_counter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(0u64.to_le_bytes().to_vec()));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from(1u32.to_le_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(counter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_UNKNOWN);
}
