//! Algorithm implementation for super fair division.

use crate::Result;

/// Calculates a super fair division for participants with equal weights.
///
/// # Algorithm
///
/// 1. 每个参与者的公平收益初始值为 v/n，其中v是该参与者的报价，n是参与者总数
/// 2. 计算超公平数值 delta = (n*maxV-sumV)/(n*n)，其中maxV是最高报价，sumV是所有报价之和
/// 3. 每个参与者的收益值更新为 v/n + delta
/// 4. 最后，修正出价最高的用户的收益为其他所有人收益之和的负值
///
/// # Arguments
///
/// * `values` - A slice of i128 values representing the input data (bids) for each participant
///
/// # Returns
///
/// * `Result<Vec<i128>>` - The calculated fair division allocation for each participant or an error
///
/// # Examples
///
/// ```
/// use super_fair_division::calculate_fair_division_equal_weights;
///
/// let input = vec![10, 20, 30];
/// let allocation = calculate_fair_division_equal_weights(&input).unwrap();
/// println!("Fair division allocation: {:?}", allocation);
/// ```
pub fn calculate_fair_division_equal_weights(values: &[i128]) -> Result<Vec<i128>> {
    if values.is_empty() {
        return Err(crate::Error::InvalidInput);
    }

    // 检查参与者人数是否至少为2
    if values.len() < 2 {
        return Err(crate::Error::NotEnoughParticipants);
    }

    // 实现超公平分配算法 - 等权重版本
    let n = values.len() as i128;

    let sum_v: i128 = values.iter().sum();

    // 找出最高报价及其索引
    let mut max_v = values[0];
    let mut max_index = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > max_v {
            max_v = v;
            max_index = i;
        }
    }

    // 计算超公平数值 delta = (n*maxV-sumV) / (n*n)
    let delta = (n * max_v - sum_v) / (n * n);

    // 为每个参与者计算收益
    let mut allocation = Vec::with_capacity(values.len());
    let mut sum_others = 0;

    for (i, &v) in values.iter().enumerate() {
        if i != max_index {
            // 对于非最高报价者，收益为 v/n + delta
            let share = v / n + delta;
            allocation.push(share);
            sum_others += share;
        } else {
            // 为最高报价者预留一个位置，稍后填充
            allocation.push(0);
        }
    }

    // 修正最高报价者的收益为其他所有人收益之和的负值
    allocation[max_index] = -sum_others;

    Ok(allocation)
}

/// Calculates a super fair division for participants with different weights.
///
/// # Algorithm
///
/// 1. 将权重比例转成正整数比例
/// 2. 按照权重比例计算总人数 n
/// 3. 计算超公平数值 delta = (n*maxV_weighted-sumV)/(n*n)，其中maxV_weighted是最高报价乘以其权重
/// 4. 每个人的收益为 (v/n+delta)*weight
/// 5. 最后，修正出价最高的用户的收益为其他所有人收益之和的负值除以该用户的权重
///
/// # Arguments
///
/// * `values` - A slice of i128 values representing the input data for each participant
/// * `weights` - A slice of i128 values representing the weight of each participant
///
/// # Returns
///
/// * `Result<Vec<i128>>` - The calculated fair division allocation for each participant or an error
///
/// # Examples
///
/// ```
/// use super_fair_division::calculate_fair_division_weighted;
///
/// let input = vec![10, 20, 30];
/// let weights = vec![1, 2, 3]; // 权重比例为 1:2:3
/// let allocation = calculate_fair_division_weighted(&input, &weights).unwrap();
/// println!("Weighted fair division allocation: {:?}", allocation);
/// ```
pub fn calculate_fair_division_weighted(values: &[i128], weights: &[i128]) -> Result<Vec<i128>> {
    if values.is_empty() || weights.is_empty() || values.len() != weights.len() {
        return Err(crate::Error::InvalidInput);
    }

    // 检查参与者人数是否至少为2
    if values.len() < 2 {
        return Err(crate::Error::NotEnoughParticipants);
    }

    // 检查权重是否都为正数

    for &weight in weights {
        if weight <= 0 {
            return Err(crate::Error::InvalidInput);
        }
    }

    // 实现超公平分配算法 - 带权重版本

    // 1. 按照权重比例计算总人数 n
    let total_weight: i128 = weights.iter().sum();
    let n = total_weight; // 总人数就是权重之和

    // 2. 计算加权后的总值和找出最高报价
    // 总值应该是每个值乘以其权重再累加
    let sum_v: i128 = values.iter().zip(weights.iter())
        .map(|(&v, &w)| v * w)
        .sum();

    // 找出最高报价及其索引
    let mut max_v = values[0];
    let mut max_index = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > max_v {
            max_v = v;
            max_index = i;
        }
    } 

    // 3. 计算超公平数值 delta = (n*maxV_weighted-sumV)/(n*n)
    let delta = (n * max_v - sum_v) / (n * n);

    // 4. 为每个参与者计算收益
    let mut allocation = Vec::with_capacity(values.len());
    let mut sum_others = 0;

    for (i, (&v, &weight)) in values.iter().zip(weights.iter()).enumerate() {
        if i != max_index {
            // 对于非最高报价者，收益为 (v/n+delta)*weight
            let share = (v / n + delta) * weight;
            allocation.push(share);
            sum_others += share;
        } else {
            // 为最高报价者预留一个位置，稍后填充
            allocation.push(0);
        }
    }

    // 5. 修正出价最高的用户的收益为其他所有人收益之和的负值除以该用户的权重
    allocation[max_index] = -sum_others;

    Ok(allocation)
}
