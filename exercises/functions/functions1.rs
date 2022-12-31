// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let tup:(u8,i8,f64) = (1, 2, 3.0);
    let mut test: String = String::from("test");

    struct MutableNumber {
        counter: u32
    }

    let mut c = 0;

    c = call_me(&mut test, c);
    println!("The value of test is {} and the value of c is: {}", test, c);
    c = call_me(&mut test, c);
    println!("The value of test is {} and the value of c is: {}", test, c);
}

fn call_me(test: &mut String, mut c: u32) -> u32 {
    c += 1;
    println!("{}", c);
    let temp = 
    test.push_str(&format!(" addition {}", c));
    c
    // return 100;
}