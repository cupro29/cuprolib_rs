# `DualSegtree<T, F>`
区間更新、1点取得。

[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/dualsegtree.rs)

## new
```rust
pub fn new(n: usize, op: F, id: T) -> Self
```

## from_vec
```rust
pub fn from_vec(v: &[T], op: F, id: T) -> Self
```

## set
```rust
pub fn set(&mut self, index: usize, x: T)
```

## get
```rust
pub fn get(&mut self, index: usize) -> T
```

## apply
```rust
pub fn apply(&mut self, index: usize, f: T)
```

## range_apply
```rust
pub fn range_apply(&mut self, left: usize, right: usize, f: T)
```

## Verify
