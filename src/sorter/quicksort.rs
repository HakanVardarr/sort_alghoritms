use super::Sort;

pub struct QuickSort;

impl Sort for QuickSort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy,
    {
        quick_sort(ar, 0, ar.len() as isize - 1);
    }
}

fn quick_sort<T: PartialOrd + Copy>(ar: &mut [T], left: isize, right: isize) {
    if left < right as isize {
        let p = _quick_sort(ar, left, right);
        quick_sort(ar, left, p - 1);
        quick_sort(ar, p + 1, right);
    }
}

fn _quick_sort<T: PartialOrd + Copy>(ar: &mut [T], left: isize, right: isize) -> isize {
    let mut i = left - 1;

    for j in left..right {
        if ar[j as usize] <= ar[right as usize] {
            i += 1;
            ar.swap(j as usize, i as usize);
        }
    }
    i += 1;
    ar.swap(i as usize, right as usize);
    i
}
