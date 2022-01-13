/// selection sort
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    selection_sort_with_comparator(arr, |x: &T, y: &T| x.lt(y))
}

pub fn selection_sort_with_comparator<T, F>(arr: &mut [T], is_less: F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    for i in 0..len {
        let mut min = i;

        for j in (i + 1)..len {
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
        selection_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        selection_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        selection_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        selection_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        selection_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        selection_sort(&mut arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn basic_reverse() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        selection_sort_with_comparator(&mut arr, |x, y| x.gt(y));
        for i in 1..arr.len() {
            assert!(arr[i - 1] >= arr[i]);
        }
    }
}
