use super::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::{ckb_error::assert_error_eq, ckb_script::ScriptError};
use ckb_tool::ckb_types::{bytes::Bytes, packed::*, prelude::*};
use ckb_tool::ckb_types::core::{TransactionBuilder};

// Constants
const MAX_CYCLES: u64 = 100_000_000;

// Error Codes
const ERROR_IC3TYPE_UNAUTHORIZED: i8 = 5;

#[test]
fn test_ic3type_invalid()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_ic3type = context.deploy_cell(Loader::default().load_binary("ic3type"));

	// Prepare Cell Deps
	let mut cell_deps = vec![];
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	cell_deps.push(always_success_dep);
	let ic3type_dep = CellDep::new_builder().out_point(out_point_ic3type.clone()).build();
	cell_deps.push(ic3type_dep);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_ic3type, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data = vec![];
	outputs_data.push(Bytes::new());

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_deps(cell_deps)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
	assert_error_eq!(err, ScriptError::ValidationFailure(ERROR_IC3TYPE_UNAUTHORIZED).input_type_script(0));
}

#[test]
fn test_ic3type_valid()
{
	// Create Context
	let mut context = Context::default();

	// Deploy Contracts
	let out_point_always_success = context.deploy_cell(ALWAYS_SUCCESS.clone());
	let out_point_ic3type = context.deploy_cell(Loader::default().load_binary("ic3type"));

	// Prepare Cell Deps
	let mut cell_deps = vec![];
	let always_success_dep = CellDep::new_builder().out_point(out_point_always_success.clone()).build();
	cell_deps.push(always_success_dep);
	let ic3type_dep = CellDep::new_builder().out_point(out_point_ic3type.clone()).build();
	cell_deps.push(ic3type_dep);

	// Prepare Scripts
	let lock_script = context.build_script(&out_point_always_success, Default::default()).expect("script");
	let type_script = context.build_script(&out_point_ic3type, Default::default()).expect("script");

	// Prepare Input Cells
	let mut inputs = vec![];
	let input_out_point = context.create_cell(CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build(), Bytes::new());
	let input = CellInput::new_builder().previous_output(input_out_point).build();
	inputs.push(input.clone());
	inputs.push(input.clone());
	inputs.push(input);

	// Prepare Output Cells
	let mut outputs = vec![];
	let output = CellOutput::new_builder().capacity(100_000_000_000u64.pack()).lock(lock_script.clone()).type_(Some(type_script.clone()).pack()).build();
	outputs.push(output);

	// Prepare Output Data
	let mut outputs_data = vec![];
	outputs_data.push(Bytes::new());

	// Build Transaction
	let tx = TransactionBuilder::default()
		.inputs(inputs)
		.outputs(outputs)
		.outputs_data(outputs_data.pack())
		.cell_deps(cell_deps)
		.build();
	let tx = context.complete_tx(tx);

	// Run
	let _cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
	// println!("consume cycles: {}", cycles);
}
