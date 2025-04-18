use super_fair_division::{calculate_fair_division_equal_weights, calculate_fair_division_weighted};

fn main() {
    println!("Super Fair Division Library Demo\n");

    // 等权重分配示例
    let values = vec![100, 200, 300, 400, 500];
    println!("Input values: {:?}", values);

    // 计算超公平分配，使用 delta = (n*maxV-sumV)/(n*n) 公式
    match calculate_fair_division_equal_weights(&values) {
        Ok(allocation) => {
            println!("Equal weights allocation: {:?}", allocation);

            // 验证结果
            let sum: i128 = allocation.iter().sum();
            println!("Sum of allocations: {} (should be 0)\n", sum);

            // 找出最高报价及其索引
            let mut max_v = values[0];
            let mut max_index = 0;
            for (i, &v) in values.iter().enumerate() {
                if v > max_v {
                    max_v = v;
                    max_index = i;
                }
            }
            println!("Highest bidder (index {}): {}", max_index, values[max_index]);
            println!("Allocation for highest bidder: {}\n", allocation[max_index]);
        },
        Err(err) => eprintln!("Error calculating equal weights division: {:?}\n", err),
    }

    // 加权分配示例，使用权重比例计算
    let values = vec![100, 200, 300, 400, 500];
    let weights = vec![1, 2, 3, 4, 5]; // 权重比例为 1:2:3:4:5

    println!("Input values: {:?}", values);
    println!("Weights: {:?}", weights);

    match calculate_fair_division_weighted(&values, &weights) {
        Ok(allocation) => {
            println!("Weighted allocation: {:?}", allocation);

            // 验证结果
            let sum: i128 = allocation.iter().sum();
            println!("Sum of allocations: {} (should be 0)\n", sum);

            // 展示权重与分配的关系
            println!("Weight vs Allocation:");
            for i in 0..values.len() {
                println!("Weight: {}, Allocation: {}", weights[i], allocation[i]);
            }
            println!();
        },
        Err(err) => eprintln!("Error calculating weighted division: {:?}\n", err),
    }

    // 处理边界情况示例
    let edge_case = vec![i128::MAX / 2, i128::MIN / 2, i128::MAX / 4];
    let edge_weights = vec![1, 2, 3];

    println!("Edge case values: {:?}", edge_case);
    println!("Edge case weights: {:?}", edge_weights);

    match calculate_fair_division_equal_weights(&edge_case) {
        Ok(allocation) => println!("Edge case equal weights allocation: {:?}", allocation),
        Err(err) => eprintln!("Error with edge case equal weights: {:?}", err),
    }

    match calculate_fair_division_weighted(&edge_case, &edge_weights) {
        Ok(allocation) => println!("Edge case weighted allocation: {:?}", allocation),
        Err(err) => eprintln!("Error with edge case weighted: {:?}", err),
    }
}
