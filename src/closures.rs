mod input_functions;
mod input_parameters;
mod iterator_any;
mod output_parameters;
mod searching_iterators;
mod type_anonymity;

pub(crate) fn main() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inffered = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_iffered: {}", closure_inffered(1));

    let one = || 1;
    println!("closure returning a one: {}", one());

    input_parameters::main();
    type_anonymity::main();
    input_functions::main();
    iterator_any::main();
    output_parameters::main();
    searching_iterators::main();
    searching_iterators::main2();
}
