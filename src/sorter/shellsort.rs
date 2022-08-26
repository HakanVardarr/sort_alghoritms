use super::Sort;

pub struct ShellSort;

impl Sort for ShellSort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy,
    {
        let optimal_gap = [701, 301, 132, 57, 23, 10, 4, 1];
        let len = ar.len();

        for gap in optimal_gap.iter() {
            for i in *gap..len {
                let mut j = i;
                while j >= *gap && &ar[j - gap] > &ar[j] {
                    ar.swap(j - gap, j);
                    j -= gap;
                }
            }
        }
    }
}
