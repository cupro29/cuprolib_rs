# `MaxFlowGraph<Cap>`
Dinic法。

[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/maxflow.rs)

## new
```rust
pub fn new(n: usize) -> Self
```

## add_edge
```rust
pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap)
```

## maxflow
```rust
pub fn maxflow(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap
```

## edges
```rust
pub fn edges(&self) -> Vec<MaxFlowEdge<Cap>>
```
```rust
#[derive(Debug, Clone)]
pub struct MaxFlowEdge<Cap> {
    pub from: usize,
    pub to: usize,
    pub flow: Cap,
}
```

## Verify
- [AOJ: Maximum Flow](https://github.com/cupro29/cuprolib_rs/blob/main/examples/aoj-maximum_flow.rs)