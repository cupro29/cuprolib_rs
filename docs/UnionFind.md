# UnionFind
[src](https://github.com/cupro29/cuprolib_rs/blob/main/src/unionfind.rs)

Union-Find木。

## new
```rust
pub fn new(size: usize) -> Self
```
要素数 `size` のUnion-Find木を宣言する。

## merge
```rust
pub fn merge(&mut self, a: usize, b: usize) -> usize
```
要素 `a` のグループと、要素 `b` のグループを併合する。
返り値は、併合された後のグループの代表元。

## same
```rust
pub fn same(&mut self, a: usize, b: usize) -> bool
```
要素 `a` と、要素 `b` が同じグループに属するかを調べる。
同じなら `true` 、違うなら `false` を返す。

## size
```rust
pub fn size(&mut self, a: usize) -> usize
```
要素 `a` が属するグループの大きさを返す。

## leader
```rust
pub fn leader(&mut self, a: usize) -> usize
```
要素 `a` が属するグループの代表元を返す。

## Verify
- [Library Checker: Unionfind](https://github.com/cupro29/cuprolib_rs/blob/main/examples/library-checker-unionfind.rs)
