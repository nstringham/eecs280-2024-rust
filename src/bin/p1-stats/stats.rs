/// returns the count (sample size) of the numbers in v
pub fn count(v: &[f64]) -> usize {
    todo!()
}

/// returns the sum of the numbers in v
pub fn sum(v: &[f64]) -> f64 {
    todo!()
}

/// returns the arithmetic mean of the numbers in v
///  http://en.wikipedia.org/wiki/Arithmetic_mean
pub fn mean(v: &[f64]) -> f64 {
    todo!()
}

/// returns the median of the numbers in v
///  https://en.wikipedia.org/wiki/Median#Finite_data_set_of_numbers
pub fn median(v: &[f64]) -> f64 {
    todo!()
}

/// returns the min number in v
pub fn min(v: &[f64]) -> f64 {
    todo!()
}

/// returns the max number in v
pub fn max(v: &[f64]) -> f64 {
    todo!()
}

/// returns the corrected sample standard deviation of the numbers in v
///  http://en.wikipedia.org/wiki/Standard_deviation#Corrected_sample_standard_deviation
pub fn stdev(v: &[f64]) -> f64 {
    todo!()
}

/// returns the percentile p of the numbers in v like Microsoft Excel.
/// Refer to the project spec for the formula to use.
/// NOTE: the definition in the spec uses indexing from 1.  You will need to
/// adapt it to use indexing from 0.
pub fn percentile(v: &[f64], p: f64) -> f64 {
    todo!()
}

/// returns a new, filtered version of v containing the elements (and
/// only those elements) at v[x] where filter[x] is equal to target,
/// in the same order as they originally appear in v.
/// Note: For this function, compare values to the criteria using ==.
pub fn filter(v: &[f64], criteria: &[f64], target: f64) -> Vec<f64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.00001;

    fn almost_equal(x: f64, y: f64) -> bool {
        return (x - y).abs() < EPSILON;
    }

    #[test]
    fn public_tests() {
        let data = &[1.0, 2.0, 3.0];

        assert_eq!(count(data), 3);
        assert_eq!(sum(data), 6.0);
        assert!(almost_equal(mean(data), 2.0));
        assert_eq!(median(data), 2.0);
        assert_eq!(min(data), 1.0);
        assert_eq!(max(data), 3.0);
        assert!(almost_equal(stdev(data), 1.0));
        assert!(almost_equal(percentile(data, 0.5), 2.0));

        let criteria = &[1.0, 0.0, 1.0];
        assert_eq!(filter(data, criteria, 1.0), &[1.0, 3.0]);
    }

    #[test]
    fn test_sum_small_data_set() {
        let data = &[1.0, 2.0, 3.0];

        assert_eq!(sum(data), 6.0);
    }
}
