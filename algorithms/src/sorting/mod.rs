pub mod bubble_sort;
pub mod quick_sort;
pub mod insertion_sort;
pub mod selection_sort;

pub fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    is_sorted_with_comparator(arr, |x, y| x.lt(y))
}

pub fn is_sorted_with_comparator<T, F>(arr: &[T], is_less: F) -> bool
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        if is_less(&arr[i], &arr[i-1]) {
            return false;
        }
    }
    true
}
