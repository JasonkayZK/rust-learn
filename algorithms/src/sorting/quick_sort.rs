/// Quick Sort
pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    quick_sort_with_comparator(arr, |x, y| x.lt(y))
}

pub fn quick_sort_with_comparator<T, F>(arr: &mut [T], is_less: F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }

    _quick_sort_with_comparator(arr, 0, len - 1, &is_less);
}

/**
 * 最基本的快速排序实现
 *
 * @param arr  待排序子数组
 *
 * @param lo   数组左边界
 *
 * @param hi   数组右边界
 *
 * @param is_less 比较函数
 */
fn _quick_sort_with_comparator<T, F>(arr: &mut [T], lo: usize, hi: usize, is_less: &F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    if lo < hi {
        let p = partition(arr, lo, hi, is_less);

        if p != 0 {
            _quick_sort_with_comparator(arr, lo, p - 1, is_less);
        }
        _quick_sort_with_comparator(arr, p + 1, hi, is_less);
    }
}

/**
 * 切分(交换法): 将数组a切分为arr[lo...j-1] < arr[j] < arr[j+1...hi]
 *
 * 快速排序的切分算法(快排核心)
 *
 * @param arr  待排序数组
 * @param lo   子数组左边界
 * @param hi   子数组右边界
 * @return     切分后的标准元素key的index
 */
fn partition<T, F>(arr: &mut [T], lo: usize, hi: usize, is_less: &F) -> usize
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    // 将数组切分为arr[lo...j-1], arr[j], arr[j+1...hi]
    let mut i = lo;
    let mut j = hi + 1;

    unsafe {
        // 内部同时涉及到数组中的不变借用和可变借用，需要使用unsafe
        // 选定开头为标准元素
        let key: *const T = &arr[lo];

        loop {
            // 左右移动, 跳过已经有序的位置
            i += 1;
            while is_less(&arr[i], &*key) {
                i += 1;
                if i == hi {
                    break;
                }
            }
            j -= 1;
            while is_less(&*key, &arr[j]) {
                j -= 1;
                if j == lo {
                    break;
                }
            }

            // 切分完毕, 跳过交换
            if i >= j {
                break;
            }

            // 找到乱序的, 交换
            arr.swap(i, j);
        }
    }
    // 将key放入对应位置
    arr.swap(lo, j);

    j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        quick_sort(&mut arr);
        println!("after sort: {:?}", arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        quick_sort(&mut arr);
        println!("after sort: {:?}", arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        quick_sort(&mut arr);
        println!("after sort: {:?}", arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        quick_sort(&mut arr);
        println!("after sort: {:?}", arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        quick_sort(&mut arr);
        println!("after sort: {:?}", arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        quick_sort(&mut arr);
        println!("after sort: {:?}", arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }
    }

    #[test]
    fn basic_reverse() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        quick_sort_with_comparator(&mut arr, |x, y| x.gt(y));
        println!("basic_reverse after sort: {:?}", arr);
        for i in 1..arr.len() {
            assert!(arr[i - 1] >= arr[i]);
        }
    }
}
