pub struct WeightedUnionFind<T> {
    _size: usize,
    parent: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<T>,
}
impl<T> WeightedUnionFind<T>
where
    T: Copy
        + Default
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::Neg<Output = T>
        + std::ops::Sub<Output = T>,
{
    pub fn new(size: usize) -> Self {
        Self {
            _size: size,
            parent: (0..size).collect(),
            rank: vec![0; size],
            diff_weight: vec![T::default(); size],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        let p = self.parent[x];
        if p == x {
            x
        } else {
            let r = self.root(p);
            let d = self.diff_weight[p];
            self.diff_weight[x] += d;
            self.parent[x] = r;
            r
        }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn merge(&mut self, mut x: usize, mut y: usize, mut w: T) -> bool {
        w += self.weight(x);
        w -= self.weight(y);
        x = self.root(x);
        y = self.root(y);
        if x == y {
            return false;
        }
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
            w = w.neg();
        }
        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }
        self.parent[y] = x;
        self.diff_weight[y] = w;
        true
    }
    pub fn diff(&mut self, x: usize, y: usize) -> Option<T> {
        if self.same(x, y) {
            Some(self.weight(y) - self.weight(x))
        } else {
            None
        }
    }
    fn weight(&mut self, x: usize) -> T {
        self.root(x);
        self.diff_weight[x]
    }
}
