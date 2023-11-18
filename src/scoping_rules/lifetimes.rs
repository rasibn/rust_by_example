mod bounds;
mod explicit_annotation;
mod functions;
mod methods;
mod structs;
mod traits;
// A lifetime is a construct of the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid
// A borrow has a lifetime that is determined by where it is declared. As a result, the borrow is
// valid as long as it ends before the lender is destroyed.
pub(crate) fn main() {
    let i = 3; // lifetime for `i` starts.

    {
        let borrow1 = &i; // lifetime for `borrow1` starts.

        println!("borrow1: {}", borrow1);
    } // `borrow1` ends.

    {
        let borrow2 = &i; // lifetime for `borrow2` starts.

        println!("borrow2: {}", borrow2);
    } // `borrow2` ends.

    explicit_annotation::main();

    functions::main();
    methods::main();
    structs::main();
    traits::main();
    bounds::main();
}
// Lifetime ends.
