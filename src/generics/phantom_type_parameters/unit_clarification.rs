// Adding a type to a struct or enum without any cost
//
//
//
// Unit conversions can be examined by implementing `Add` with a phantom type parameter.
// The `Add trait` is examined below:

// pub trait Add<RHS = Self> {
//    type Output;
//    fn add(self, rhs: RHS) -> Self::Output;
// }

// `Output` must  be `T<U>` so that `T<U> + T<U> = T<U>`.

//  impl<U> Add for T<U> {
//      type Output = T<U>;
//  }
//  pub(crate) fn main() {}

use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

// `Length` is a type with phantom type paramter `Unit`
// // and is not generic over the length type (that is `f64`).
//
// `f64` already implements the `Clone` and `Copy` traits

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in ", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // let one_feter = one_foot + one_meter;
}
