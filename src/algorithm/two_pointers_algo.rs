use std::cmp::Ordering;

// 双指针(Two Pointers)
// 双指针模式是使用两个指针来遍历数组或列表,常用于查找满足特定条件的数对或元素。在处理有序数组或列表,且需要查找满足特定条件的数对时,可使用该模式。
// 示例问题:在一个有序数组中找出两个数,使其和等于目标值。
// 示例：输入: nums = [1, 2, 3, 4, 6], target = 6 输出: [1, 3]
// 解释: 初始化两个指针,一个指向数组开头(左指针),一个指向数组末尾(右指针)。
//      检查两个指针所指元素的和。如果和等于目标值,返回这两个元素的索引。如果和小于目标值,将左指针向右移动。如果和大于目标值,将右指针向左移动。
pub fn two_pointer_ex(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Equal => return Some((left, right)),
            Ordering::Greater => right -= 1,
        }
    }

    None
}
//-----------------------------------------------------

/// 283.移动零(数组,双指针)
pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 双指针操作(快指针&慢指针)
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            // 交换两个索引位置的元素
            // nums.swap(i, j);
            nums[j] = nums[i];
            if i != j {
                nums[i] = 0;
            }

            j += 1;
        }
    }
    println!("{nums:?}");
}
//-----------------------------------------------------

/// 392.判断子序列(字符串,双指针,动态规划)
// 给定字符串 s 和 t,判断 s 是否为 t 的子序列
pub fn is_subsequence(s: &str, t: &str) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars = t.chars().collect::<Vec<char>>();
    let (s_len, t_len) = (s_chars.len(), t_chars.len());

    // 双指针操作
    let (mut s_index, mut t_index) = (0, 0);
    while t_index < t_len {
        if s_index < s_len && s_chars[s_index] == t_chars[t_index] {
            s_index += 1;
        }

        t_index += 1;
    }

    s_index == s_len
}
//-----------------------------------------------------

/// 11.盛最多水的容器(数组,双指针,贪心)
pub fn max_area(height: &[i32]) -> i32 {
    // 双指针操作
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max_area = 0;
    while left < right {
        // let curr_area = cmp::min(height[left], height[right]) * (right - left) as i32;
        // max_area = cmp::max(curr_area, max_area);
        // let curr_area = i32::min(height[left], height[right]) * (right - left) as i32;
        // max_area = i32::max(max_area, curr_area);
        // 这两种操作方式通常会内联调用且性能非常接近，建议使用下面的方式(易写且易读)
        let curr_area = height[left].min(height[right]) * i32::try_from(right - left).unwrap_or_default();
        max_area = max_area.max(curr_area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}
//-----------------------------------------------------

/// 1679.K和数对的最大数目(数组,哈希表,双指针,排序)
// 整数数组 nums 和整数 k,每一步操作中,需要从数组中选出和为 k 的两个整数,并将它们移出数组。返回可以对数组执行的最大操作数。
// nums = [3, 1, 3, 4, 3], k = 6
pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
    //解法一:哈希表
    /*let mut answer = 0;
    let mut cnt_map = HashMap::new();
    for x in nums {
        if let Some(c) = cnt_map.get_mut(&(k - x)) {
            if *c > 0 {
                *c -= 1;
                answer += 1;
                continue;
            }
        }
        *cnt_map.entry(x).or_insert(0) += 1;
    }
    answer*/

    // 解法二:排序,双指针
    nums.sort_unstable();
    let (mut left, mut right) = (0, nums.len() - 1);
    let mut answer = 0;
    while left < right {
        match (nums[left] + nums[right]).cmp(&k) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => {
                answer += 1;
                left += 1;
                right -= 1;
            }
        }
    }

    answer
}
//-----------------------------------------------------