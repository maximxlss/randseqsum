//! # Randseqsum
//!
//! `randseqsum` is a simple function that generates a random integer sequence that sums up to some value.

use rand::{thread_rng, Rng, seq::SliceRandom};

/// Generates a random sequence of length nums that sums up to total
///
/// # Examples
///
/// ```
/// let result = randseqsum(10, 100);
/// 
/// assert_eq!(result.iter().sum::<i32>(), 100);
/// ```
pub fn randseqsum(nums: usize, total: i32) -> Vec<i32> {
    let mut rng = thread_rng();

    let mut result = Vec::with_capacity(nums);
    for _ in 0..nums {
        let i: f32 = rng.gen();
        result.push(i)
    }
    let result_sum: f32 = result.iter().sum();
    let mut result: Vec<i32> = result.iter().map(|x| (x/result_sum*total as f32).round() as i32).collect();
    let result_sum: i32 = result.iter().sum();
    let mut error: i32 = total-result_sum;
    let step = if error > 0 {1} else {-1};
    while error != 0 {
        let chosen = result.choose_mut(&mut rng).unwrap();
        if *chosen + step > 0 {
            *chosen += step;
            error -= step;
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn very_small() {
        let result = randseqsum(1, 1);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn small() {
        let result = randseqsum(2, 1);

        assert_eq!(result.iter().sum::<i32>(), 1);
    }

    #[test]
    fn normal() {
        let result = randseqsum(10, 100);

        assert_eq!(result.iter().sum::<i32>(), 100);
    }

    #[test]
    fn big() {
        let result = randseqsum(100, 1000);

        assert_eq!(result.iter().sum::<i32>(), 1000);
    }

    #[test]
    fn very_big() {
        let result = randseqsum(1000, 100000);

        assert_eq!(result.iter().sum::<i32>(), 100000);
    }
}
