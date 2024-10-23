use collatz::calculate_collatz;
use control_flow::{blocks, divide_in_loop, is_even, is_even_ternary, nested_loops, print_in_for_loop};
use datatypes::only_take_u32;
use fibonacci::calculate;
use macros::factorial;

mod fibonacci;

mod collatz;

mod control_flow;

mod macros;

mod datatypes;

fn main() {
    println!("Hello, world!");
    /*
    signed interger: iN -> i32, i64 etc.
    unsigned interger: uN -> u32, u64 etc.
    string: char -> 32 bits wide
    booleans: bool -> 8 bits
    float: fN -> f32, f64
     */
    nested_loops();

    let explicit_type_int: u32 = 245;
    let inferred_type_int = 10;
    only_take_u32(inferred_type_int);
    only_take_u32(explicit_type_int);
    let n = 6;
    println!("fibonacci number at position {n}: {}", calculate(n));

    is_even(9);

    is_even_ternary(10);

    let res = divide_in_loop(123);
    print!("result of divide_in_loop function: {res}");
    print_in_for_loop(10);

    blocks();

    factorial(5);


    println!("collatz sequence for 10: {}", calculate_collatz(10));
}



