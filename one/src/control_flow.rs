pub fn is_even(x: i128) -> bool {
if x%2 == 0 {
    println!("the number is even");
    return true;
} else {
    println!("the number is odd");
    return false;
}
}

pub fn is_even_ternary(size: i32) {
    println!("you can use if statements much like ternaries in other languages");
    let is_big: &str = if size < 20 {"small"} else {"large"};
    println!("The size is: {}", is_big);
}

pub fn divide_in_loop(num: i32) -> i32{
    let mut number = num;
    while number > 10 {
        number = number / 2;
    }
    return number;
}

pub fn print_in_for_loop(num: i32) {
    for x in 0..num {
        println!("i love alina {x} times");
    }
}