# Super Fair Division

[![Crates.io](https://img.shields.io/crates/v/super_fair_division.svg)](https://crates.io/crates/super_fair_division)
[![Documentation](https://docs.rs/super_fair_division/badge.svg)](https://docs.rs/super_fair_division)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

A Rust library for fair division algorithms.

## Features

- Calculate super fair division based on input values
- Support for both equal weights and weighted fair division
- Implementation of specialized super fair division algorithm
- Support for large integers (i128)
- Comprehensive error handling
- Requires at least 2 participants for fair division

## Algorithm

超公平分配是一种对于不可分割的共享物品的分配处理程序.
出价最高的参与者会得到该不可分割的物品,其他参与者按照物品的主观估值得到对应的补偿.
该分割模型是基于公平核的分配模式.
其中,公平核是所有参与者应该得到的主观公平值,其大小仅仅由参与者自己的报价得到.
公平核的计算是反策略勾结的,仅仅由参与者自己的主观偏好决定.

### Equal Weights Super Fair Division

1. 每个参与者的公平收益值为 v/n，其中v是该参与者的报价，n是参与者总数
2. 计算超公平数值 delta = (n*maxV-sumV)/(n*n)，其中maxV是最高报价，sumV是所有报价之和
3. 每个参与者的收益值更新为 v/n + delta
4. 最后，修正出价最高的用户的收益为其他所有人收益之和的负值

### Weighted Super Fair Division

1. 将权重比例转成正整数比例
2. 按照权重比例计算总人数 n（总人数就是权重之和）
3. 计算超公平数值 delta = (n*maxV_weighted-sumV)/(n*n)，其中maxV_weighted是最高报价乘以其权重
4. 每个人的收益为 (v/n+delta)*weight
5. 最后，修正出价最高的用户的收益为其他所有人收益之和的负值

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
super_fair_division = "0.1.0"
```

## Usage

### Equal Weights

```rust
use super_fair_division::calculate_fair_division_equal_weights;

fn main() {
    let values = vec![100, 200, 300, 400, 500];

    match calculate_fair_division_equal_weights(&values) {
        Ok(allocation) => println!("Fair division allocation: {:?}", allocation),
        Err(err) => eprintln!("Error calculating fair division: {:?}", err),
    }
}
```

### Weighted Division

```rust
use super_fair_division::calculate_fair_division_weighted;

fn main() {
    let values = vec![100, 200, 300, 400, 500];
    let weights = vec![1, 2, 3, 4, 5]; // 权重比例为 1:2:3:4:5

    match calculate_fair_division_weighted(&values, &weights) {
        Ok(allocation) => println!("Weighted fair division allocation: {:?}", allocation),
        Err(err) => eprintln!("Error calculating weighted fair division: {:?}", err),
    }
}
```

## Examples

See the [examples](examples/) directory for more usage examples.

## Documentation

For detailed documentation, visit [docs.rs/super_fair_division](https://docs.rs/super_fair_division).

## Testing

Run the tests with:

```bash
cargo test
```

## Benchmarking

Run the benchmarks with:

```bash
cargo bench
```

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## Repository

The source code is available on [GitHub](https://github.com/iunknow588/super_fair_division).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
# super_fair_division
