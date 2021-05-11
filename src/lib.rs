//! This library offers an encapsulated iterative subset generator for any 
//! vectorized dataset. Applications are in designing FPT algorithms, such as 
//! iterative compression algorithms, or simple iterative bruteforce algorithms 
//! instead of backtracking algorithms. The subsets of the dataset are
//! *generated* each time the next element of the iterator is called, this 
//! ensures that the memory usage is *O(n)* at any point when using the 
//! generator. The overall complexity is *O(n)* per item, and, of course,
//! *O(n * 2^n)* to generate all items.
use bit_vec::BitVec;
pub struct SubsetGenerator<'a, T> {
    data: &'a Vec<T>,
    with_emptyset: bool,
}

pub struct SubsetIter<'a, T> {
    data: &'a Vec<T>,
    set: BitVec,
    with_emptyset: bool,
}

impl<'a, T> SubsetGenerator<'a, T> {
    /// Constructs a new container holding the (linearized) data set. If
    /// `with_emptyset` is true, then the generator will also output the empty
    /// vector. Otherwise, only subsets with at least one element are reported.
    ///
    /// Example
    /// ```
    /// let data = vec![1, 2, 3, 4];
    /// let sg = SubsetGenerator::new(&data, false);
    /// ```
    pub fn new(data: &Vec<T>, with_emptyset: bool) -> SubsetGenerator<T> {
        SubsetGenerator {
            data,
            with_emptyset,
        }
    }

    pub fn into_iter(self) -> SubsetIter<'a, T> {
        let len = self.data.len();
        SubsetIter {
            data: &self.data,
            set: BitVec::from_elem(len, false),
            with_emptyset: self.with_emptyset,
        }
    }

    pub fn iter(&self) -> SubsetIter<T> {
        let len = self.data.len();
        SubsetIter {
            data: &self.data,
            set: BitVec::from_elem(len, false),
            with_emptyset: self.with_emptyset,
        }
    }
}



impl<'a, T> SubsetIter<'a, T> {
    fn next_set(&mut self) -> bool {
        let mut all_set = true;
        for i in 0..self.set.len() {
            all_set &= self.set[i];
        }
        if all_set {
            return false;
        }

        for i in 0..self.set.len() {
            if self.set[i] {
                self.set.set(i, false);
            } else {
                self.set.set(i, true);
                break;
            }
        }

        true
    }
}

impl<'a, T> Iterator for SubsetIter<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.with_emptyset {
            self.with_emptyset = false;
            return Some(vec![]);
        }

        if self.next_set() {
            let mut result = Vec::new();
            for i in 0..self.set.len() {
                if self.set[i] {
                    result.push(&self.data[i]);
                }
            }
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_set_without_empty() {
        let data = vec![(1, 0), (2, 0), (3, 0), (4, 0)];
        let small_generator = SubsetGenerator::new(&data, false);
        let mut iters = 0;
        for _ in small_generator.iter() {
            iters += 1;
        }
        assert_eq!(iters, 15);
    }

    #[test]
    fn small_set_with_empty() {
        let data = vec![(1, 0), (2, 0), (3, 0), (4, 0)];
        let small_generator = SubsetGenerator::new(&data, true);
        let mut iters = 0;
        for _ in small_generator.iter() {
            iters += 1;
        }
        assert_eq!(iters, 16);
    }

    #[test]
    fn into_iter_test() {
        let mut data = vec![(1, 0), (2, 0), (3, 0), (4, 0)];
        let small_generator = SubsetGenerator::new(&data, false);
        let mut iters = 0;
        for _ in small_generator.into_iter() {
            iters += 1;
        }
        assert_eq!(iters, 15);
        data[0] = (2, 0);
    }
}
