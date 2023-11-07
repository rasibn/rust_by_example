// when attributes apply to  a whole crate, their sytnax is #![crate_attribute],
// when they apply to a module or tiem the syntax  is #[item_attribute]

pub(crate) fn main() {
    println!("Hi");
}
