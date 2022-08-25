use super::Sort;

pub struct QuickSort;

impl Sort for QuickSort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd,
    {
        quicksort(ar, 0, ar.len() as isize - 1);
    }
}

fn partition<T: PartialOrd>(ar: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right as usize;
    let mut i = left - 1;

    for j in left..right {
        if ar[j as usize] <= ar[pivot] {
            i += 1;
            ar.swap(j as usize, i as usize);
        }
    }
    ar.swap(i as usize + 1, right as usize);
    i + 1
}

fn quicksort<T: PartialOrd>(ar: &mut [T], left: isize, right: isize) {
    if left < right {
        let pivot = partition(ar, left, right);
        quicksort(ar, left, pivot - 1);
        quicksort(ar, pivot + 1, right)
    }
}
