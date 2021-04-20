use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use ckb_tool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_tool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_SUDT_ENCODING: i8 = 4;
const ERROR_SUDT_AMOUNT: i8 = 5;
const ERROR_SUDT_ARGS_LENGTH: i8 = 6;

#[test]
fn test_sudt_burn()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 9_000u128.to_le_bytes().to_vec();
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
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_burn_zero_token_cell()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u128.to_le_bytes().to_vec();
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
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_burn_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 9_000u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input.clone());
	inputs.push(input.clone());
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
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_burn_multiple_zero_token_cells()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 0u128.to_le_bytes().to_vec();
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input.clone());
	inputs.push(input.clone());
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
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_create()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_owner: [u8; 32] = lock_script.calc_script_hash().unpack();
	let script_args: Bytes = lock_script_hash_owner.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
	let data = 9_000u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_create_no_owner()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
	let data = 9_000u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_SUDT_AMOUNT).output_type_script(0));
}

#[test]
fn test_sudt_create_zero_token_cell()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_owner: [u8; 32] = lock_script.calc_script_hash().unpack();
	let script_args: Bytes = lock_script_hash_owner.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
	let data = 0u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_create_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_owner: [u8; 32] = lock_script.calc_script_hash().unpack();
	let script_args: Bytes = lock_script_hash_owner.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
	let data = 9_000u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_create_multiple_zero_token_cell()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_owner: [u8; 32] = lock_script.calc_script_hash().unpack();
	let script_args: Bytes = lock_script_hash_owner.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
	let data = 0u128.to_le_bytes().to_vec();
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data.clone()));
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_create_no_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_owner: [u8; 32] = lock_script.calc_script_hash().unpack();
	let script_args: Bytes = lock_script_hash_owner.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_create_no_script_args()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let script_args: Bytes = vec!().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_SUDT_ARGS_LENGTH).output_type_script(0));
}

#[test]
fn test_sudt_create_invalid_output_data_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_owner: [u8; 32] = lock_script.calc_script_hash().unpack();
	let script_args: Bytes = lock_script_hash_owner.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

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
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_transfer()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 1u128.to_le_bytes().to_vec();
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
	data.append(&mut 1u128.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_transfer_high_value()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 1_000_000_000u128.to_le_bytes().to_vec());
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
	data.append(&mut 1_000_000_000u128.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_transfer_multiple()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 9000u128.to_le_bytes().to_vec());
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(10_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::from(data));
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let mut data = vec!();
	data.append(&mut 1_000_000u128.to_le_bytes().to_vec());
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
	let mut data = vec!();
	data.append(&mut 9000u128.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));
	let mut data = vec!();
	data.append(&mut 1_000_000u128.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_sudt_transfer_invalid_input_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let mut data = vec!();
	data.append(&mut 1u32.to_le_bytes().to_vec());
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
	data.append(&mut 1u128.to_le_bytes().to_vec());
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_SUDT_ENCODING).input_type_script(0));
}

#[test]
fn test_sudt_transfer_invalid_output_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_sudt = context.deploy_cell(Loader::default().load_binary("sudt"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let sudt_dep = CellDep::new_builder().out_point(out_point_sudt.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let lock_script_hash_zero = [0u8; 32];
	let script_args: Bytes = lock_script_hash_zero.to_vec().into();
	let type_script = context.build_script(&out_point_sudt, script_args).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let data = 1u128.to_le_bytes().to_vec();
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
	outputs_data.push(Bytes::from(data));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(sudt_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_SUDT_ENCODING).input_type_script(0));
}
