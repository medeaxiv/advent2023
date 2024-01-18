pub trait SliceExt<T> {
    fn multi_index_mut(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)>;
}

impl<T> SliceExt<T> for [T] {
    fn multi_index_mut(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)> {
        if a == b {
            return None;
        }

        if a < b {
            let (low, high) = self.split_at_mut(b);
            Some((&mut low[a], &mut high[0]))
        } else {
            let (low, high) = self.split_at_mut(a);
            Some((&mut high[0], &mut low[b]))
        }
    }
}
