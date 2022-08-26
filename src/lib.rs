#![allow(unused)]

pub mod sorter;
use sorter::{SortTypes, Sorter};

trait Sort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubblesort() {
        let mut ar = [1, 5, 3, 2, 4, 4];

        let sorter = Sorter {
            sort_type: SortTypes::BubbleSort,
        };
        sorter.sort(&mut ar);

        assert_eq!([1, 2, 3, 4, 4, 5], ar);
    }
    #[test]
    fn insertionsort() {
        let mut ar = [1, 5, 3, 2, 4, 4];

        let sorter = Sorter {
            sort_type: SortTypes::InsertionSort,
        };
        sorter.sort(&mut ar);

        assert_eq!([1, 2, 3, 4, 4, 5], ar);
    }
    #[test]
    fn mergesort() {
        let mut ar = [1, 5, 3, 2, 4, 4];

        let sorter = Sorter {
            sort_type: SortTypes::MergeSort,
        };
        sorter.sort(&mut ar);

        assert_eq!([1, 2, 3, 4, 4, 5], ar);
    }
    #[test]
    fn quicksort() {
        let mut ar = [1, 5, 3, 2, 4, 4];

        let sorter = Sorter {
            sort_type: SortTypes::QuickSort,
        };
        sorter.sort(&mut ar);

        assert_eq!([1, 2, 3, 4, 4, 5], ar);
    }
    #[test]
    fn selectionsort() {
        let mut ar = [1, 5, 3, 2, 4, 4];

        let sorter = Sorter {
            sort_type: SortTypes::SelectionSort,
        };
        sorter.sort(&mut ar);

        assert_eq!([1, 2, 3, 4, 4, 5], ar);
    }
    #[test]
    fn shellsort() {
        let mut ar = [1, 5, 3, 2, 4, 4];

        let sorter = Sorter {
            sort_type: SortTypes::ShellSort,
        };
        sorter.sort(&mut ar);

        assert_eq!([1, 2, 3, 4, 4, 5], ar);
    }
}
