# LazySegtree
区間更新、区間取得。

[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/lazysegtree.rs)

## new
```rust
pub fn new(n: usize, op: F, e: S, mapping: G, composition: H, id: T) -> Self
```
```rust
F: Fn(S, S) -> S,
G: Fn(T, S) -> S,
H: Fn(T, T) -> T,
```

## set
```rust
pub fn set(&mut self, index: usize, x: S)
```

## get
```rust
pub fn get(&mut self, index: usize) -> S
```

## prod
```rust
pub fn prod(&mut self, left: usize, right: usize) -> S
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
