//! Small example of how instantiate a wasm module that imports a function
//! showing you how you can fill in host functionality for a wasm module.
//!

use anyhow::Result;
use wasmtime::*;

struct MyState {
    name: String,
    count: usize,
}

fn main() -> Result<()> {
    // First the wasm module needs to be compiled. This is done with a global
    // "compilation environment" within an "Engine". Note that the engines can
    // be further configured through "Config" if desired instead of using the
    // default like this is here.
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "../modules/hello.wat")?;

    // After a module is compiled we created a "Store" which will contain
    // instantiated module and other items like host functions. A Store contains
    // an arbitrary piece of host information, and we use "MyState" here.
    println!("Initialing...");
    let mut store = Store::new(
        &engine,
        MyState {
            name: "hello, world!".to_owned(),
            count: 0,
        },
    );

    // Our wasm module we'll be instantiating requires one imported function.
    // the function takes no parameters and returns no results. We create a host
    // implementation of that function here, and the `caller` parameter here is
    // used to get access our original `MyState` value.
    println!("Creating callback...");
    let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
        println!("Calling back ...");
        println!("> {}", caller.data().name);
        caller.data_mut().count += 1;
    });

    // Once we've got that all set up we can then move to the instantiation
    // phase, pairing together a compiled module as well as a set of imports.
    // Note that this is where the wasm `start` function, if any, would run.
    println!("Instantiating module...");
    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    // Next we poke around a bit to extract the `run` function from the module.
    println!("Extracting export...");
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

    // And last but not least we can call it!
    println!("Calling export ...");
    run.call(&mut store, ())?;

    println!("Done");
    Ok(())
}
