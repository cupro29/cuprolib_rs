pub struct DualSegtree<T, F> {
    n: usize,
    sz: usize,
    lg: usize,
    d: Vec<T>,
    op: F,
    id: T,
}
impl<T, F> DualSegtree<T, F>
where
    T: Copy + PartialEq,
    F: Fn(T, T) -> T,
{
    pub fn new(n: usize, op: F, id: T) -> Self {
        DualSegtree::from_vec(&vec![id; n], op, id)
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
        Self {
            n,
            sz,
            lg,
            d,
            op,
            id,
        }
    }
    pub fn set(&mut self, index: usize, x: T) {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..self.lg).rev() {
            self.push(p >> i);
        }
        self.d[p] = x;
    }
    pub fn get(&mut self, index: usize) -> T {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..self.lg).rev() {
            self.push(p >> i);
        }
        self.d[p]
    }
    pub fn apply(&mut self, index: usize, f: T) {
        assert!(index < self.n);
        let p = index + self.sz;
        for i in (1..self.lg).rev() {
            self.push(p >> i);
        }
        self.d[p] = (self.op)(f, self.d[p]);
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
        self.d[k] = (self.op)(f, self.d[k]);
    }
    fn push(&mut self, k: usize) {
        if self.d[k] != self.id {
            self.all_apply(2 * k, self.d[k]);
            self.all_apply(2 * k + 1, self.d[k]);
            self.d[k] = self.id;
        }
    }
}
