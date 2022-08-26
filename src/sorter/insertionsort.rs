use super::Sort;

pub struct InsertionSort;

impl Sort for InsertionSort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy,
    {
        for i in 1..ar.len() {
            let mut j = i;
            while j > 0 && &ar[j - 1] > &ar[j] {
                ar.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}
