mod Capturing;

pub(crate) fn main() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inffered = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_iffered: {}", closure_inffered(1));

    let one = || 1;
    println!("closure returning a one: {}", one());
}
