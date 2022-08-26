use super::Sort;

pub struct BubbleSort;

impl Sort for BubbleSort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy,
    {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 0..ar.len() {
                if i == ar.len() - 1 {
                    continue;
                }
                if ar[i] > ar[i + 1] {
                    ar.swap(i, i + 1);
                    sorted = false;
                };
            }
        }
    }
}
