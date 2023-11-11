// Diverging function never return. They are marked using !, which is an empty.
#[allow(dead_code)]
fn foo() -> ! {
    panic!("This call never returns.");
}

#[allow(dead_code)]
fn some_fn() {
    ()
}

pub(crate) fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i % 2 == 1 {
                // The `i` variable is of type u32, which is perfectly fine variable is of type
                // u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return.
                // u23, but it is still fine, because it never returns and therefor
                // does not violate the type requirements of the match expression
                false => continue,
            };
            acc += addition;
        }
        acc
    }
}

// return type of loop {} is is also ! like network servers
// or functions that terminate the process (e.g. exit())
