mod borrowing;
mod ownership_and_moves;
mod raii;

pub(crate) fn main() {
    raii::main();
    ownership_and_moves::main();
    borrowing::main();
}
