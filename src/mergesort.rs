#![allow(unused)]

use super::Sort;

pub struct MergeSort;

impl Sort for MergeSort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy,
    {
        merge_sort(ar);
    }
}

fn merge_sort<T: PartialOrd + Copy>(ar: &mut [T]) {
    let len = ar.len();
    let mid = len / 2;

    if mid == 0 {
        return;
    }

    merge_sort(&mut ar[mid..]);
    merge_sort(&mut ar[..mid]);

    let mut temp = ar.to_vec();

    _merge_sort(&ar[mid..], &ar[..mid], &mut temp);

    ar.copy_from_slice(&temp[..]);
}

fn _merge_sort<T: PartialOrd + Copy>(ar1: &[T], ar2: &[T], temp: &mut [T]) {
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    while left < ar1.len() && right < ar2.len() {
        if ar1[left] <= ar2[right] {
            temp[index] = ar1[left];
            left += 1;
        } else {
            temp[index] = ar2[right];
            right += 1;
        }
        index += 1;
    }

    while right < ar2.len() {
        temp[index] = ar2[right];
        right += 1;
        index += 1;
    }

    while left < ar1.len() {
        temp[index] = ar1[left];
        left += 1;
        index += 1;
    }
}
