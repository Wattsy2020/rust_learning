use std::cmp::min;
use std::fmt::Debug;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;

fn factorial_sum_1(n: u128) -> u128 {
    (1..=n).fold(0, |acc, x| acc + x)
}

fn factorial_sum_2(n: u128) -> u128 {
    (1..=n).sum()
}

fn factorial_sum_3(n: u128) -> u128 {
    let nums: Vec<u128> = (1..=n).collect();
    nums.iter().fold(0, |acc, x| acc + x)
}

fn factorial_sum_4(n: u128) -> u128 {
    let nums: Vec<u128> = (1..=n).collect();
    nums.iter().sum()
}

fn factorial_sum_5(n: u128) -> u128 {
    let mut result = 0;
    for i in 1..=n {
        result += i;
    }
    result
}

fn range_sum(from: u128, to: u128) -> u128 {
    let mut result = 0;
    for i in from..=to {
        result += i;
    }
    result
}

fn factorial_sum_threaded(n: u128) -> u128 {
    let mut result = 0;
    // number to calculate the next range from
    let mut start_range = 1;

    let (tx, rx) = mpsc::channel::<u128>();

    // start new threads to calculate results
    while start_range <= n {
        let tx_clone = tx.clone();
        let next_start_range = min(n, start_range + 1_000_000);
        thread::spawn(move || {
            let result = range_sum(start_range, next_start_range);
            _ = tx_clone.send(result)
        });

        start_range = next_start_range + 1;
    }

    // drop this transmitter, so that once the clones are dropped then the channel closes,
    // and the following loop terminates after reading all messages
    drop(tx);

    // receive results
    while let Ok(thread_result) = rx.recv() {
        result += thread_result;
    }

    result
}

fn time_function<T: Debug>(f: impl FnOnce() -> T) {
    let start_time = SystemTime::now();
    let result = f();
    let end_time = SystemTime::now();
    let time_taken = end_time
        .duration_since(start_time)
        .expect("end time should be after start time")
        .as_millis();
    println!("Function ran in {time_taken} milliseconds, producing result: {result:?}");
}

fn main() {
    let n: u128 = std::env::args().nth(1).unwrap().parse().unwrap();

    // no significant difference between these implementations in debug
    // However the iterator methods have a compiler optimisation that cause them to take 0 ms when using --release
    println!("Iterator fold");
    time_function(|| factorial_sum_1(n));

    println!("Iterator sum method");
    time_function(|| factorial_sum_2(n));

    println!("Vector fold");
    time_function(|| factorial_sum_3(n));

    // Vector sum is 2x faster than Vector fold in --release
    println!("Vector sum method");
    time_function(|| factorial_sum_4(n));

    // for loop is 2x faster than Vector fold in debug and release
    println!("For loop");
    time_function(|| factorial_sum_5(n));

    // Runs 10x faster than Vector fold in debug, 20x in release (with a larger input size)
    println!("Threaded");
    time_function(|| factorial_sum_threaded(n));
}
