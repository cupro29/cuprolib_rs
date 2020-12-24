pub trait Monoid {
    type S: Clone;
    fn id() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct Segtree<M: Monoid> {
    n: usize,
    sz: usize,
    lg: usize,
    d: Vec<M::S>,
}
impl<M: Monoid> Segtree<M> {
    pub fn new(size: usize) -> Self {
        let mut log = 0;
        while 1 << log < size {
            log += 1;
        }
        Self {
            n: size,
            sz: 1 << log,
            lg: log,
            d: vec![M::id(); 2 << log],
        }
    }
    pub fn set(&mut self, p: usize, x: M::S) {
        assert!(p < self.n);
        let p = p + self.sz;
        self.d[p] = x;
        for i in 0..self.lg {
            self.update(p >> 1 + i);
        }
    }
    pub fn get(&self, p: usize) -> M::S {
        assert!(p < self.n);
        self.d[p + self.sz].clone()
    }
    pub fn prod(&self, left: usize, right: usize) -> M::S {
        assert!(left <= right);
        assert!(right <= self.n);
        let mut sml = M::id();
        let mut smr = M::id();
        let mut l = left + self.sz;
        let mut r = right + self.sz;
        while l < r {
            if l & 1 == 1 {
                sml = M::op(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = M::op(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&sml, &smr)
    }
    pub fn all_prod(&self) -> M::S {
        self.d[1].clone()
    }
    fn update(&mut self, k: usize) {
        self.d[k] = M::op(&self.d[k * 2], &self.d[k * 2 + 1]);
    }
}

#[cfg(tests)]
mod tests {
    use super::Segtree;

    #[test]
    fn test_segtree() {
        let mut t = Segtree::new(10);
    }
}
