/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    insertion_sort_with_comparator(arr, |x: &T, y: &T| x.lt(y))
}

pub fn insertion_sort_with_comparator<T, F>(arr: &mut [T], is_less: F)
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    for i in 0..arr.len() {
        let mut min = i;

        for j in (i + 1)..arr.len() {
            if is_less(&arr[j], &arr[min]) {
                min = j;
            }
        }
        if min != i {
            arr.swap(min, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        insertion_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn basic_reverse() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        insertion_sort_with_comparator(&mut arr, |x, y| x.gt(y));
        for i in 1..arr.len() {
            assert!(arr[i - 1] >= arr[i]);
        }
    }
}
