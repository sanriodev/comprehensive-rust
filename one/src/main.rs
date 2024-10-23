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
}
//functions = fn keyword, params, return type = "-> type"
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b* c + c*a;
}
