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

	let module = utils::externalize(
		tetsy_wasm::deserialize_file(&args[1]).expect("Module to deserialize ok"),
		vec!["_free", "_malloc", "_memcpy", "_memset", "_memmove"],
	);

	tetsy_wasm::serialize_to_file(&args[2], module).expect("Module to serialize ok");
}
