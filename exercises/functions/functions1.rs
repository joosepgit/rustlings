// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let tup:(u8,i8,f64) = (1, 2, 3.0);
    let x = tup.2;
    let y = call_me();
    println!("The value of x is {}", x);
}

fn call_me() -> u32 {
    100
    // return 100;
}