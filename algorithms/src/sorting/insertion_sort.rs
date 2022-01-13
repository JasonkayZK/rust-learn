/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    insertion_sort_with_comparator(arr, |x: &T, y: &T| x.lt(y))
}

pub fn insertion_sort_with_comparator<T, F>(arr: &mut [T], is_less: F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && is_less(&arr[j], &arr[j - 1]) {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::{is_sorted, is_sorted_with_comparator};

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        insertion_sort(&mut arr);
        println!("after sort: {:?}", arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort(&mut arr);
        println!("after sort: {:?}", arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        println!("after sort: {:?}", arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn random_elements() {
        let mut arr: Vec<usize> = vec![43, 22, 333, 1];
        insertion_sort(&mut arr);
        println!("after sort: {:?}", arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn random_elements_with_comparator() {
        let mut arr: Vec<usize> = vec![43, 22, 333, 1, 333, 2134, 0, 2];
        let comp = |x: &usize, y: &usize| y.lt(x);

        insertion_sort_with_comparator(&mut arr, comp);
        println!("after sort: {:?}", arr);
        assert!(is_sorted_with_comparator(&arr, comp));
    }
}
