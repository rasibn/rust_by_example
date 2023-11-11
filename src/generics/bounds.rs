use std::fmt::Display;
mod multiple_bounds;
mod where_clause;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct S<T: Display>(T);

use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
//  the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

pub(crate) fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
    // --- Testcase: Empty Bounds ---

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));

    multiple_bounds::main();
    where_clause::main();
}

// Empty Bounds -> A consequence of how bounds work is that if a `trait` doesn't include any
// functionality, you can still use it as a bound, ,`Eq` and `Copy` are examples of such `triat`
// from the `std` library.

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// these functions are only valid for types which implement these traits.
// The fact that traits are empty is irrelevent

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}
