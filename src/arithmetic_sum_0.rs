
fn arithmetic_sum(start: i32, end: i32, interval: i32) -> i32 {
    if start >= end {
        return 0;
    }
    let n = 1 + ((end-1) - start) / interval;
    let end = start + (n-1) * interval;
    (start + end) * n / 2
}


fn arithmetic_sum_test() {
    let upper = 1000;
    let three = arithmetic_sum(3, upper, 3);
    let five = arithmetic_sum(5, upper, 5);
    let fifteen = arithmetic_sum(15, upper, 15);
    println!("{}, {}, {}, {}", three + five - fifteen, three, five, fifteen)
}