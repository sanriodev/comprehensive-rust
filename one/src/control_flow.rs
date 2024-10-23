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