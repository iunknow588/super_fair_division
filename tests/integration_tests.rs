use super_fair_division::{calculate_fair_division_equal_weights, calculate_fair_division_weighted};

#[test]
fn test_equal_weights_basic() {
    let input = vec![10, 50];
    let result = calculate_fair_division_equal_weights(&input).unwrap();

    // 检查结果长度与输入相同
    assert_eq!(result.len(), input.len());

    // 检查总和为0（超公平分配的特性）
    assert_eq!(result.iter().sum::<i128>(), 0);

    // 找出最高报价及其索引
    let mut max_v = input[0];
    let mut max_index = 0;
    for (i, &v) in input.iter().enumerate() {
        if v > max_v {
            max_v = v;
            max_index = i;
        }
    }

    // 检查最高报价者的收益是否为负值
    assert!(result[max_index] < 0);

    // 检查最高报价者的收益是否等于其他所有人收益之和的负值
    let sum_others: i128 = result.iter().enumerate()
        .filter(|&(i, _)| i != max_index)
        .map(|(_, &v)| v)
        .sum();
    assert_eq!(result[max_index], -sum_others);


    // 检查 结果值 公平值是5,25;
    assert_eq!(result[0], 15);
    assert_eq!(result[1], -15);
}

#[test]
fn test_weighted_basic() {
    let input = vec![30, 60];
    let weights = vec![1, 2];
    let result = calculate_fair_division_weighted(&input, &weights).unwrap();

    // 检查结果长度与输入相同
    assert_eq!(result.len(), input.len());

    // 检查总和为0（超公平分配的特性）
    assert_eq!(result.iter().sum::<i128>(), 0);

    // 理论上的公平值 10; 40;
    // 超公平值delta = 
    assert_eq!(result[0], 13);
    assert_eq!(result[1], -13); 
}

#[test]
fn test_weighted_algorithm_correctness() {
    // 测试加权算法的正确性
    let input = vec![9000, 7000, 6000,5000,4000];
    let weights = vec![5,40,40,5,10];
    let result = calculate_fair_division_weighted(&input, &weights).unwrap();

    // 计算预期结果
    // 检查总和为0
    assert_eq!(result[0], -8415, "result[0] should be -8415"); 
    assert_eq!(result[1], 3880, "result[0] should be 3880"); 
    assert_eq!(result[2], 3480, "result[0] should be 3480"); 
    assert_eq!(result[3], 385, "result[0] should be 385"); 
    assert_eq!(result[4], 670, "result[0] should be 670");  
}

#[test]
fn test_empty_input() {
    let empty: Vec<i128> = vec![];
    assert!(calculate_fair_division_equal_weights(&empty).is_err());
    assert!(calculate_fair_division_weighted(&empty, &empty).is_err());
}

#[test]
fn test_not_enough_participants() {
    // 只有一个参与者的情况
    let single_input = vec![100];
    let single_weight = vec![1];

    // 等权重算法
    let result_equal = calculate_fair_division_equal_weights(&single_input);
    assert!(result_equal.is_err());
    match result_equal {
        Err(super_fair_division::Error::NotEnoughParticipants) => {}, // 期望的错误
        _ => panic!("Expected NotEnoughParticipants error for equal weights with single participant")
    }

    // 加权算法
    let result_weighted = calculate_fair_division_weighted(&single_input, &single_weight);
    assert!(result_weighted.is_err());
    match result_weighted {
        Err(super_fair_division::Error::NotEnoughParticipants) => {}, // 期望的错误
        _ => panic!("Expected NotEnoughParticipants error for weighted with single participant")
    }
}

#[test]
fn test_large_numbers() {
    // 使用较小的大数值来避免溢出
    let input = vec![i128::MAX / (1 << 40), i128::MAX / (1 << 41), i128::MAX / (1 << 42)];
    let weights = vec![1, 2, 3];

    let result_equal = calculate_fair_division_equal_weights(&input);
    assert!(result_equal.is_ok());

    let result_weighted = calculate_fair_division_weighted(&input, &weights);
    assert!(result_weighted.is_ok());
}

#[test]
fn test_negative_numbers() {
    let input = vec![-10, -20, -30];
    let weights = vec![1, 2, 3];

    let result_equal = calculate_fair_division_equal_weights(&input);
    assert!(result_equal.is_ok());

    let result_weighted = calculate_fair_division_weighted(&input, &weights);
    assert!(result_weighted.is_ok());
}

#[test]
fn test_equal_weights_algorithm_correctness() {
    // 测试算法的正确性
    let input = vec![10, 20, 30, 40];
    let result = calculate_fair_division_equal_weights(&input).unwrap();

    // 计算预期结果
    let n = input.len() as i128;
    let sum_v: i128 = input.iter().sum();
    let max_v = *input.iter().max().unwrap();
    let max_index = input.iter().position(|&v| v == max_v).unwrap();
    let delta = (n * max_v - sum_v) / (n * n);

    // 验证非最高报价者的收益
    for (i, &v) in input.iter().enumerate() {
        if i != max_index {
            let expected = v / n + delta;
            assert_eq!(result[i], expected, "Incorrect allocation for participant {}", i);
        }
    }

    // 验证最高报价者的收益
    let sum_others: i128 = result.iter().enumerate()
        .filter(|&(i, _)| i != max_index)
        .map(|(_, &v)| v)
        .sum();
    assert_eq!(result[max_index], -sum_others, "Incorrect allocation for highest bidder");

    // 验证总和为0
    assert_eq!(result.iter().sum::<i128>(), 0, "Sum of allocations should be 0");
}

#[test]
fn test_mixed_numbers() {
    let input = vec![-10, 0, 30];
    let weights = vec![1, 2, 3];

    let result_equal = calculate_fair_division_equal_weights(&input);
    assert!(result_equal.is_ok());

    let result_weighted = calculate_fair_division_weighted(&input, &weights);
    assert!(result_weighted.is_ok());
}

#[test]
fn test_invalid_weights() {
    let input = vec![10, 20, 30];

    // 权重和值的长度不匹配
    let short_weights = vec![1, 2];
    assert!(calculate_fair_division_weighted(&input, &short_weights).is_err());

    // 负权重
    let negative_weights = vec![1, -2, 3];
    assert!(calculate_fair_division_weighted(&input, &negative_weights).is_err());

    // 零权重
    let zero_weights = vec![1, 0, 3];
    assert!(calculate_fair_division_weighted(&input, &zero_weights).is_err());
}
