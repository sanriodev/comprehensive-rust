use control_flow::{divide_in_loop, is_even, is_even_ternary, print_in_for_loop};
use fibonacci::calculate;

mod fibonacci;

mod control_flow;

fn main() {
    println!("Hello, world!");
    /*
    signed interger: iN -> i32, i64 etc.
    unsigned interger: uN -> u32, u64 etc.
    string: char -> 32 bits wide
    booleans: bool -> 8 bits
    float: fN -> f32, f64
     */

    let num: i32 = 10; // i32  == int32 
    println!("immutable (const) variable declared with let: {num}");

    let mut num: i32 = 10; // mutable variable declared with let mut
    println!("mutable variable declared with let mut: {num}");
    num += 10;
    println!("mutable variable incremented by 10: {num}");

    let result: i32 = interproduct(10, 12, 14);
    println!("result of interproduct function: {result}");

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
}
//functions = fn keyword, params, return type = "-> type"
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b* c + c*a;
}
//functions also snake_case
fn only_take_u32(param: u32) {
    println!("onlyTake_u32 function: {param}");
}


