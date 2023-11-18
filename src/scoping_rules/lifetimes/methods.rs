struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

pub(crate) fn main() {
    let mut owner = Owner(18);

    // Owner::add_one(&mut owner);
    // Owner::print(&owner);

    owner.add_one();
    owner.print();
}
