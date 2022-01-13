pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    bubble_sort_with_comparator(arr, |x: &T, y: &T| x.lt(y))
}

pub fn bubble_sort_with_comparator<T, F>(arr: &mut [T], is_less: F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    let mut ordered;

    for i in 0..arr.len() {
        ordered = true;
        for j in 0..arr.len() - i - 1 {
            if is_less(&arr[j + 1], &arr[j]) {
                arr.swap(j, j + 1);
                if cfg!(debug_assertions) {
                    println!("swap: arr[{}] <=> arr[{}]", i, j);
                }
                ordered = false;
            }
        }
        if ordered {
            if cfg!(debug_assertions) {
                println!("fast end optimized!");
            }
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending_nature() {
        // 降序排列
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
        println!("descending_nature, result: {:?}", ve1);
    }

    #[test]
    fn ascending_nature() {
        // 升序，预排序
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
        println!("ascending_nature, result: {:?}", ve2);
    }

    #[test]
    fn random_nature() {
        //升序，预排序
        let mut ve3 = vec![23, 32, 543, 23, 565, 34, 54];
        bubble_sort(&mut ve3);
        for i in 0..ve3.len() - 1 {
            assert!(ve3[i] <= ve3[i + 1]);
        }
        println!("random_nature, result: {:?}", ve3);
    }

    #[test]
    fn descending_comparator() {
        //降序排列
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort_with_comparator(&mut ve1, |x, y| x.gt(y));
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] >= ve1[i + 1]);
        }
        println!("descending_comparator, result: {:?}", ve1);
    }

    #[test]
    fn ascending_comparator() {
        //升序，预排序
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort_with_comparator(&mut ve2, |x, y| x.gt(y));
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] >= ve2[i + 1]);
        }
        println!("ascending_comparator, result: {:?}", ve2);
    }
}
