use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use ckb_tool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_tool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_UNKNOWN: i8 = -1;
const ERROR_DOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE: i8 = 5;
const ERROR_DOUBLECOUNTER_INVALID_COUNTER_VALUE_1: i8 = 6;
const ERROR_DOUBLECOUNTER_INVALID_COUNTER_VALUE_2: i8 = 7;

#[test]
fn test_doublecounter_burn()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(vec![0u8; 16]));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![];
	let outputs_data: Vec<Bytes> = vec![];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_DOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_doublecounter_burn_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(vec![0u8; 16]));
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
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_DOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_doublecounter_create()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build()];
	let outputs_data = vec![Bytes::from(vec![0u8; 16])];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_doublecounter_create_no_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

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
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_doublecounter_create_invalid_output_data_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

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
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_doublecounter_create_invalid_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	let outputs = vec![CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build()];
	let outputs_data = vec![Bytes::from(vec![0])];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.input(input)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_doublecounter_transfer()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 0u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 0u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 2u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_doublecounter_transfer_high_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1_000_000_001u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_002u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_doublecounter_invalid_transfer_plus_2_value_1()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1_000_000_002u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_002u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_DOUBLECOUNTER_INVALID_COUNTER_VALUE_1).input_type_script(0));
}

#[test]
fn test_doublecounter_invalid_transfer_plus_1_value_2()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1_000_000_001u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_001u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_DOUBLECOUNTER_INVALID_COUNTER_VALUE_2).input_type_script(0));
}

#[test]
fn test_doublecounter_invalid_transfer_plus_9000_value_1()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1_000_009_000u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_002u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_DOUBLECOUNTER_INVALID_COUNTER_VALUE_1).input_type_script(0));
}

#[test]
fn test_doublecounter_invalid_transfer_minus_1_value_1()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 1_000_000_001u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_001u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1_000_000_000u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1_000_000_003u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_DOUBLECOUNTER_INVALID_COUNTER_VALUE_1).input_type_script(0));
}

#[test]
fn test_doublecounter_transfer_overflow_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = u64::MAX.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = u64::MAX.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 0u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 1u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_UNKNOWN).input_type_script(0));
}

#[test]
fn test_doublecounter_transfer_invalid_input_data_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 0u32.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 0u32.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1u32.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 2u32.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_UNKNOWN).input_type_script(0));
}

#[test]
fn test_doublecounter_transfer_invalid_output_data_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_doublecounter = context.deploy_cell(Loader::default().load_binary("doublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let doublecounter_dep = CellDep::new_builder().out_point(out_point_doublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_doublecounter, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	let mut counter1 = 0u64.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 0u64.to_le_bytes().to_vec();
	data.append(&mut counter2);
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	let mut data = vec!();
	let mut counter1 = 1u32.to_le_bytes().to_vec();
	data.append(&mut counter1);
	let mut counter2 = 2u32.to_le_bytes().to_vec();
	data.append(&mut counter2);
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(doublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_UNKNOWN).input_type_script(0));
}
