use std::cmp::Ordering;

/// 374.猜数字大小(二分查找,交互)
// 题目要求:数字范围是[1, n]
pub fn guess_number(n: i32) -> i32 {
    let (mut low, mut answer) = (1, n);
    while low < answer {
        let mid = low + (answer - low) / 2; // 计算中间位
        match guess(mid) {
            1 => low = mid + 1,
            0 => {
                answer = mid;
                break;
            }
            _ => answer = mid,
        }
    }

    answer
}

// 题目提供的
fn guess(num: i32) -> i32 {
    // 这里应该是调用实际的猜数字接口的逻辑,但只是模拟一下,假设选中的数字是某个固定的值,比如7
    match num.cmp(&7) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    }
}
//-----------------------------------------------------

// 变形二分查找(Modified Binary Search)
// 变形二分查找模式对二分查找进行改进，以解决更广泛的问题，例:在旋转排序数组中查找元素。在处理涉及排序或旋转数组，且需要查找特定元素的问题时，可使用该模式。
// 示例问题：在一个旋转排序数组中查找元素。示例：输入：nums = [4, 5, 6, 7, 0, 1, 2]，target = 0 输出：4
// 解释：进行二分查找时，额外增加一个判断，以确定数组的哪一半是有序的。然后检查目标元素是否在有序的那一半范围内。如果在，就在这一半中查找；否则，在另一半中查找。
pub fn search_rotate_array(nums: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right {
        // 根据左右指针计算中间索引mid
        let mid = left + (right - left) / 2;
        // 如果中间元素等于目标值，则返回其索引。
        if nums[mid] == target {
            return Some(mid);
        }

        // 判断哪一半是有序的
        if nums[left] <= nums[mid] {
            // 左半部分有序
            if nums[left] <= target && target <= nums[mid] {
                // 目标值在左半部分范围内
                right = mid - 1;
            } else {
                // 目标值在右半部分或者不存在
                left = mid + 1;
            }
        } else {
            // 右半部分有序
            if nums[mid] < target && target <= nums[right] {
                // 目标值在右半部分范围内
                left = mid + 1;
            } else {
                // 目标值在左半部分或者不存在
                right = mid - 1;
            }
        }
    }

    None
}
//-----------------------------------------------------

/// 2300.咒语和药水的成功对数(数组,双指针,二分查找)
pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable(); // 默认升序排列
    let n = potions.len();
    // partition_point() 内部使用 binary_search_by 进行查找
    // potions.partition_point() 返回符合条件的元素数量
    // let v = [1, 2, 3, 3, 5, 6, 7];
    // let i = v.partition_point(|&x| x < 5);  // 4, 注:目前只提供 < 操作
    // let i = v.partition_point(|&x| 5 < x);  // 不支持使用?
    spells.iter().map(|&x| (n - potions.partition_point(|&p| (x as i64) * (p as i64) < success)) as i32).collect()
}
//-----------------------------------------------------

/// 162.寻找峰值元素的索引(数组,二分查找)
// 峰值元素是指其值严格大于左右相邻值的元素。
// 给你一个整数数组 nums,找到峰值元素并返回其索引。数组可能包含多个峰值,在这种情况下,返回 任何一个峰值 所在位置即可。
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    // 双指针
    let (mut left, mut right) = (0, nums.len() - 1);
    // 二分搜索
    while left < right {
        // (right - left >> 1) 将这个宽度右移一位,相当于将宽度除以2(在二进制中,右移一位等同于除以2的整数部分).
        // 这里的作用是找到搜索范围的中间点,同时避免了整数除法运算,从而提高了效率。
        let middle = left + ((right - left) >> 1); // 计算中间索引
        match nums[middle] > nums[middle + 1] {
            true => right = middle,
            false => left = middle + 1,
        }
    }
    // 当left和right相等,此时就找到了峰值元素的索引。
    i32::try_from(left).unwrap_or_default()

    // 解法二:
    // max_by_key() 返回指定函数中给出最大值的元素。如果多个元素的最大值相等,则返回最后一个元素。如果迭代器为空,则返回None。
    // max_by_key(|(_, &v)| v) 元组的第一个元素（即索引）,并返回元组的第二个元素（即值）的引用。
    // nums.iter().enumerate().max_by_key(|(_, &v)| v).unwrap().0 as i32
}
//-----------------------------------------------------

/// 875.爱吃香蕉的珂珂(数组,二分查找)
// 这里有 n 堆香蕉,第 i 堆中有 piles[i] 根香蕉。警卫已经离开了,将在 h 小时后回来。
// 珂珂可以决定她吃香蕉的速度 k (单位:根/小时)。每个小时,她将会选择一堆香蕉,从中吃掉 k 根。
// 如果这堆香蕉少于 k 根,她将吃掉这堆的所有香蕉,然后这一小时内不会再吃更多的香蕉。
// 珂珂喜欢慢慢吃,但仍然想在警卫回来前吃掉所有的香蕉。
// 返回她可以在 h 小时内吃掉所有香蕉的最小速度 k(整数)
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    /*let check = |k: i32| -> bool {
        let mut sum = piles.len() as i32;
        for &p in &piles {
            sum += (p - 1) / k;
            if sum > h {
                return false;
            }
        }
        true
    };

    let mut left = 0;
    let mut right = *piles.iter().max().unwrap();
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    right*/

    // 解法二:
    // let mut right = *piles.iter().max().unwrap(); 计算这个二分法的上界
    let (mut left, mut right) = (1, 1_000_000_000); // 假设一个二分法的上界
    while left < right {
        // 二分中间数
        let mid = left + ((right - left) >> 1);
        // p + mid - 1 是为了这个整数除法向上取整
        let total: i32 = piles.iter().map(|p| (p + mid - 1) / mid).sum();
        if total > h {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
//-----------------------------------------------------