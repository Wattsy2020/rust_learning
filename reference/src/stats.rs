use std::collections::HashMap;
use std::ops::Div;
use crate::stats::IntMedian::{SingleNumber, TwoNumberMean};

#[allow(dead_code)]
#[derive(Debug)]
pub enum IntMedian {
    SingleNumber(i32),
    TwoNumberMean(f64)
}

/// Return the mean of a vector
///
/// Returns None if the vector is empty
pub fn mean(nums: &[i32]) -> Option<f64> {
    nums
        .iter()
        .copied()
        .reduce(|acc, next| acc + next) // take sum
        .map(|sum| f64::from(sum).div(nums.len() as f64)) // Option.map handles the length == 0 case
}

/// Return the median of a vector
///
/// Returns None if the vector is empty
pub fn median(nums: &[i32]) -> Option<IntMedian> {
    let length = nums.len();
    // note if the length is 0, subtracting 1 causes a panic, so we need to handle it with a checked sub
    let middle = length.div_ceil(2).checked_sub(1)?; // e.g. middle of 5 = index 2 (3rd item)
    if length % 2 == 0 {
        Some(TwoNumberMean(f64::from(nums[middle] + nums[middle + 1]) / 2f64))
    } else {
        Some(SingleNumber(nums[middle]))
    }
}

/// Return the mode of a vector
///
/// Returns None if the vector is empty
pub fn mode(nums: &[i32]) -> Option<i32> {
    let mut num_counts: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let count = num_counts.entry(*num).or_insert(0);
        *count += 1;
    }
    num_counts
        .iter()
        .max_by(|(_, left_count), (_, right_count)| left_count.cmp(right_count))
        .map(|(key, _)| *key)
}