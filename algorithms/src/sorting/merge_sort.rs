/**
 * 归并排序
 *
 * 归并排序的核心是: 归并操作
 *
 * 归并操作将两个有序的数组归并为更大的一个有序数组
 *
 * 要将一个数组排序, 可以先(递归的)将它分为两半分别排序, 然后将结果归并起来
 *
 * 平均时间: O(NlgN)
 *
 * 最坏时间: O(6NlgN) 此时数组为完全树
 *
 * 最好时间: O(N) 数组元素全部相同
 *
 * 空间: O(N) 开辟了一个和排序数组相同大小的数组用于归并
 *
 * @author zk
 */
pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    merge_sort_with_comparator(arr, |x, y| x.lt(y))
}

pub fn merge_sort_with_comparator<T, F>(arr: &mut [T], is_less: F)
where
    T: PartialOrd + Copy,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    if len > 1 {
        _merge_sort_with_comparator(arr, 0, len - 1, &is_less)
    }
}

/**
 * 自顶向下的归并排序(lo...hi)区间
 *
 * @param arr  待排序数组
 *
 * @param lo   排序左边界
 *
 * @param hi   排序右边界(包括)
 *
 * @param is_less 比较函数
 */
pub fn _merge_sort_with_comparator<T, F>(arr: &mut [T], lo: usize, hi: usize, is_less: &F)
where
    T: PartialOrd + Copy,
    F: Fn(&T, &T) -> bool,
{
    if lo < hi {
        let mid = lo + (hi - lo) / 2;
        _merge_sort_with_comparator(arr, lo, mid, is_less);
        _merge_sort_with_comparator(arr, mid + 1, hi, is_less);
        _merge(arr, lo, mid, hi, is_less);
    }
}

/**
 * 原地归并的方法
 * <p>
 * 将子数组a[lo...mid]和a[mid+1...hi]归并成一个有序的数组
 * <p>
 * 并将结果放在a[lo...hi]中
 *
 * @param arr 待归并数组
 *
 * @param lo  左边界
 *
 * @param mid 中间
 *
 * @param hi  右边界
 */
pub fn _merge<T, F>(arr: &mut [T], lo: usize, mid: usize, hi: usize, is_less: &F)
where
    T: PartialOrd + Copy,
    F: Fn(&T, &T) -> bool,
{
    // create temporary arrays to support merge
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();
    for v in arr.iter().take(mid + 1).skip(lo) {
        left_half.push(*v);
    }
    for v in arr.iter().take(hi + 1).skip(mid + 1) {
        right_half.push(*v);
    }

    let lsize = left_half.len();
    let rsize = right_half.len();

    // pointers to track the positions while merging
    let mut l = 0;
    let mut r = 0;
    let mut a = lo;

    // pick smaller element one by one from either left or right half
    while l < lsize && r < rsize {
        if is_less(&left_half[l], &right_half[r]) {
            arr[a] = left_half[l];
            l += 1;
        } else {
            arr[a] = right_half[r];
            r += 1;
        }
        a += 1;
    }

    // put all the remaining ones
    while l < lsize {
        arr[a] = left_half[l];
        l += 1;
        a += 1;
    }

    while r < rsize {
        arr[a] = right_half[r];
        r += 1;
        a += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp() {
        let arr = vec![10, 8, 4, 3, 1, 9, 2, 7, 5];
        let lo = 0;
        let hi = arr.len() - 1;
        let mid = lo + (hi - lo) / 2;

        // create temporary arrays to support merge
        let mut left_half = Vec::new();
        let mut right_half = Vec::new();
        for v in arr.iter().take(mid + 1).skip(lo) {
            left_half.push(v);
        }
        for v in arr.iter().take(hi + 1).skip(mid + 1) {
            right_half.push(v);
        }

        println!("left: {:?}", left_half);
        println!("right: {:?}", right_half);

        let arr2 = vec![10, 8, 4, 3, 1, 9, 2, 7, 5];
        let (left, right) = arr2.split_at(mid + 1);
        println!("left: {:?}", left);
        println!("right: {:?}", right);
    }

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        merge_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
