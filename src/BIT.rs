pub struct BIT<T> {
    sz: usize,
    d: Vec<T>,
}
impl<T> BIT<T>
where
    T: Clone + std::ops::AddAssign + Default,
{
    pub fn new(n: usize) -> Self {
        Self {
            sz: n,
            d: vec![T::default(); n],
        }
    }
    pub fn add(&mut self, idx: usize, x: T) {
        assert!(idx < self.sz);
        let mut p = idx + 1;
        while p <= self.sz {
            self.d[p - 1] += x.clone();
            p += p & p.wrapping_neg();
        }
    }
    pub fn range(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.sum(r) - self.sum(l)
    }
    pub fn sum(&self, idx: usize) -> T {
        let mut res = T::default();
        let mut r = idx;
        while r > 0 {
            res += self.d[r - 1].clone();
            r -= r & r.wrapping_neg();
        }
        res
    }
}
