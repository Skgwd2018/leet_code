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
            // if n == 0 { return true; }

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
// 给定一个整数数组 nums，将该数组升序排列。
pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    // mut 关键字用于函数参数nums上, 允许函数内部对nums进行可变操作
    // 由于nums是按值传递的，即在函数内部你将获得nums的一个完全独立的副本。
    // 因此，对nums的任何修改都不会影响到原始的数据结构（除非通过某种方式返回这个修改后的副本）。
    // 如果函数并没有返回任何值，则原始数据保持不变。
    // 如果希望函数内部处理数据的副本，以避免对原始数据的直接修改，则使用mut版本的函数签名
    // (注意，由于是按值传递，实际上需要在函数内部修改并返回这个副本，否则修改将不会反映到原始数据上)。

    // &mut 表示传入的是一个可变的引用（mutable reference）到Vec<i32>。
    // 即函数内部可以直接修改原始数据结构，因为函数接收的是对原始数据的直接引用。
    // 由于是按引用传递，没有数据的复制发生，这使得函数操作更加高效，特别是当处理大型数据结构时。
    // 如果希望函数能够修改原始数据，并且不想因为数据复制而消耗额外的内存或性能，应该使用 &mut 函数签名。

    // bubble_sort(&mut nums); // 冒泡排序(超时)
    // insert_sort(&mut nums); // 插入排序(1712ms, 2.74mb)
    // select_sort(&mut nums); // 选择排序(超时)
    merge_sort(&mut nums); // 归并排序(31ms, 2.76mb)
    // quick_sort(&mut nums); // 快速排序(40ms, 2.88mb)
    // heap_sort(&mut nums); // 堆排序(53ms, 2.70mb)

    nums
}

/// 1.冒泡排序,时间复杂度:O(n^2),空间复杂度:O(1),稳定性排序
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

            if j == 0 { nums[0] = insert_val; }
        }
    }
}

/// 3.选择排序，时间复杂度：O(n^2)，空间复杂度：O(1)，非稳定性排序
#[allow(unused)]
fn select_sort(nums: &mut Vec<i32>) {
    let n = nums.len();

    for i in (1..n).rev() {
        let mut max_index = i;

        for j in 0..i {
            if nums[j] > nums[max_index] {
                max_index = j;
            }
        }

        nums.swap(i, max_index);
    }
}

/// 4.归并排序，时间复杂度：O(nlogn),空间复杂度：O(n),稳定性排序
#[allow(unused)]
fn merge_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    merge(nums, 0, n);
}
fn merge(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start + 1 >= end { return; }

    let mid = (start + end) / 2;
    merge(nums, start, mid);
    merge(nums, mid, end);
    sort(nums, start, mid, end);
}
fn sort(nums: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    let nums2 = nums[mid..end].to_vec();
    let (mut index1, mut index2, mut index) = (mid - 1, nums2.len() - 1, end - 1);

    loop {
        if nums[index1] > nums2[index2] {
            nums[index] = nums[index1];
            index -= 1;
            if index1 == start {
                // for i in 0..=index2 { nums[i + start] = nums2[i]; } // 作用同下
                // 用于将一个切片（slice）的内容复制到另一个切片中。这个函数通常用于处理字节数据或者需要在两个切片之间复制数据时
                // let src = [1, 2, 3, 4];
                // let mut dst = [0; 4]; // 初始化一个全为0的切片
                // dst.copy_from_slice(&src); // 将src的内容复制到dst中
                // println!("{:?}", dst); // 输出: [1, 2, 3, 4]
                nums[start..=(index2 + start)].copy_from_slice(&nums2[..=index2]);
                break;
            }

            index1 -= 1;
        } else {
            nums[index] = nums2[index2];
            index -= 1;
            if index2 == 0 { break; }
            index2 -= 1;
        }
    }
}

/// 5.快速排序，时间复杂度：O(nlogn)，空间复杂度：O(1)，非稳定性排序
#[allow(unused)]
fn quick_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    quick(nums, 0, n);
}
fn quick(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start + 1 >= end { return; }

    // 相当于随机选取一个点
    let index = rand::thread_rng().gen_range(start..end);
    //let index = (start + end) / 2;
    nums.swap(start, index);
    let val = nums[start];

    let mut index = start;
    let (mut l, mut r) = (start + 1, end);
    while l < r {
        while l < r && nums[r - 1] > val {
            r -= 1;
        }
        if l < r {
            nums[index] = nums[r - 1];
            index = r - 1;
            r -= 1;
        }

        while l < r && nums[l] < val {
            l += 1;
        }
        if l < r {
            nums[index] = nums[l];
            index = l;
            l += 1;
        }
    }

    nums[index] = val;
    quick(nums, start, index);
    quick(nums, index + 1, end);
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
fn heap_fy(nums: &mut Vec<i32>, mut index: usize, n: usize) {
    loop {
        let mut max_index = index;
        let left_index = index * 2 + 1;
        let right_index = left_index + 1;

        if left_index < n && nums[left_index] > nums[max_index] {
            max_index = left_index;
        }
        if right_index < n && nums[right_index] > nums[max_index] {
            max_index = right_index;
        }
        if max_index == index {
            break;
        }

        nums.swap(index, max_index);
        index = max_index;
    }
}
//-----------------------------------------------------