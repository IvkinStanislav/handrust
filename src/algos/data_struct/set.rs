use std::collections::{HashMap, HashSet};

pub enum HashSetOperation {
    Insert(usize),
    Check(usize),
}

pub struct HashSetOperationPerformer<T> {
    set: HashSet<usize>,
    ops: T,
}

impl<T: Iterator<Item = HashSetOperation>> Iterator for HashSetOperationPerformer<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        use HashSetOperation::*;

        for op in self.ops.by_ref() {
            match op {
                Insert(value) => {
                    self.set.insert(value);
                }
                Check(value) => {
                    return Some(self.set.contains(&value));
                }
            }
        }
        None
    }
}

pub fn count_interesting_couples<T: ToString>(words: impl Iterator<Item = T>) -> usize {
    let words: HashSet<String> = HashSet::from_iter(words.map(|w| w.to_string()));
    let Some(len) = words.iter().next().map(|w| w.len()) else {
        return 0;
    };

    let mut map = HashMap::new();
    let mut count = 0;
    for i in 0..len {
        for original_word in words.iter() {
            let mut word = original_word.to_string();
            word.replace_range(i..i + 1, "_");
            map.entry(word).and_modify(|c| *c += 1).or_insert(1);
        }
        count += map.values().map(|c| c * (c - 1) / 2).sum::<usize>();
        map.clear();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perform_operations_test() {
        use HashSetOperation::*;

        let iter = HashSetOperationPerformer {
            set: HashSet::new(),
            ops: [
                Check(5),
                Insert(5),
                Check(5),
                Insert(6),
                Insert(10),
                Check(7),
                Insert(7),
                Check(10),
                Check(7),
            ]
            .into_iter(),
        };
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![false, true, false, true, true]
        );
    }

    #[test]
    fn count_interesting_couples_test() {
        assert_eq!(
            count_interesting_couples(["rom", "bom", "dom", "bot", "rot"].into_iter()),
            6
        );
        assert_eq!(count_interesting_couples(["aa", "aa", "aa"].into_iter()), 0);
        assert_eq!(
            count_interesting_couples(["aaa", "aaB", "aBa", "Baa", "BBB", "abb"].into_iter()),
            3
        );
    }
}
