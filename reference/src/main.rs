fn fib(n: i32) -> Option<i32> {
    if n < 0 { return None };
    if n < 2 { return Some(1) };

    let mut prev = 1;
    let mut current = 1;
    for _ in 0..=(n-2) {
        let next = prev + current;
        prev = current;
        current = next;
    }
    Some(current)
}

fn main() {
    // Can check the array access
    // Also arrays have a size known at compile time
    let array = [1, 2, 3, 4, 5];
    match array.get(10) {
        None => println!("Couldn't find index 10"),
        Some(value) => println!("Found {}", value),
    };

    // there is a ternary
    let _ = if array.len() > 5 { 2 } else { 1 };

    // can return a value from a loop using `break`
    let mut i = 2;
    let mut prev = 1;
    let mut current = 1;
    const N: i32 = 6;
    let fib_result = loop {
        if i == N { break current }
        let next = current + prev;
        prev = current;
        current = next;
        i = i + 1;
    };
    println!("fib({N}): {fib_result}");

    // for in loop of course
    for element in array {
        println!("{element}");
    }

    for range_element in (1..=4).rev() {
        print!("{range_element} ");
    }
    println!();

    let fibs = [fib(0), fib(1), fib(2), fib(3), fib(4), fib(5)];
    for result in fibs {
        println!("{}", result.unwrap());
    }
    assert_eq!(fib(-1), None);
}
