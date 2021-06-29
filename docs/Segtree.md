# Segtree
1点更新、区間取得。

[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/segtree.rs)


## new
```rust
pub fn new(n: usize, op: F, id: T) -> Self
```
`F: Fn(T, T) -> T`

## from_vec
```rust
pub fn from_vec(v: &[T], op: F, id: T) -> Self
```

## set
```rust
pub fn set(&mut self, p: usize, x: T)
```

## get
```rust
pub fn get(&self, p: usize) -> T
```

## prod
```rust
pub fn prod(&self, left: usize, right: usize) -> T
```

## all_prod
```rust
pub fn all_prod(&self) -> T
```

## max_right
```rust
pub fn max_right<G>(&self, left: usize, func: G) -> usize
where
    G: Fn(T) -> bool
```

## min_left
```rust
pub fn min_left<G>(&self, right: usize, func: G) -> usize
where
    G: Fn(T) -> bool
```

## Verify
- [Library Checker: Static RMQ](https://github.com/cupro29/cuprolib_rs/blob/main/examples/library-checker-staticrmq.rs)
