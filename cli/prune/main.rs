extern crate tetsy_wasm;
extern crate twasm_utils as utils;
extern crate twasm_utils_cli as logger;
extern crate clap;

use clap::{App, Arg};

fn main() {
    logger::init_log();

	let target_runtime = utils::TargetRuntime::pwasm();

    let matches = App::new("wasm-prune")
                        .arg(Arg::with_name("input")
                            .index(1)
                            .required(true)
                            .help("Input WASM file"))
                        .arg(Arg::with_name("output")
                            .index(2)
                            .required(true)
                            .help("Output WASM file"))
                        .arg(Arg::with_name("exports")
                            .long("exports")
                            .short("e")
                            .takes_value(true)
                            .value_name("functions")
                            .help(&format!("Comma-separated list of exported functions to keep. Default: '{}'", target_runtime.call_symbol)))
                        .get_matches();

    let exports = matches
                    .value_of("exports")
                    .unwrap_or(target_runtime.call_symbol)
                    .split(',')
                    .collect();

    let input = matches.value_of("input").expect("is required; qed");
    let output = matches.value_of("output").expect("is required; qed");

    let mut module = tetsy_wasm::deserialize_file(&input).unwrap();

    // Invoke optimizer
    //   Contract is supposed to have only these functions as public api
    //   All other symbols not usable by this list is optimized away
    utils::optimize(&mut module, exports).expect("Optimizer failed");

    tetsy_wasm::serialize_to_file(&output, module).expect("Serialization failed");
}
