#![allow(unused)]

use super::Sort;

mod bubblesort;
mod insertionsort;
mod mergesort;
mod quicksort;
mod selectionsort;
mod shellsort;

pub enum SortTypes {
    BubbleSort,
    InsertionSort,
    MergeSort,
    QuickSort,
    SelectionSort,
    ShellSort,
}

/// Sorter for easy sorting.
///
/// # Examples
///
/// ```
/// let sorter = sort_algos::sorter::Sorter {
///         sort_type: sort_algos::sorter::SortTypes::BubbleSort,
///     };
///
/// let mut ar = [5,4,3,2,1];
/// sorter.sort(&mut ar);
///  
/// ```
pub struct Sorter {
    pub sort_type: SortTypes,
}

impl Sorter {
    pub fn sort<T: PartialOrd + Copy>(&self, ar: &mut [T]) {
        match self.sort_type {
            SortTypes::BubbleSort => {
                bubblesort::BubbleSort::sort(ar);
            }
            SortTypes::InsertionSort => {
                insertionsort::InsertionSort::sort(ar);
            }
            SortTypes::MergeSort => {
                mergesort::MergeSort::sort(ar);
            }
            SortTypes::QuickSort => {
                quicksort::QuickSort::sort(ar);
            }
            SortTypes::SelectionSort => {
                selectionsort::SelectionSort::sort(ar);
            }
            SortTypes::ShellSort => {
                shellsort::ShellSort::sort(ar);
            }
        }
    }
}
