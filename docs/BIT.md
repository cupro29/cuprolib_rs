# BIT
1点加算、区間和の取得。

[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/binaryindexedtree.rs)

## new
```rust
pub fn new(n: usize) -> Self
```

## add
```rust
pub fn add(&mut self, idx: usize, x: T)
```

## range
```rust
pub fn range(&self, l: usize, r: usize) -> T
where
    T: std::ops::Sub<Output = T>,
```

## Verify
- [Library Checker: Static Range Sum](https://github.com/cupro29/cuprolib_rs/blob/main/examples/library-checker-static_range_sum.rs)
- [Library Checker: Point Add Range Sum](https://github.com/cupro29/cuprolib_rs/blob/main/examples/library-checker-point_add_range_sum.rs)
