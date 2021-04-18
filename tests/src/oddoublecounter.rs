use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use ckb_tool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_tool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_ODDOUBLECOUNTER_VALUE_OVERFLOW: i8 = 5;
const ERROR_ODDOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE: i8 = 6;
const ERROR_ODDOUBLECOUNTER_INVALID_INPUT_CELL_DATA: i8 = 7;
const ERROR_ODDOUBLECOUNTER_INVALID_OUTPUT_CELL_DATA: i8 = 8;
const ERROR_ODDOUBLECOUNTER_INVALID_COUNTER_VALUE_1: i8 = 9;
const ERROR_ODDOUBLECOUNTER_INVALID_COUNTER_VALUE_2: i8 = 10;

#[test]
fn test_oddoublecounter_burn()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let outputs = vec![];

	// Prepare Output Data
	let outputs_data: Vec<Bytes> = vec![];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_oddoublecounter_burn_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let outputs = vec![];

	// Prepare Output Data
	let outputs_data: Vec<Bytes> = vec![];

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_oddoublecounter_create()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_oddoublecounter_create_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input.clone());
	inputs.push(input.clone());
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output.clone());
	outputs.push(output.clone());
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE).output_type_script(0));
}

#[test]
fn test_oddoublecounter_create_no_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	outputs_data.push(Bytes::new());

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_OUTPUT_CELL_DATA).output_type_script(0));
}

#[test]
fn test_oddoublecounter_create_invalid_output_data_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = vec![1u8; 1];
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_OUTPUT_CELL_DATA).output_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1u64.to_le_bytes().to_vec());
	data.append(&mut 2u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_oddoublecounter_transfer_high_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 1_000_000_000u64.to_le_bytes().to_vec());
	data.append(&mut 2_000_000_000u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1_000_000_001u64.to_le_bytes().to_vec());
	data.append(&mut 2_000_000_002u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_oddoublecounter_transfer_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.append(&mut 9000u64.to_le_bytes().to_vec());
	data.append(&mut 9000u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.append(&mut 1_000_000u64.to_le_bytes().to_vec());
	data.append(&mut 2_000_000u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output.clone());
	outputs.push(output.clone());
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1u64.to_le_bytes().to_vec());
	data.append(&mut 2u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));
	let mut data = vec!();
	data.append(&mut 9001u64.to_le_bytes().to_vec());
	data.append(&mut 9002u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));
	let mut data = vec!();
	data.append(&mut 1_000_001u64.to_le_bytes().to_vec());
	data.append(&mut 2_000_002u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer_multiple_mismatch_cell_count_too_few_outputs()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input.clone());
	inputs.push(input.clone());
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output.clone());
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1u64.to_le_bytes().to_vec());
	data.append(&mut 2u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer_multiple_mismatch_cell_count_too_many_outputs()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input.clone());
	inputs.push(input.clone());
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output.clone());
	outputs.push(output.clone());
	outputs.push(output.clone());
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1u64.to_le_bytes().to_vec());
	data.append(&mut 2u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer_value_1_plus_2()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 2u64.to_le_bytes().to_vec());
	data.append(&mut 2u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_COUNTER_VALUE_1).input_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer_value_2_plus_9000()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1u64.to_le_bytes().to_vec());
	data.append(&mut 9000u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_COUNTER_VALUE_2).input_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer_overflow()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut u64::MAX.to_le_bytes().to_vec());
	data.append(&mut u64::MAX.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 1u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_VALUE_OVERFLOW).input_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer_invalid_input_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u32.to_le_bytes().to_vec());
	data.append(&mut 0u32.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1u64.to_le_bytes().to_vec());
	data.append(&mut 2u64.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_INPUT_CELL_DATA).input_type_script(0));
}

#[test]
fn test_oddoublecounter_transfer_invalid_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_oddoublecounter = context.deploy_cell(Loader::default().load_binary("oddoublecounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggdoublecounter_dep = CellDep::new_builder().out_point(out_point_oddoublecounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_oddoublecounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 0u64.to_le_bytes().to_vec());
	data.append(&mut 0u64.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let mut data = vec!();
	data.append(&mut 1u32.to_le_bytes().to_vec());
	data.append(&mut 2u32.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggdoublecounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_ODDOUBLECOUNTER_INVALID_OUTPUT_CELL_DATA).input_type_script(0));
}
