use std::cmp::Ordering;

/// 215.数组中的第k个最大元素(数组,分治,快速选择,排序,堆(优先队列))
/// 数组中的第k个最大元素(例:当 k=1(即最大的元素))
pub fn find_kth_largest(mut nums: Vec<i32>, k: usize) -> i32 {
    // 要找的是第 k 大的元素,即目标位置是排序后的数组长度减去 k
    let index = nums.len() - k;
    // select_nth_unstable() 从重新排序的切片中返回一个三元组:索引前的子切片的引用、索引处的元素的引用 和 索引后的子切片的引用。
    // 注:select_nth_unstable() 方法可能并不会保持原始数组的排序,它只是一个快速选择算法的实现,用于在未排序的数组中查找第 n 个最小元素。
    // 如果目的是查找第 k 大的元素,且不在乎算法是否保持排序。
    *nums.select_nth_unstable(index).1

    // 解法二: 快速选择(超时)
    // let n = nums.len();
    // quick_select(&mut nums, 0, n - 1, n - k)
}
// "快速选择"(QuickSelect)算法,它是快速排序算法的一个变种。
// 快速选择算法基于分治的思想,并不完全对数组进行排序.而是通过一次划分将待查找的元素定位到数组的某一部分,从而缩小查找范围。
#[allow(unused)]
fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
    if left == right { return nums[left]; }

    let pivot_index = partition(nums, left, right);
    match k.cmp(&pivot_index) {
        Ordering::Less => quick_select(nums, left, pivot_index - 1, k),
        Ordering::Equal => nums[k],
        Ordering::Greater => quick_select(nums, pivot_index + 1, right, k),
    }
}
#[allow(unused)]
fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut i = left;
    for j in left..right {
        if nums[j] <= pivot {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, right);

    i
}
//-----------------------------------------------------