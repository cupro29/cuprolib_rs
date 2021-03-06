pub struct Segtree<T, F> {
    n: usize,
    sz: usize,
    d: Vec<T>,
    op: F,
    id: T,
}
impl<T, F> Segtree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(n: usize, op: F, id: T) -> Self {
        Self::from_vec(&vec![id; n], op, id)
    }
    pub fn from_vec(v: &[T], op: F, id: T) -> Self {
        let n = v.len();
        let mut lg = 0;
        while 1 << lg < n {
            lg += 1;
        }
        let sz = 1 << lg;
        let mut d = vec![id; sz * 2];
        d[sz..(sz + n)].clone_from_slice(&v);
        let mut res = Segtree { n, sz, d, op, id };
        for i in (1..sz).rev() {
            res.update(i);
        }
        res
    }
    pub fn set(&mut self, p: usize, x: T) {
        assert!(p < self.n);
        let mut q = p + self.sz;
        self.d[q] = x;
        while q >> 1 > 0 {
            q >>= 1;
            self.update(q);
        }
    }
    pub fn get(&self, p: usize) -> T {
        assert!(p < self.n);
        self.d[p + self.sz].clone()
    }
    pub fn prod(&self, left: usize, right: usize) -> T {
        assert!(left <= right);
        assert!(right <= self.n);
        let mut sml = self.id;
        let mut smr = self.id;
        let mut l = left + self.sz;
        let mut r = right + self.sz;
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
    pub fn all_prod(&self) -> T {
        self.d[1].clone()
    }
    pub fn max_right<G>(&self, left: usize, func: G) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(left <= self.n);
        assert!(func(self.id));
        if left == self.n {
            return self.n;
        }
        let mut l = left + self.sz;
        let mut sm = self.id;
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !func((self.op)(sm, self.d[l])) {
                while l < self.sz {
                    l = 2 * l;
                    if func((self.op)(sm, self.d[l])) {
                        sm = (self.op)(sm, self.d[l]);
                        l += 1;
                    }
                }
                return l - self.sz;
            }
            sm = (self.op)(sm, self.d[l]);
            l += 1;
            if (l & l.wrapping_neg()) == l {
                break;
            }
        }
        self.n
    }
    pub fn min_left<G>(&self, right: usize, func: G) -> usize
    where
        G: Fn(T) -> bool,
    {
        assert!(right <= self.n);
        assert!(func(self.id));
        if right == 0 {
            return 0;
        }
        let mut r = right + self.sz;
        let mut sm = self.id;
        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !func((self.op)(self.d[r], sm)) {
                while r < self.sz {
                    r = 2 * r + 1;
                    if func((self.op)(self.d[r], sm)) {
                        sm = (self.op)(self.d[r], sm);
                        r -= 1;
                    }
                }
                return r + 1 - self.sz;
            }
            sm = (self.op)(self.d[r], sm);
            if (r & r.wrapping_neg()) == r {
                break;
            }
        }
        0
    }
    fn update(&mut self, k: usize) {
        self.d[k] = (self.op)(self.d[k * 2], self.d[k * 2 + 1]);
    }
}
