pub mod binary_search;
pub mod dominant_element_search;
pub mod inversions_counting;
pub mod modification_of_quicksort;
pub mod nearest_points_pair;

pub struct MergeIterator<I: Iterator> {
    iter1: std::iter::Peekable<I>,
    iter2: std::iter::Peekable<I>,
}

impl<I: Iterator> MergeIterator<I> {
    pub fn new<II: IntoIterator<IntoIter = I>>(source1: II, source2: II) -> Self {
        Self {
            iter1: source1.into_iter().peekable(),
            iter2: source2.into_iter().peekable(),
        }
    }
}

impl<T: Ord, I: Iterator<Item = T>> Iterator for MergeIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering::*;

        match (self.iter1.peek(), self.iter2.peek()) {
            (Some(e1), Some(e2)) => match e1.cmp(&e2) {
                Less | Equal => self.iter1.next(),
                Greater => self.iter2.next(),
            },
            (Some(_), None) => self.iter1.next(),
            (None, Some(_)) => self.iter2.next(),
            (None, None) => None,
        }
    }
}
