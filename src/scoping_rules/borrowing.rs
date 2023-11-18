mod aliasing;
mod mutability;
mod ref_pattern;
// Most of the time we'd like to access data without taking ownership over it.
// To accomplish this, Rust uses a borrowiing mechanism. Instead of passing object
// by value ( T ) , object can passed by ( &T ).
pub(crate) fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error! Can't destroy `boxed_i32` while the inner value is borrowed.
        // eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);

    aliasing::main();
    ref_pattern::main();
}

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
    mutability::main();
}
