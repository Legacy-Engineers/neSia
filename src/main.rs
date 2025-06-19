use inkwell::context::Context;

fn main() {
    let context = Context::create();
    let module = context.create_module("nesia");
    let builder = context.create_builder();

    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);
    let return_val = i32_type.const_int(42, false);
    builder.build_return(Some(&return_val));

    module.print_to_stderr();
}
