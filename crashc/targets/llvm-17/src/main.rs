use inkwell::context::Context;
use inkwell::types::BasicMetadataTypeEnum::IntType;

fn main() {
    let context = Context::create();

    let module = context.create_module("main_module");

    let builder = context.create_builder();

    let fn_type = context.i32_type().fn_type(&[IntType(context.i32_type()), IntType(context.i32_type())], false);

    let function = module.add_function("test", fn_type, None);

    let entry = context.append_basic_block(function, "entry");

    builder.position_at_end(entry);

    let x = function.get_first_param().unwrap().into_int_value();
    let y = function.get_nth_param(1).unwrap().into_int_value();

    let sum = builder.build_int_add(x, y, "sum");

    builder.build_return(Some(&sum.expect("Ey"))).expect("TODO: panic message");

    module.print_to_stderr();
}