extern crate tetsy_wasm;
extern crate twasm_utils as utils;
extern crate twasm_utils_cli as logger;

use std::env;

fn main() {
	logger::init_log();

	let args = env::args().collect::<Vec<_>>();
	if args.len() != 3 {
		println!("Usage: {} input_file.wasm output_file.wasm", args[0]);
		return;
	}

	// Loading module
	let module = tetsy_wasm::deserialize_file(&args[1]).expect("Module deserialization to succeed");

	let result = utils::inject_gas_counter(
		module, &Default::default()
	).expect("Failed to inject gas. Some forbidden opcodes?");

	tetsy_wasm::serialize_to_file(&args[2], result).expect("Module serialization to succeed")
}
