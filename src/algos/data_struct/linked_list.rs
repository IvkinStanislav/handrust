use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub enum LinkedList {
    Cons(usize, Rc<RefCell<LinkedList>>),
    Nil,
}

impl LinkedList {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::Nil))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<T: DoubleEndedIterator<Item = usize>>(mut iter: T) -> Rc<RefCell<Self>> {
        let mut head = LinkedList::new();
        while let Some(item) = iter.next_back() {
            head = Rc::new(RefCell::new(Self::Cons(item, head)));
        }
        head
    }

    fn shift_right(mut head: Rc<RefCell<Self>>, mut i: usize) -> Rc<RefCell<Self>> {
        use LinkedList::*;

        while i > 0 {
            let new_head = match head.borrow().deref() {
                Cons(_, next) => next.clone(),
                Nil => break,
            };
            head = new_head;
            i -= 1;
        }
        head
    }
}

pub enum LinkedListOperation {
    Insert(usize, usize),
    Delete(usize),
    Get(usize),
}

pub struct LinkedListOperationPerformer<T> {
    head: Rc<RefCell<LinkedList>>,
    ops: T,
}

impl<T: Iterator<Item = LinkedListOperation>> Iterator for LinkedListOperationPerformer<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        use LinkedList::*;
        use LinkedListOperation::*;

        for op in self.ops.by_ref() {
            match op {
                Insert(i, data) => {
                    if i == 0 {
                        let head = self.head.clone();
                        self.head = Rc::new(RefCell::new(Cons(data, head)));
                        continue;
                    }

                    let head = LinkedList::shift_right(self.head.clone(), i - 1);

                    match head.borrow_mut().deref_mut() {
                        Cons(_, next) => {
                            let second_next = next.clone();
                            *next = Rc::new(RefCell::new(Cons(data, second_next)));
                        }
                        Nil => {}
                    };
                }
                Delete(i) => {
                    if i == 1 {
                        let head = self.head.clone();
                        match head.borrow().deref() {
                            Cons(_, next) => {
                                self.head = next.clone();
                            }
                            Nil => {}
                        };
                        continue;
                    }

                    let head = LinkedList::shift_right(self.head.clone(), i - 2);

                    match head.borrow_mut().deref_mut() {
                        Cons(_, deleted) => {
                            let deleted_next = match deleted.borrow().deref() {
                                Cons(_, next) => next.clone(),
                                Nil => Rc::new(RefCell::new(LinkedList::Nil)),
                            };
                            *deleted = deleted_next;
                        }
                        Nil => {}
                    };
                }
                Get(i) => {
                    let head = LinkedList::shift_right(self.head.clone(), i - 1);
                    return match head.borrow().deref() {
                        Cons(data, _) => Some(*data),
                        Nil => None,
                    };
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perform_operations_test() {
        use LinkedListOperation::*;

        let iter = LinkedListOperationPerformer {
            head: LinkedList::new(),
            ops: [
                Insert(0, 5),
                Get(1),
                Delete(1),
                Insert(0, 6),
                Insert(0, 7),
                Get(2),
                Insert(1, 5),
                Get(2),
            ]
            .into_iter(),
        };
        assert_eq!(iter.collect::<Vec<_>>(), vec![5, 6, 5]);
    }

    #[test]
    fn max_min_ord_diff_test() {
        fn max_min_ord_diff(list: impl IntoIterator<Item = isize>) -> (usize, usize, usize, usize) {
            let mut list: Vec<_> = list.into_iter().enumerate().collect();

            let mut min = isize::MAX;
            let mut min_i = 0;
            let mut min_j = 0;
            let mut max = isize::MIN;
            let mut max_i = 0;
            let mut max_j = 0;
            let mut checked = (0, 0);

            list.sort_by(|(_, a), (_, b)| a.cmp(b));

            for i in 0..list.len() {
                for j in (i + 1..list.len()).rev() {
                    if i >= checked.0 && j <= checked.1 {
                        continue;
                    }

                    let (index_i, value_i) = list[i];
                    let (index_j, value_j) = list[j];
                    let temp = value_i - value_j;
                    if index_i < index_j && temp < min {
                        min = temp;
                        min_i = index_i;
                        min_j = index_j;
                        checked = (i, j);
                        break;
                    }
                }
            }

            list.reverse();
            checked = (0, 0);

            for i in 0..list.len() {
                for j in (i + 1..list.len()).rev() {
                    if i >= checked.0 && j <= checked.1 {
                        continue;
                    }

                    let (index_i, value_i) = list[i];
                    let (index_j, value_j) = list[j];
                    let temp = value_i - value_j;
                    if index_i < index_j && temp > max {
                        max = temp;
                        max_i = index_i;
                        max_j = index_j;
                        checked = (i, j);
                        break;
                    }
                }
            }

            (min_i + 1, min_j + 1, max_i + 1, max_j + 1)
        }

        assert_eq!(max_min_ord_diff([2, 1, 3, 5, 2, 4]), (2, 4, 4, 5));
        assert_eq!(max_min_ord_diff([3, 2, 4, 5, 6]), (2, 5, 1, 2));
    }
}
