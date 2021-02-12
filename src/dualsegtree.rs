pub struct DualSegtree<S, T, F, G> {
    n: usize,
    sz: usize,
    lg: usize,
    d: Vec<S>,
    lz: Vec<T>,
    mapping: F,
    composition: G,
    id: T,
}
impl<S, T, F, G> DualSegtree<S, T, F, G>
where
    S: Copy + Default,
    T: Copy,
    F: Fn(T, S) -> S,
    G: Fn(T, T) -> T,
{
    pub fn new(n: usize, mapping: F, composition: G, id: T) -> Self {
        DualSegtree::from_vec(&vec![S::default(); n], mapping, composition, id)
    }
    pub fn from_vec(v: &[S], mapping: F, composition: G, id: T) -> Self {
        let n = v.len();
        let mut lg = 0;
        while 1 << lg < n {
            lg += 1;
        }
        let sz = 1 << lg;
        let mut d = vec![S::default(); sz];
        let lz = vec![id; sz];
        d[0..n].clone_from_slice(&v);
        Self {
            n,
            sz,
            lg,
            d,
            lz,
            mapping,
            composition,
            id,
        }
    }
    pub fn set(&mut self, index: usize, x: S) {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..=self.lg).rev() {
            self.push(p >> i);
        }
        self.d[index] = x;
    }
    pub fn get(&mut self, index: usize) -> S {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..=self.lg).rev() {
            self.push(p >> i);
        }
        self.d[index]
    }
    pub fn apply(&mut self, index: usize, f: T) {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..self.lg).rev() {
            self.push(p >> i);
        }
        self.d[index] = (self.mapping)(f, self.d[index]);
    }
    pub fn range_apply(&mut self, left: usize, right: usize, f: T) {
        assert!(left <= right);
        assert!(right <= self.n);
        if left == right {
            return;
        }
        let mut l = left + self.sz;
        let mut r = right + self.sz;
        for i in (1..self.lg).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        while l < r {
            if l & 1 != 0 {
                self.all_apply(l, f);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }
    }
    fn all_apply(&mut self, k: usize, f: T) {
        if k < self.sz {
            self.lz[k] = (self.composition)(f, self.lz[k]);
        } else {
            self.d[k - self.sz] = (self.mapping)(f, self.d[k - self.sz]);
        }
    }
    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lz[k]);
        self.all_apply(2 * k + 1, self.lz[k]);
        self.lz[k] = self.id;
    }
}
