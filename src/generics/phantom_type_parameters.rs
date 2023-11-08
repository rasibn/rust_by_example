use std::marker::PhantomData;
mod unit_clarification;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // Allow equality test for this type.
struct PhantomTuple<A, B>(A, PhantomData<B>);

// a phantom type struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)] // Allow equality test for this type.
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// Note: Storage is allocated for generic type `A`, but not for `B`. Therefore, `B` cannot be used
// in computations.

pub(crate) fn main() {
    // Here, `f32` and `f64` are hidden parameters.
    // Phantom type specified as `<char, f32>`
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let main = unit_clarification::main();

    // println!("_tuple1 == tuple2: {}", _tuple1 == _tuple2);
    // println!("_struct1 == struct2: {}", _struct1 == _struct2);
}
