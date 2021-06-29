# `MinCostFlowGraph<T, U>`
[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/mincostflow.rs)

## new
```rust
pub fn new(n: usize) -> Self
```

## add_edge
```rust
pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: U)
```

## min_cost_flow
```rust
pub fn min_cost_flow(&mut self, s: usize, t: usize, mut flow_limit: T) -> Option<U>
```
`flow_limit` だけ流せたら `Some(cost)` 、
流せなかったら `None` を返す。

## Verify
- [AOJ: Minimum Cost Flow](https://github.com/cupro29/cuprolib_rs/blob/main/examples/aoj-minimum_cost_flow.rs)