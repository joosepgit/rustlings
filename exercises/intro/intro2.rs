// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.
use std::fmt;

fn main() {
    let world: &str = "world";
    println!("Hello {}!", world);
    println!("I'm a Rustacean!");
    println!("{number:>12}", number=1);
    println!("{number:0<5}", number=1);

    struct Structure {
        longitude: i32,
    }

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})", self.longitude)
        }
    }

    // This will not compile because `Structure` does not implement
    // fmt::Display
    println!("This struct `{}` won't print...", Structure {longitude: 3 });
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}

//I AM DONE
