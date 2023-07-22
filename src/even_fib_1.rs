fn sum_even_fibonacci_numbers(max: u32) -> u32 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;
    while b < max {
        if b % 2 == 0 {
            sum += b;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    sum
}

pub fn test_sum_even_fibonacci_numbers() {
    assert_eq!(sum_even_fibonacci_numbers(100), 44);
    assert_eq!(sum_even_fibonacci_numbers(4_000_000), 4613732);
}