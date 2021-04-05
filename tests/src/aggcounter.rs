use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use ckb_tool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_tool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_UNKNOWN: i8 = -1;
const ERROR_AGGCOUNTER_INVALID_TRANSACTION_STRUCTURE: i8 = 5;
const ERROR_AGGCOUNTER_INVALID_COUNTER_VALUE: i8 = 6;

#[test]
fn test_aggcounter_burn()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = vec![0u8; 8];
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
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AGGCOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_aggcounter_burn_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = vec![0u8; 8];
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
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AGGCOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_aggcounter_create()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

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
	let data = vec![0u8; 8];
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_create_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

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
	let data = vec![0u8; 8];
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_create_no_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

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
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_create_invalid_output_data_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

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
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_create_invalid_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

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
	let data = vec![0u8; 7];
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_transfer()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_transfer_high_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 1_000_000_000u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 1_000_000_001u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_transfer_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 9000u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 1_000_000_000u64.to_le_bytes().to_vec();
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
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 9001u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 1_000_000_001u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_aggcounter_transfer_multiple_mismatch_cell_count_too_few_outputs()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output.clone());
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AGGCOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_aggcounter_transfer_multiple_mismatch_cell_count_too_many_outputs()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
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
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));
	let data = 9001u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AGGCOUNTER_INVALID_TRANSACTION_STRUCTURE).input_type_script(0));
}

#[test]
fn test_aggcounter_transfer_plus_2()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 1u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 3u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AGGCOUNTER_INVALID_COUNTER_VALUE).input_type_script(0));
}

#[test]
fn test_aggcounter_transfer_plus_9000()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 1u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 9001u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AGGCOUNTER_INVALID_COUNTER_VALUE).input_type_script(0));
}

#[test]
fn test_aggcounter_transfer_minus_1()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 9001u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 9000u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_AGGCOUNTER_INVALID_COUNTER_VALUE).input_type_script(0));
}

#[test]
fn test_aggcounter_transfer_overflow_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = u64::MAX.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 0u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_UNKNOWN).input_type_script(0));
}

#[test]
fn test_aggcounter_transfer_invalid_input_data_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u32.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 1u64.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_UNKNOWN).input_type_script(0));
}

#[test]
fn test_aggcounter_transfer_invalid_output_data_panic_expected()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_aggcounter = context.deploy_cell(Loader::default().load_binary("aggcounter"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let aggcounter_dep = CellDep::new_builder().out_point(out_point_aggcounter.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_aggcounter, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u64.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data: Vec<Bytes> = vec![];
	let data = 1u32.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(aggcounter_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_UNKNOWN).input_type_script(0));
}
