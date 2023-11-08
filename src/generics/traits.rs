struct Empty;
struct Null;

// A trait generic over `T`
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
// implementation for my
impl<U, T> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

pub(crate) fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}
