fn fib(n: i32) -> Option<i32> {
    if n < 0 {
        return None;
    };
    if n < 2 {
        return Some(1);
    };

    let mut prev = 1;
    let mut current = 1;
    for _ in 0..=(n - 2) {
        let next = prev + current;
        prev = current;
        current = next;
    }
    Some(current)
}

fn print_optional(value: Option<i32>) -> () {
    match value {
        Some(val) => println!("{val}"),
        None => println!("None"),
    }
}

fn exclaim(str: &String) -> () {
    println!("{}!", str);
}

// taking &str as a parameter is better than taking &String
// since it is more permissive, it allows normal String and string slices
fn words(str: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut prev_word_start = 0;
    for (idx, char) in str.chars().enumerate() {
        if char == ' ' {
            result.push(&str[prev_word_start..idx]);
            prev_word_start = idx + 1;
        }
    }
    if prev_word_start < str.len() {
        result.push(&str[prev_word_start..]);
    }
    result
}

/// Return all strings that contain the substring
fn find_contains<'a, 'b>(haystack: &'a [String], substr: &'b str) -> Vec<&'a String> {
    // the collect uses type inference to know it must output a Vec
    // and Vec implements the FromIterator trait, so it has an implementation of collect
    haystack
        .iter()
        .filter(|full| full.contains(substr))
        .collect()
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
        if i == N {
            break current;
        }
        let next = current + prev;
        prev = current;
        current = next;
        i += 1;
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
        print_optional(result);
    }
    assert_eq!(fib(-1), None);

    // Borrowing, need to have the function take a reference to borrow the value
    let str = String::from("hello");
    exclaim(&str);
    exclaim(&str); // won't compile unless exclaim takes a reference

    // Rust implicitly dereferences things when operating on them
    let x: i32 = -1;
    let ref_x = &x;
    println!("{}", ref_x + x); // same result even without dereferencing
    println!("{}", *ref_x + x);
    let box_x = Box::new(ref_x);
    println!("{}", box_x.abs()); // multiple layers of dereferencing

    // vector operations
    let vec = vec!["hello", "there", "!", "general", "kenobi"];
    let max_string_length = vec.iter().map(|str| str.len()).max().unwrap();
    println!("{}", max_string_length);

    // string slicing
    let mut string = String::from("hello world");
    string.push('!');
    let first = &string[0..5];
    let second = &string[6..];
    println!("first: {} second: {}", first, second);

    println!("{:?}", words(&string));
    println!("{:?}", words(&string[..9])); // can pass in a slice

    let strs: Vec<String> = vec!["Hello there!", "General Kenobi", "Hello world!"]
        .iter()
        .map(|str| String::from(*str))
        .collect();
    println!("{:?}", find_contains(&strs, "Hello"));
}
