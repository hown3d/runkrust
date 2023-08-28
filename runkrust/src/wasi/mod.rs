//! Running a WASI compiled WebAssembly module with Wasmer.
//!
//! This example illustrates how to run WASI modules with
//! Wasmer. To run WASI we have to have to do mainly 3 steps:
//!
//!   1. Create a `WasiEnv` instance
//!   2. Attach the imports from the `WasiEnv` to a new instance
//!   3. Run the `WASI` module.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example wasi --release --features "cranelift,wasi"
//! ```
//!
//! Ready?

use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use oci_spec::runtime::Spec;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;
use which::which_in;

pub fn run(spec: &Spec) -> Result<(), Box<dyn std::error::Error>> {
    let store = Store::default();
    let rootfs = spec.root().as_ref().unwrap().path();
    let args = spec.process().as_ref().unwrap().args().as_ref().unwrap();
    let env = get_env(spec);
    let path_env = env.get("PATH");
    let program_name = get_wasm_file(args.get(0).unwrap(), path_env, &rootfs)?;

    let module = Module::from_file(&store, rootfs.join(args.get(0).unwrap()))?;

    let mut wasi_env = WasiState::new(program_name.into_os_string().into_string().unwrap())
        .args(&args[1..])
        .envs(env)
        .finalize()?;
    // Generate an `ImportObject`.
    let import_object = wasi_env.import_object(&module)?;

    // Let's instantiate the module with the imports.
    let instance = Instance::new(&module, &import_object)?;
    instance.store().set_trap_handler(handler)

    // Let's call the `_start` function, which is our `main` function in Rust.
    let start = instance.exports.get_function("_start")?;
    start.call(&[])?;

    Ok(())
}

fn get_wasm_file(wasm_file: &str, path_env: Option<&String>, cwd: &PathBuf) -> Result<PathBuf> {
    let wasm_file_path = PathBuf::from(wasm_file);
    if wasm_file_path.has_root() && wasm_file_path.is_relative() {
        return Ok(wasm_file_path);
    }
    let wasm_file = which_in(wasm_file, path_env, cwd)?;
    Ok(wasm_file)
}

fn get_env(spec: &Spec) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let env = spec.process().as_ref().unwrap().env().as_ref().unwrap();
    for val in env {
        let split: Vec<&str> = val.split("=").collect();
        if split.len() != 2 {
            continue;
        };
        map.insert(String::from(split[0]), String::from(split[1]));
    }
    map
}

#[test]
#[cfg(feature = "wasi")]
fn test_wasi() -> Result<(), Box<dyn std::error::Error>> {
    main()
}
