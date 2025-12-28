pub(crate) mod limit_tunables;

use std::path::Path;
use wasmer::{imports, Instance, Engine, Module, Store, TypedFunction, Pages, NativeEngineExt};

#[cfg(feature = "cranelift")]
use wasmer_compiler_cranelift::Cranelift;

#[cfg(feature = "llvm")]
use wasmer_compiler_llvm::LLVM;

#[cfg(feature = "singlepass")]
use wasmer_compiler_singlepass::Singlepass;
use std::time::Instant;
use limit_tunables::LimitingTunables;

#[cfg(feature = "compile")]
use wasmer_compiler::ArtifactCreate;

fn main() -> anyhow::Result<()> {
    let start = Instant::now();

    let mut store: Store;
    let module: Module;

    #[cfg(not(feature = "precompiled"))]
    {
        let wasmfile = "../build/exports.wasm";
        println!("wasmfile: {}", wasmfile);

        let wasm_bytes = std::fs::read(wasmfile)?;

        let elapsed = start.elapsed();
        println!("Elapsed after read wasmfile: {:.2?}", elapsed);

        let mut compiler: wasmer::Engine;

        #[cfg(feature = "cranelift")]
        {
            compiler = Cranelift::default().into();
        }
        #[cfg(feature = "llvm")]
        {
            compiler = LLVM::default().into();
        }
        #[cfg(feature = "singlepass")]
        {
            compiler = Singlepass::default().into();
        }

        let elapsed = start.elapsed();
        println!("Elapsed after engine creation: {:.2?}", elapsed);

        let base = wasmer::BaseTunables {
            // Always use dynamic heap memory to save memory
            static_memory_bound: Pages(0),
            static_memory_offset_guard_size: 0,
            dynamic_memory_offset_guard_size: 8192 as _,
        };
        let tunables = LimitingTunables::new(base, Pages(8192));

        let elapsed = start.elapsed();
        println!("Elapsed after tunables creation: {:.2?}", elapsed);

        // compiler.set_tunables(tunables);
        store = Store::new(compiler);

        #[cfg(feature = "compile")]
        {
            let res = unsafe { store.engine().compile(&wasm_bytes) }.unwrap();
            &res.serialize_to_file(Path::new("./compiled.bin")).unwrap();
            println!("done compiling and saved to ./compiled.bin. BYE");
            return Ok(());
        }

        // let mut store = Store::new(compiler);

        let elapsed = start.elapsed();
        println!("Elapsed after store creation: {:.2?}", elapsed);

        //We then use our store and Wasm bytes to compile a `Module`.
        //A `Module` is a compiled WebAssembly module that isn't ready to execute yet.
        module = Module::new(&store, wasm_bytes)?;
    }

    #[cfg(feature = "precompiled")]
    {
        println!("Reading precompiled wasm file from ./compiled.bin");
        store = Store::default();
        module = unsafe { Module::deserialize_from_file(&store, Path::new("./compiled.bin")) }?;
    }

    let elapsed = start.elapsed();
    println!("Elapsed after module creation: {:.2?}", elapsed);

    // We then create an import object so that the `Module`'s imports can be satisfied.
    let import_object = imports! {};

    // We then use the `Module` and the import object to create an `Instance`.
    // An `Instance` is a compiled WebAssembly module that has been set up
    // and is ready to execute.
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let elapsed = start.elapsed();
    println!("Elapsed after instance creation: {:.2?}", elapsed);

    // We get the `TypedFunction` with no parameters and no results from the instance.
    // Recall that the Wasm module exported a function named "run", this is getting
    // that exported function from the `Instance`.
    let run_func: TypedFunction<(), i32> = instance.exports.get_typed_function(&mut store, "fhe_setup")?;

    let elapsed = start.elapsed();
    println!("Elapsed after function retrieval: {:.2?}", elapsed);

    let func_call_time = Instant::now();

    // Finally, we call our exported Wasm function and see the return value.
    let result = run_func.call(&mut store)?;

    let elapsed = func_call_time.elapsed();
    println!("Time that function ran: {:.2?}", elapsed);


    println!("got result from wasm: {}", result);
    Ok(())
}

#[test]
fn test_hello_world() -> anyhow::Result<()> {
    main()
}