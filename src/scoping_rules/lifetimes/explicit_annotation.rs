// We need generic types for closures because for every closure defined the compiler makes a new
// type that implicitly stores the values captured by the closures
// so we need a generic `Fn`, `FnOnce` or `FnMut` trait bound to use it bound to pass a closure to
//  to pass a closure to a function.

pub(crate) fn main() {
    // Usually lifetimesa are elided
    // You need to explicitly annotate the lifetimes using generics (similar to closures)

    // foo<'a>
    // -> `foo` has a lifetime parameter `'a` which means that lifetime of `foo` may not
    // exceed that of `a
    // -> Explicit annotation of a type has the form `&'a T` where `'a` has already been
    // introduced.

    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn _print_refs(x: i32, y: i32) {
    println!("x is {} and y is {}", x, y);
}
// A function that takes no argument, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;
    //let y: &'a i32 = &_x;

    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime cannot be coerced into a longer one
}
