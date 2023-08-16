use std::path::Path;

use anyhow::Result;
use wasmtime::*;

static IC0: &str = r#"(module
    (func (export "msg_reply"))
    (func (export "debug_print") (param i32  i32))
    (func (export "trap") (param i32  i32))
    (func (export "msg_caller_size") (result i32) i32.const 0)
    (func (export "msg_caller_copy") (param i32 i32 i32))
    (func (export "msg_reply_data_append") (param i32 i32))
    (func (export "msg_arg_data_size") (result i32) i32.const 0)
    (func (export "msg_arg_data_copy") (param i32 i32 i32))
)"#;

fn generate_candid<P>(wasm_path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let mut store: Store<()> = Store::<()>::default();

    let mut linker = Linker::new(store.engine());
    let ic0_module = Module::new(store.engine(), IC0)?;
    let ic0 = linker.instantiate(&mut store, &ic0_module)?;
    linker.instance(&mut store, "ic0", ic0)?;

    let module = Module::from_file(store.engine(), wasm_path)?;
    let canister = linker.instantiate(&mut store, &module)?;

    let get_candid_pointer =
        canister.get_typed_func::<(), i32>(&mut store, "get_candid_pointer")?;
    let candid_pointer = get_candid_pointer.call(&mut store, ())?;

    let memory = canister
        .get_memory(&mut store, "memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;
    let memory_buffer = memory.data(&store);

    let mut i = candid_pointer as usize;
    let mut str_vec = vec![];
    while memory_buffer[i] != 0 {
        str_vec.push(memory_buffer[i]);
        i += 1;
    }
    let s = String::from_utf8(str_vec)?;
    Ok(s)
}

fn main() -> Result<()> {
    let c = generate_candid("target/wasm32-unknown-unknown/release/canister.wasm")?;
    println!("{c}");
    Ok(())
}
