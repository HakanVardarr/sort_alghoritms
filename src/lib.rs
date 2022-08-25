mod bubblesort;

trait Sort {
    fn sort<T>(ar: &mut [T])
    where
        T: Ord;
}

#[allow(dead_code)]
fn sort<T, S>(ar: &mut [T])
where
    T: Ord,
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
}
