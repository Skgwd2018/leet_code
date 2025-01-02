use std::collections::Bound::{Included, Unbounded};
use std::collections::BTreeSet;

// 滑动窗口(Sliding Window)
// 滑动窗口模式用于查找满足特定条件的子数组或子字符串，通过维护一个元素窗口来优化时间复杂度。在处理涉及连续子数组或子字符串的问题时，可使用该模式。
// 示例问题：找出大小为k的子数组的最大和。
// 示例：输入：nums = [2, 1, 5, 1, 3, 2]，k = 3 输出：9
// 解释：先计算前k个元素的和。每次将窗口向右滑动一个元素，减去移出窗口的元素，再加上新进入窗口的元素。记录遇到的最大和。
pub fn sliding_window_ex(nums: &[i32], k: usize) -> i32 {
    assert!(nums.len() >= k, "Array length is less than k");

    // 初始化窗口和:计算前k个元素的和
    let mut window_sum = nums.iter().take(k).sum();
    // 初始化最大和
    let mut max_sum = window_sum;
    // 滑动窗口操作，更新 窗口和(window_sum) 以及 最大和(max_sum)
    for i in k..nums.len() {
        window_sum += nums[i] - nums[i - k];
        max_sum = i32::max(max_sum, window_sum);
    }

    max_sum
}
//-----------------------------------------------------

/// 643.子数组最大平均数Ⅰ(数组,滑动窗口)
/// 找出平均数最大值且长度为 k 的连续子数组
pub fn find_max_average(nums: Vec<i32>, k: usize) -> f64 {
    // 推荐使用 assert! 比 panic! 更好
    // if nums.len() < k { panic!("Array length is less than k"); }
    assert!(nums.len() >= k, "Array length is less than k");

    // 滑动窗口计算总和
    let mut window_sum: i32 = nums.iter().take(k).sum();
    let mut max_sum = window_sum;
    for (&num_in, &num_out) in nums.iter().skip(k).zip(nums.iter()) {
        window_sum += num_in - num_out;
        max_sum = i32::max(max_sum, window_sum); // 返回较大值
    }

    max_sum as f64 / k as f64
}
//-----------------------------------------------------

/// 1456.定长子串中元音的最大数目(字符串,滑动窗口)
// 题目要求:s 由小写英文字母组成且非空
pub fn max_vowels(s: String, k: usize) -> i32 {
    // let k = k as usize;
    if s.len() < k { return 0; }
    // let s = s.chars().collect::<Vec<char>>();
    let s = s.as_bytes(); // 操作byte效率更高
    // 闭包
    let is_vowel = |x| {
        match x {
            b'a' | b'e' | b'i' | b'o' | b'u' => 1,
            _ => 0
        }
    };

    let mut r = k;
    // 计算第一个区间元音数
    let mut vowels = s[..k].iter().map(|&x| is_vowel(x)).sum::<i32>();
    let mut max_vowels = vowels;
    while r < s.len() {
        // 滑动窗口操作
        vowels += is_vowel(s[r]);
        vowels -= is_vowel(s[r - k]);
        max_vowels = max_vowels.max(vowels);
        r += 1;
    }

    max_vowels
}
//-----------------------------------------------------

/// 1004.最大连续1的个数 III(数组,双指针,前缀和,滑动窗口)
// 给定二进制数组 nums 和整数 k,如果可以翻转最多 k 个 0 ,则返回 数组中连续 1 的最大个数
// nums = [0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], K = 3
pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    // 双指针
    let (mut answer, mut left, mut cnt) = (0, 0, 0);
    for (right, &num) in nums.iter().enumerate() {
        cnt += 1 - num;
        while cnt > k {
            cnt -= 1 - nums[left];
            left += 1;
        }
        answer = answer.max(right - left + 1);
    }

    // answer as i32
    i32::try_from(answer).unwrap_or_default()
}
//-----------------------------------------------------

/// 1493.删掉一个元素以后全为 1 的最长子数组(数组,滑动窗口,动态规划)
// 给定一个二进制数组 nums,需要从中删掉一个元素。请在删掉元素的结果数组中,返回最长的且只包含 1 的非空子数组的长度。
// 如果不存在这样的子数组,请返回 0
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    /*let (mut zero_cnt, mut max_len, mut n) = (0, 0, 0);
    for (i, &num) in nums.iter().enumerate() {
        if num == 0 { zero_cnt += 1; }

        while zero_cnt > 1 {
            if nums[n] == 0 { zero_cnt -= 1; }
            n += 1;
        }

        max_len = max_len.max(i - n);
    }
    max_len as i32*/

    //解法二:
    nums.into_iter().fold((-1, 0, 0), |(prev, curr, answer), n| {
        match n {
            1 => (prev, curr + 1, answer.max(curr + prev + 1)),
            0 => (curr, 0, answer.max(curr + prev)),
            _ => panic!()
        }
    }).2
}
//-----------------------------------------------------

/// LCR 057.存在重复元素 III(数组,桶排序,有序集合,滑动窗口)
// 给定一个整数数组 nums 和两个整数 k 和 t
// 请判断是否存在 两个不同下标 i 和 j,使得 abs(nums[i] - nums[j]) <= t ,同时又满足 abs(i - j) <= k
// 如果存在则返回 true,不存在返回 false
// 输入: nums = [1, 5, 9, 1, 5, 9], k = 2, t = 3
// 输出: false
pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let k = k as usize;
    let mut v: BTreeSet<i32> = BTreeSet::new();
    for i in 0..nums.len() {
        let iter = v.range((Included(nums[i].max(i32::MIN + t) - t), Unbounded)).next();
        if let Some(&res) = iter {
            if res <= (nums[i].min(i32::MAX - t) + t) {
                return true;
            }
        }
        v.insert(nums[i]);
        if i >= k {
            v.remove(&nums[i - k]);
        }
    }

    false
}
//-----------------------------------------------------