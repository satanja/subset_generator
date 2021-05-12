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

///
/// # Examples
///
/// ```
/// use subset_generator::SubsetGenerator;
/// let data = vec![(0, 0), (1, 0)];
/// let sg = SubsetGenerator::new(&data, false);
///
/// let mut iter = sg.iter();
/// assert_eq!(sg.iter().count(), 3);
/// ```
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
    /// Examples
    /// ```
    /// use subset_generator::SubsetGenerator;
    ///
    /// let data = vec![1, 2, 3, 4];
    /// let sg = SubsetGenerator::new(&data, false);
    /// ```
    pub fn new(data: &Vec<T>, with_emptyset: bool) -> SubsetGenerator<T> {
        SubsetGenerator {
            data,
            with_emptyset,
        }
    }

    /// Consumes self and returns an iterator over all the subsets. The
    /// underlying dataset cannot be consumed, so references are still returned.
    /// The generator returns an emptyset if (and only if) the generator was
    /// configured to include the empty set. In case the underlying dataset does
    /// not mutate, consider using the `iter` version instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use subset_generator::SubsetGenerator;
    ///
    /// let mut data = vec![1, 2, 3];
    /// let sg = SubsetGenerator::new(&data, true);
    /// assert_eq!(sg.into_iter().count(), 8);
    ///
    /// data = vec![42];
    /// // additional computations...
    /// ```
    pub fn into_iter(self) -> SubsetIter<'a, T> {
        let len = self.data.len();
        SubsetIter {
            data: &self.data,
            set: BitVec::from_elem(len, false),
            with_emptyset: self.with_emptyset,
        }
    }

    /// Returns an iterator over all the subsets of the given dataset.
    /// The generator returns an emptyset if (and only if) the generator was
    /// configured to include the empty set.
    ///
    /// # Examples
    ///
    /// ```
    /// use subset_generator::SubsetGenerator;
    ///
    /// let data = vec![1, 2, 3];
    /// let sg = SubsetGenerator::new(&data, true);
    /// assert_eq!(sg.iter().count(), 8);
    ///
    /// let sg = SubsetGenerator::new(&data, false);
    /// assert_eq!(sg.iter().count(), 7);
    /// ```
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
    /// Adds 1 to the underlying BitVec. This effectively computes the next
    /// subset. Returns false if all the bits were set, and so all subsets have
    /// been exhausted.
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
