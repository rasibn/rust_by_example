// using a closure as a function parameter requires generics, which is necessary because of how
// they are defined:
// The variable which are captures are assigned these types
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply2<F>(f: F)
where
    F: Fn(),
{
    f();
}

pub fn main() {
    let x = 7;
    let print = || println!("{}", x);

    apply(print);
    apply2(print);
}
