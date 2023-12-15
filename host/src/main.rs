use wasmtime::*;
// use wasmtime_wasi::preview2::WasiCtxBuilder;
use wasmtime_wasi::sync::WasiCtxBuilder;

// Partially based on https://guptanikhil.medium.com/rust-wasm-embed-wasmtime-in-your-rust-app-51c4da4231f6
fn main() -> wasmtime::Result<()> {
    let target = std::env::args()
        .nth(1)
        .expect("expected a path to the .wasm or .wat file");

    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi);
    // TODO: figure out how to make it work with wasmtime_wasi::preview2::WasiCtxBuilder
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let module = Module::from_file(&engine, &target)?;

    println!("Imports:");
    for (i, import) in module.imports().enumerate() {
        println!(
            "{i}. {}::{}: {:?}",
            import.module(),
            import.name(),
            import.ty()
        );
    }
    println!();

    println!("Exports:");
    for (i, export) in module.exports().enumerate() {
        println!("{i}. {:?}: {:?}", export.name(), export.ty());
    }
    println!();

    println!("Execution:");
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .unwrap();
    // let main_void = instance
    //     .get_typed_func::<(), i32>(&mut store, "__main_void")
    //     .unwrap();
    start.call(&mut store, ()).unwrap();
    // let ret = main_void.call(&mut store, ()).unwrap();
    // println!("ret: {:?}", ret);
    Ok(())
}
