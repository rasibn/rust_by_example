mod associated_items;
mod bounds;
mod functions;
mod implementation;
mod new_type_idiom;
mod phantom_type_parameters;
mod traits;
// A contrete type `A`
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`.
// Therefore, `Single` is a concrete type, and `A` is defined above.
struct Single(A);

// Here `<T>` precedes the first use of `T`, so SingleGen is a generic type.
// Because the type parameter `T` is generic, it could be anything, including the contrete
// type A at the top
struct SingleGen<T>(T);

pub(crate) fn main() {
    // Single is concrete and explicity taken A
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _i32 = SingleGen(6);
    let _it = SingleGen(A);
    let _char = SingleGen('a');

    self::implementation::main();
    functions::main();
    traits::main();
    bounds::main();
    new_type_idiom::main();
    associated_items::main();
    phantom_type_parameters::main();
}
