use rand::Rng;

/// 1431.拥有最多糖果的孩子(数组)
// 通过遍历candies并比较每个孩子的糖果数量加上extra_candies之后是否大于或等于数组中的最大值。
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = *candies.iter().max().unwrap_or(&0) - extra_candies;

    // .iter():遍历向量中的每一个元素。迭代器是一个可以记住遍历的位置并在之后再次访问这些位置的对象。
    // .enumerate():这个方法附加在迭代器之后,它会改变迭代器产生的内容。
    // 原本迭代器只产生向量中的元素，但调用enumerate()后,迭代器现在产生的是元组(Tuple),
    // 每个元组包含两个元素:第一个是元素的索引(从0开始),第二个是元素的值。
    /*for (i, &candy) in candies.iter().enumerate() {
        if candy >= *max_candies {
            answer[i] = true;
        }
    }*/

    // .map(|&candy| candy >= max_candies)
    // 对迭代器中的每个元素(使用模式匹配|&candy|来借用每个candy的值,避免不必要的复制)应用一个函数。
    // 这个函数计算后会返回一个bool: true表示当前孩子的糖果加上额外的糖果后至少和最大的糖果数量一样多,false则表示不够。
    // .collect()方法调用,将map()步骤返回的迭代器中的所有布尔值收集到一个新的(Vec<bool>)中
    // candies.iter().map(|&candy| candy >= max_candies).collect()
    candies.into_iter().map(|candy| candy >= max_candies).collect()
}
//-----------------------------------------------------

/// 605.种花问题(数组,贪心)
// 题目要求:每朵花的旁边都不能种花，所以种花必须是间隔种1朵
// n:是否可以种的花数量
pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
    let len = flowerbed.len();
    let mut i = 0;
    while i < len {
        // 检查头尾&相邻项的问题
        if flowerbed[i] == 0 && (i == 0 || flowerbed[i - 1] == 0) && (i == len - 1 || flowerbed[i + 1] == 0) {
            n -= 1;
            /*if n == 0 {
                return true;
            }*/

            i += 2; // 下个位置肯定不能种花,可以跳过
        } else {
            i += 1;
        }
    }

    n <= 0
}
//-----------------------------------------------------

/// 238.除自身以外数组的乘积(数组,前缀和)
// 给你一个整数数组 nums，返回 数组 answer ，其中 answer[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积，且不能使用除法
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    /*let n = nums.len();
    let mut answer = vec![0; n];
    answer[n - 1] = 1;
    for i in (0..n - 1).rev() {
        answer[i] = answer[i + 1] * nums[i + 1];
    }
    let mut pre = 1;
    for (i, num) in nums.iter().enumerate() {
        // 此时 pre 为 nums[0] 到 nums[i-1] 的乘积，然后直接乘到 answer[i] 中
        answer[i] *= pre;
        pre *= num;
    }
    answer*/

    // 解法二:
    let (mut left, mut right, n) = (1, 1, nums.len());
    let mut answer = vec![1; n];
    for (i, &num) in nums.iter().enumerate() {
        answer[i] *= left;
        // print!("{} ", answer[i]);
        left *= num;
        // print!("{} ", left);
        answer[n - 1 - i] *= right;
        // print!("{} ", answer[n - 1 - i]);
        right *= nums[n - 1 - i];
        // println!("{} ", right);
    }

    answer
}
//-----------------------------------------------------

/// 334.递增的三元子序列(贪心,数组)
// 如果存在这样的三元组下标 (i, j, k) 且满足 i < j < k,使得 nums[i] < nums[j] < nums[k],返回 true;否则,返回 false
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let (mut first, mut second) = (i32::MAX, i32::MAX);
    for num in nums {
        // 贪心算法(Greed Algorithm)
        if num <= first {
            first = num;
        } else if num <= second {
            second = num;
        } else {
            return true;
        }
    }

    false
}
//-----------------------------------------------------

/// 1979.找出数组的最大公约数(数组,数学,数论)
// 给定一个整数数组 nums ，返回数组中最大数和最小数的 最大公约数。
// 两个数的 最大公约数 是能够被两个数整除的最大正整数。
// 输入：nums = [7,5,6,8,3]
// 输出：1
// 解释：
// nums 中最小的数是 3
// nums 中最大的数是 8
// 3 和 8 的最大公约数是 1
pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let (min, max) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
    (1..=min).rev().find(|i| min % i == 0 && max % i == 0).unwrap()
}
//-----------------------------------------------------

/// 1534.统计好三元组(数组,枚举)
// 给定一个整数数组arr，以及 a、b 、c 三个整数。请你统计其中好三元组的数量。
// 如果三元组 (arr[i], arr[j], arr[k]) 满足下列全部条件，则认为它是一个 好三元组。
// 0 <= i < j < k < arr.length
// |arr[i] - arr[j]| <= a
// |arr[j] - arr[k]| <= b
// |arr[i] - arr[k]| <= c
// 其中 |x| 表示 x 的绝对值。
// 返回 好三元组的数量。
// 输入：arr = [3,0,1,1,9,7], a = 7, b = 2, c = 3
// 输出：4
// 解释：一共有 4 个好三元组：[(3,0,1), (3,0,1), (3,1,1), (0,1,1)]
pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let mut ans = 0;
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if (arr[i] - arr[j]).abs() <= a {
                for k in j + 1..arr.len() {
                    if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}
//-----------------------------------------------------

/// 912.排序数组(桶排序,分治,数组,基数排序,计数排序,归并排序,堆(优先队列))
// 给定一个整数数组 nums，请你将该数组升序排列。
pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    // bubble_sort(&mut nums); // 冒泡排序
    // insert_sort(&mut nums); // 插入排序
    // select_sort(&mut nums); // 选择排序
    merge_sort(&mut nums); // 归并排序
    // quick_sort(&mut nums); // 快速排序
    // heap_sort(&mut nums); // 堆排序

    nums
}

/// 1.冒泡排序，时间复杂度：O(n^2)，空间复杂度：O(1)，稳定性排序
#[allow(unused)]
fn bubble_sort(nums: &mut Vec<i32>) {
    let n = nums.len();

    for i in (1..n).rev() {
        let mut flag = true;

        for j in 1..=i {
            if nums[j - 1] > nums[j] {
                nums.swap(j - 1, j);
                flag = false;
            }
        }

        if flag { break; }
    }
}

/// 2.插入排序，时间复杂度：O(n^2)，空间复杂度：O(1)，稳定性排序
#[allow(unused)]
fn insert_sort(nums: &mut Vec<i32>) {
    let n = nums.len();

    for i in 1..n {
        let insert_val = nums[i];

        for j in (0..i).rev() {
            if nums[j] > insert_val {
                nums[j + 1] = nums[j];
            } else {
                nums[j + 1] = insert_val;
                break;
            }

            if j == 0 {
                nums[0] = insert_val;
            }
        }
    }
}

/// 3.选择排序，时间复杂度：O(n^2)，空间复杂度：O(1)，非稳定性排序
#[allow(unused)]
fn select_sort(nums: &mut Vec<i32>) {
    let n = nums.len();

    for i in (1..n).rev() {
        let mut max_idx = i;

        for j in 0..i {
            if nums[j] > nums[max_idx] {
                max_idx = j;
            }
        }

        nums.swap(i, max_idx);
    }
}

/// 4.归并排序，时间复杂度：O(nlogn)，空间复杂度：O(n)，稳定性排序
#[allow(unused)]
fn merge_sort(nums: &mut Vec<i32>) {
    let n = nums.len();

    merge(nums, 0, n)
}
fn merge(nums: &mut Vec<i32>, beg: usize, end: usize) {
    if beg + 1 >= end { return; }

    let mid = (beg + end) / 2;
    merge(nums, beg, mid);
    merge(nums, mid, end);

    sort(nums, beg, mid, end)
}
fn sort(nums: &mut Vec<i32>, beg: usize, mid: usize, end: usize) {
    let nums2 = nums[mid..end].to_vec();
    let mut idx1 = mid - 1;
    let mut idx2 = nums2.len() - 1;
    let mut idx = end - 1;

    loop {
        if nums[idx1] > nums2[idx2] {
            nums[idx] = nums[idx1];
            idx -= 1;
            if idx1 != beg { idx1 -= 1; } else {
                for i in 0..=idx2 {
                    nums[i + beg] = nums2[i];
                }

                break;
            }
        } else {
            nums[idx] = nums2[idx2];
            idx -= 1;
            if idx2 != 0 { idx2 -= 1; } else { break; }
        }
    }
}

/// 5.快速排序，时间复杂度：O(nlogn)，空间复杂度：O(1)，非稳定性排序
#[allow(unused)]
fn quick_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    quick(nums, 0, n);
}
fn quick(nums: &mut Vec<i32>, beg: usize, end: usize) {
    if beg + 1 >= end { return; }

    // 相当于随机选取一个点
    let idx = rand::thread_rng().gen_range(beg..end);
    //let idx = (beg + end) / 2;
    nums.swap(beg, idx);
    let val = nums[beg];

    let mut idx = beg;
    let (mut l, mut r) = (beg + 1, end);
    while l < r {
        while l < r && nums[r - 1] > val { r -= 1; }
        if l < r {
            nums[idx] = nums[r - 1];
            idx = r - 1;
            r -= 1;
        }

        while l < r && nums[l] < val { l += 1; }
        if l < r {
            nums[idx] = nums[l];
            idx = l;
            l += 1;
        }
    }

    nums[idx] = val;
    quick(nums, beg, idx);
    quick(nums, idx + 1, end);
}

/// 6.堆排序，时间复杂度：O(nlogn)，空间复杂度：O(1)，非稳定性排序
#[allow(unused)]
fn heap_sort(nums: &mut Vec<i32>) {
    let n = nums.len();

    // 1.建立大顶堆
    for i in (0..n / 2).rev() {
        heap_fy(nums, i, n);
    }

    // 2.此时nums[0]就是最大元素
    for i in (1..n).rev() {
        nums.swap(0, i);
        heap_fy(nums, 0, i);
    }
}
fn heap_fy(nums: &mut Vec<i32>, mut idx: usize, n: usize) {
    loop {
        let mut max_idx = idx;
        let left_idx = idx * 2 + 1;
        let right_idx = left_idx + 1;

        if left_idx < n && nums[left_idx] > nums[max_idx] {
            max_idx = left_idx;
        }
        if right_idx < n && nums[right_idx] > nums[max_idx] {
            max_idx = right_idx;
        }
        if max_idx == idx { break; }

        nums.swap(idx, max_idx);
        idx = max_idx;
    }
}
//-----------------------------------------------------