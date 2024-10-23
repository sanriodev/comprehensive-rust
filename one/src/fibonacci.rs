pub fn calculate(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => calculate(n - 1) + calculate(n - 2),
    }
}


