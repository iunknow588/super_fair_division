//! # Super Fair Division
//!
//! `super_fair_division` is a library that provides algorithms for super fair division problems.
//! It allows users to calculate fair allocations based on input values with or without weights.
//!
//! ## Equal Weights
//!
//! ```
//! use super_fair_division::calculate_fair_division_equal_weights;
//!
//! let input = vec![10, 20, 30];
//! let allocation = calculate_fair_division_equal_weights(&input).unwrap();
//! println!("Fair division allocation: {:?}", allocation);
//! ```
//!
//! ## Weighted Division
//!
//! ```
//! use super_fair_division::calculate_fair_division_weighted;
//!
//! let input = vec![10, 20, 30];
//! let weights = vec![1, 2, 3]; // 权重比例为 1:2:3
//! let allocation = calculate_fair_division_weighted(&input, &weights).unwrap();
//! println!("Weighted fair division allocation: {:?}", allocation);
//! ```

mod algorithm;

pub use algorithm::{calculate_fair_division_equal_weights, calculate_fair_division_weighted};

/// The main error type for the super_fair_division library
#[derive(Debug)]
pub enum Error {
    /// Indicates that the input data is invalid
    InvalidInput,
    /// Indicates that the calculation failed
    CalculationFailed,
    /// Indicates that there are not enough participants (minimum 2)
    NotEnoughParticipants,
}

/// Result type for the super_fair_division library
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_weights() {
        let input = vec![10, 20, 30];
        let result = calculate_fair_division_equal_weights(&input);
        assert!(result.is_ok());

        let allocation = result.unwrap();
        assert_eq!(allocation.len(), input.len());
        // 超公平分配算法的结果总和应为0
        assert_eq!(allocation.iter().sum::<i128>(), 0);
    }

    #[test]
    fn test_weighted() {
        let input = vec![100, 200, 300];
        let weights = vec![1, 2, 3];
        let result = calculate_fair_division_weighted(&input, &weights);
        assert!(result.is_ok());

        let allocation = result.unwrap();
        assert_eq!(allocation.len(), input.len());
        // 超公平分配算法的结果总和应为0
        assert_eq!(allocation.iter().sum::<i128>(), 0);
    }

    #[test]
    fn test_invalid_input() {
        // 空输入
        let empty: Vec<i128> = vec![];
        assert!(calculate_fair_division_equal_weights(&empty).is_err());

        // 权重和值的长度不匹配
        let input = vec![10, 20, 30];
        let weights = vec![1, 2];
        assert!(calculate_fair_division_weighted(&input, &weights).is_err());

        // 负权重
        let input = vec![10, 20, 30];
        let weights = vec![1, -2, 3];
        assert!(calculate_fair_division_weighted(&input, &weights).is_err());

        // 参与者人数不足
        let single_input = vec![100];
        let single_weight = vec![1];
        assert!(calculate_fair_division_equal_weights(&single_input).is_err());
        assert!(calculate_fair_division_weighted(&single_input, &single_weight).is_err());
    }
}
