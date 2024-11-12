use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Div;
use crate::stats::IntMedian::{SingleNumber, TwoNumberMean};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
pub fn mode<T: Eq + Hash>(nums: &[T]) -> Option<&T> {
    let num_counts = count_values(nums);
    num_counts
        .iter()
        .max_by(|(_, left_count), (_, right_count)| left_count.cmp(right_count))
        .map(|(key, _)| *key)
}

fn count_values<T: Eq + Hash>(values: &[T]) -> HashMap<&T, i32> {
    let mut counts = HashMap::new();
    for value in values {
        let count = counts.entry(value).or_insert(0);
        *count += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&vec![]), None);
        assert_eq!(mean(&vec![100]), Some(100f64));
        assert_eq!(mean(&vec![7, 5]), Some(6f64));
        assert_eq!(mean(&vec![5, 3, 9, 4, 10]), Some(6.2));
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&vec![]), None);
        assert_eq!(median(&vec![100]), Some(SingleNumber(100)));
        assert_eq!(median(&vec![7, 5]), Some(TwoNumberMean(6f64)));
        assert_eq!(median(&vec![5, 3, 9, 4, 10]), Some(SingleNumber(9)));
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode::<i32>(&vec![]), None);
        assert_eq!(mode(&vec![100]).copied(), Some(100));
        assert_eq!(mode(&vec![5, 3, 9, 4, 10, 4]).copied(), Some(4));
        assert_eq!(mode(&vec![5, 3, 5, 100, -4, 3, 3, -4]).copied(), Some(3));
    }
}