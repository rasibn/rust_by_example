mod ass_functions;
mod attributes;
mod closures;
mod diverging_functions;
mod enums;
mod higher_order_function;
mod let_else;
mod modules;
mod structs;
mod while_let;

// A crate is a compilation unit in Rust
// rustc some_file.rs treats some_file.rs as a crate file
// The contents of a module would be inserted wherever mod declarations in the crate file are foun before running the compiler over it
// Crates can compile into a binary or into a library. Default is a binary.
// Cargo may run concurrently

fn main() {
    enums::main();
    structs::main();
    let_else::main();
    while_let::main();
    ass_functions::main();
    closures::main();
    higher_order_function::main();
    diverging_functions::main();
    modules::main();
    attributes::main();
}
