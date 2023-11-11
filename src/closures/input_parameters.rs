fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn main() -> i32 {
    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    // Capture 2 vriables: `greeting` by reference and `farewell` by value
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // farewell.push_str("!!!");
        println!("Now I can sleep. zzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
    println!("{}", greeting);

    return 0;
}
