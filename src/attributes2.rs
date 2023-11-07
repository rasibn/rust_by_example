#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are on linux");
}

// this function gets compiled only if the target os is not linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* on linux");
}

pub(crate) fn main() {
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes, you are surely on linux!");
    } else {
        println!("Yep, it's definately *not* linux!");
    }

    // conditional_function();
}
// run this function use
//  rustc --cfg some_condition src/attributes2.rs && ./attributes2
#[cfg(some_condition)]
fn conditional_function() {
    println!("conditions met!");
}
