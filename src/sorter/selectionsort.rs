use super::Sort;

pub struct SelectionSort;

impl Sort for SelectionSort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy,
    {
        for i in 0..ar.len() {
            let mut smallest = i;
            for j in i + 1..ar.len() {
                if &ar[smallest] > &ar[j] {
                    smallest = j;
                }
            }
            if i != smallest{
                ar.swap(smallest, i);   
            }
        }
    }
}
