mod bubblesort;
mod mergesort;
mod quicksort;
trait Sort {
    fn sort<T>(ar: &mut [T])
    where
        T: PartialOrd + Copy;
}

#[allow(dead_code)]
fn sort<T, S>(ar: &mut [T])
where
    T: PartialOrd + Copy,
    S: Sort,
{
    S::sort(ar);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubblesort() {
        let mut ar = [1, 5, 3, 2, 4, 4];
        sort::<_, bubblesort::BubbleSort>(&mut ar);
        assert_eq!([1, 2, 3, 4, 4, 5], ar);
    }
    #[test]
    fn quicksort() {
        let mut ar = [5, 4, 3, 2, 1];
        sort::<_, quicksort::QuickSort>(&mut ar);
        assert_eq!([1, 2, 3, 4, 5], ar);
    }

    #[test]
    fn mergesort() {
        let mut ar = [2, 1];
        sort::<_, mergesort::MergeSort>(&mut ar);
        assert_eq!([1, 2], ar);
    }
}
