mod driver;
mod jitter;

use inkwell::OptimizationLevel;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;

fn main() {
    // Create the top-level LLVM context
    let context = Context::create();

    // Create a module to hold functions/IR
    let module = context.create_module("nesia_module");

    // A builder helps generate instructions
    let builder = context.create_builder();

    // Get the i32 type (native pointer-sized int would be different)
    let i32_type = context.i32_type();

    // Define function type: fn() -> i32
    let fn_type = i32_type.fn_type(&[], false);

    // Add function to module
    let function = module.add_function("main", fn_type, None);

    // Create a basic block and position the builder
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    // Build a constant i32(42)
    let const_42 = i32_type.const_int(42, false);

    // Return const_42 from the function
    builder.build_return(Some(&const_42));

    // Optionally: print the generated IR for inspection
    module.print_to_stderr();

    // Create a JIT execution engine and run the function
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .expect("Failed to create JIT execution engine");

    // Get a typed JIT wrapper for the function: unsafe extern "C" fn() -> i32
    unsafe {
        // `JitFunction<F>` ensures the function pointer doesn't outlive the engine
        let jit_fn: JitFunction<unsafe extern "C" fn() -> i32> = execution_engine
            .get_function("main")
            .expect("Couldn't find function");

        // Call the function
        let result = jit_fn.call();

        println!("JIT returned: {}", result);
    }
}
