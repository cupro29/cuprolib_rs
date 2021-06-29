# `MinCostFlowGraph<Flow, Cost>`
容量スケーリング法。

[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/mincostflow_capacity_scaling.rs)

## new
```rust
pub fn new(n: usize) -> Self
```

## add_edge
```rust
pub fn add_edge(&mut self, from: usize, to: usize, lower: Flow, upper: Flow, cost: Cost)
```

## add_supplly
```rust
pub fn add_supply(&mut self, v: usize, amount: Flow)
```

## add_demand
```rust
pub fn add_demand(&mut self, v: usize, amount: Flow)
```

## solve
```rust
pub fn solve(&mut self) -> Result<Cost, Cost>
```

## edges
```rust
pub fn edges(&self) -> Vec<MinCostFlowEdge<Flow, Cost>>
```

## potential
```rust
pub fn potential(&self) -> Vec<Cost>
```

## Verify
- [Library Checker: Minimum cost b-flow](https://github.com/cupro29/cuprolib_rs/blob/main/examples/library-checker-min_cost_b_flow.rs)