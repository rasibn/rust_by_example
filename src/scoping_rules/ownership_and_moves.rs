mod mutability;
mod partial_moves;

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

pub(crate) fn main() {
    let x = 5u32;

    // copy `x` into `y` - no resources are moved!
    let y = x;

    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;

    // The pointer of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // println!("a contains: {}", a);

    destroy_box(b);

    // memory has been fred by the function `destroy_box`, println! would result is deferencing freed memory
    // which is forbidden by the compiler
    // println!("b contains: {}", b);
    mutability::main();
    partial_moves::main();
}
