#[allow(dead_code)]
pub(crate) fn main() {
    println!("Hi");
    let book = "The name of the book";
    load_to_lucy(&book.to_owned());

    my::indirect_call();
}

fn load_to_lucy(book: &String) {
    println!("The book loaned {}", book);
}

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `my::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::cool::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call)()`, that \n>");
        // the `self` keyword refers to the current module scope - in this case ``.
        // both are same!
        self::function();
        function();

        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        super::function();

        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.

        {
            use crate::modules::super_and_self::my::cool::function as root_function;
            root_function();
        }
    }
}
