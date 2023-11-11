use my_mod::nested::public_function_in_nested_;
mod super_and_self;

mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n>");
        private_function();
    }

    pub mod nested {
        pub(in crate::modules::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()` that\n>");
            public_function_in_nested();
        }

        pub fn public_function_in_nested_() {
            println!("called `my_mod::nested::public_function_in_my_nested()`");
        }

        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }
}

pub(crate) fn main() {
    super_and_self::main();
}
