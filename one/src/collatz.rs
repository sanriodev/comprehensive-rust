pub fn calculate_collatz(n: u32) -> u32 {
    let mut count = 0;
    let mut num = n;
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = 3 * num + 1;
        }
        count += 1;
    }
    count
}

#[test]
fn test_calculate_collatz() {
    assert_eq!(calculate_collatz(10), 6);
    assert_eq!(calculate_collatz(1), 0);
    assert_eq!(calculate_collatz(2), 1);
    assert_eq!(calculate_collatz(3), 7);
    assert_eq!(calculate_collatz(4), 2);
}