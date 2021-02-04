pub struct LazySegree<S, T, F, G, H> {
    n: usize,
    sz: usize,
    lg: usize,
    d: Vec<S>,
    op: F,
    e: S,
    lz: Vec<T>,
    mapping: G,
    composition: H,
    id: T,
}
impl<S, T, F, G, H> LazySegree<S, T, F, G, H>
where
    S: Copy,
    T: Copy,
    F: Fn(S, S) -> S,
    G: Fn(T, S) -> S,
    H: Fn(T, T) -> T,
{
    pub fn new(n: usize, op: F, e: S, mapping: G, composition: H, id: T) -> Self {
        let mut lg = 0;
        while 1 << lg < n {
            lg += 1;
        }
        let sz = 1 << lg;
        Self {
            n,
            sz,
            lg,
            d: vec![e; sz * 2],
            op,
            e,
            lz: vec![id; sz],
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
        self.d[p] = x;
        for i in 1..=self.lg {
            self.update(p >> i);
        }
    }
    pub fn get(&mut self, index: usize) -> S {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..=self.lg).rev() {
            self.push(p >> i);
        }
        self.d[p]
    }
    pub fn prod(&mut self, left: usize, right: usize) -> S {
        assert!(left <= right);
        assert!(right <= self.n);
        if left == right {
            return self.e;
        }
        let mut l = left + self.sz;
        let mut r = right + self.sz;
        for i in (1..=self.lg).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }
        let mut sml = self.e;
        let mut smr = self.e;
        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(sml, self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(self.d[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(sml, smr)
    }
    pub fn apply(&mut self, index: usize, f: T) {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..=self.lg).rev() {
            self.push(p >> i);
        }
        self.d[p] = (self.mapping)(f, self.d[p]);
        for i in 1..=self.lg {
            self.update(p >> i);
        }
    }
    pub fn range_apply(&mut self, left: usize, right: usize, f: T) {
        assert!(left <= right);
        assert!(right <= self.n);
        if left == right {
            return;
        }
        let mut l = left + self.sz;
        let mut r = right + self.sz;
        for i in (1..=self.lg).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let l2 = l;
            let r2 = r;
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
            l = l2;
            r = r2;
        }
        for i in 1..=self.lg {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }
    fn update(&mut self, k: usize) {
        self.d[k] = (self.op)(self.d[2 * k], self.d[2 * k + 1]);
    }
    fn all_apply(&mut self, k: usize, f: T) {
        self.d[k] = (self.mapping)(f, self.d[k]);
        if k < self.sz {
            self.lz[k] = (self.composition)(f, self.lz[k]);
        }
    }
    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lz[k]);
        self.all_apply(2 * k + 1, self.lz[k]);
        self.lz[k] = self.id;
    }
}
