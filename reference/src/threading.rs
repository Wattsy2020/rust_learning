use std::thread;

const NUM_CORES: u8 = 8;

/// Calculate the factorial of a number, splitting the calculations across multiple threads
pub fn parallel_factorial(n: u128) -> u128 {
    let nums_per_thread = n / NUM_CORES as u128; // note integer division

    // create the threads, collect them all into a vector so all the threads are spawned and running
    let product_calculation_threads: Vec<_> = (0..NUM_CORES)
        .map(|thread_num| thread::spawn(move || calc_product(thread_num as u128, nums_per_thread)))
        .collect();

    // join the threads and accumulate the results
    let thread_product: u128 = product_calculation_threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .product();

    // multiply by any number cut off at the end (because of the integer division by NUM_CORES)
    let final_parts: u128 = ((nums_per_thread * (NUM_CORES as u128)+1)..=n).product();
    thread_product * final_parts
}

fn calc_product(offset: u128, num_to_multiply: u128) -> u128 {
    let start = offset * num_to_multiply + 1; // add one to avoid multiplying by zero when offset = 0
    let end = (offset + 1) * num_to_multiply;
    (start..=end).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fac_zero() {
        assert_eq!(parallel_factorial(0), 1)
    }

    #[test]
    fn test_fac_one() {
        assert_eq!(parallel_factorial(1), 1);
    }

    #[test]
    fn test_fac_small() {
        assert_eq!(parallel_factorial(2), 2);
        assert_eq!(parallel_factorial(3), 6);
        assert_eq!(parallel_factorial(4), 24);
        assert_eq!(parallel_factorial(5), 120);
    }
    
    fn fac(n: u128) -> u128 {
        (1..=n).product()
    }
    
    #[test]
    fn test_fac_large() {
        for i in 6..35 {
            println!("calculating fac of {i}");
            assert_eq!(parallel_factorial(i), fac(i))
        }
    }
}
