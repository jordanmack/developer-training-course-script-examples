use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_testtool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_testtool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_DATACAP_ARGSLEN: i8 = 5;
const ERROR_DATACAP_DATA_LIMIT_EXCEEDED: i8 = 6;

#[test]
fn test_datacap_valid_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_datacap = context.deploy_cell(Loader::default().load_binary("datacap"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let datacap_dep = CellDep::new_builder().out_point(out_point_datacap.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let args = 16u32.to_le_bytes().to_vec();
	let type_script = context.build_script(&out_point_datacap, Bytes::from(args)).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from("Hello World!".as_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(datacap_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_datacap_empty_data()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_datacap = context.deploy_cell(Loader::default().load_binary("datacap"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let datacap_dep = CellDep::new_builder().out_point(out_point_datacap.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let args = 16u32.to_le_bytes().to_vec();
	let type_script = context.build_script(&out_point_datacap, Bytes::from(args)).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from("".as_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(datacap_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}

#[test]
fn test_datacap_empty_args()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_datacap = context.deploy_cell(Loader::default().load_binary("datacap"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let datacap_dep = CellDep::new_builder().out_point(out_point_datacap.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_datacap, Default::default()).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from("".as_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(datacap_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_DATACAP_ARGSLEN);
}

#[test]
fn test_datacap_data_limit_exceeded()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_datacap = context.deploy_cell(Loader::default().load_binary("datacap"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let datacap_dep = CellDep::new_builder().out_point(out_point_datacap.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let args = 16u32.to_le_bytes().to_vec();
	let type_script = context.build_script(&out_point_datacap, Bytes::from(args)).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let mut outputs = vec![];
	let mut outputs_data: Vec<Bytes> = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);
	let output = CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);
	outputs_data.push(Bytes::from("Hello World! Hello World!".as_bytes().to_vec()));

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_dep(always_success_dep)
		.cell_dep(datacap_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_script_error(err, ERROR_DATACAP_DATA_LIMIT_EXCEEDED);
}

#[test]
fn test_datacap_burn()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_datacap = context.deploy_cell(Loader::default().load_binary("datacap"));

	// Prepare Cell Deps
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	let datacap_dep = CellDep::new_builder().out_point(out_point_datacap.clone()).build();

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let args = 16u32.to_le_bytes().to_vec();
	let type_script = context.build_script(&out_point_datacap, Bytes::from(args)).expect("script");

	// Prepare Cells
	let mut inputs = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000_u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.cell_dep(always_success_dep)
		.cell_dep(datacap_dep)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}
