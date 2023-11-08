// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

impl<A, D> MyTrait<A, D> for YourType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{
}

trait TraitB {}
trait TraitC {}
trait TraitF {}
trait TraitE {}

trait MyTrait<A, D> {}

struct YourType;

// When using a `where` clause is more expressive than using normal syntax. The `impl` in this
// example cannot directly expressed without a where caluse:

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // We want `Option<T>: Debug` as our bound because that is what's being printed. Doing
    // otherwise woudl be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self))
    }
}

pub(crate) fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
