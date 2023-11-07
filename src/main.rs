mod ass_functions;
mod attributes2;
mod closures;
mod enums;
mod generics;
mod let_else;
mod structs;
mod while_let;

fn main() {
    enums::main();
    structs::main();
    let_else::main();
    while_let::main();
    ass_functions::main();
    closures::main();
    attributes2::main();
    generics::main();
}
