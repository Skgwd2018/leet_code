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
    // 由于nums是按值传递的，即在函数内部将获得nums的一个完全独立的副本。
    // 因此，对nums的任何修改都不会影响到原始的数据结构（除非通过某种方式返回这个修改后的副本）。
    // 如果函数并没有返回任何值，则原始数据保持不变。
    // 如果希望函数内部处理数据的副本，以避免对原始数据的直接修改，则使用mut版本的函数签名
    // (注意，由于是按值传递，实际上需要在函数内部修改并返回这个副本，否则修改将不会反映到原始数据上)。

    // &mut 表示传入的是一个可变的引用（mutable reference）到Vec<i32>。
    // 即函数内部可以直接修改原始数据结构，因为函数接收的是对原始数据的直接引用。
    // 由于是按引用传递，没有数据的复制发生，这使得函数操作更加高效，特别是当处理大型数据结构时。
    // 如果希望函数能够修改原始数据，并且不想因为数据复制而消耗额外的内存或性能，应该使用 &mut 函数签名。

    // 稳定性排序和非稳定性排序,主要区别在于排序过程中相等元素的相对位置是否保持不变。
    // 以下是两者的详细区别：
    // 稳定性排序:
    // 定义:稳定性排序算法在排序过程中，能够保持相等元素的原始相对位置不变。
    // 即，如果两个元素在排序前是相等的，并且它们在原数组中的先后顺序是A在B前面，那么在排序后的数组中，A仍然会出现在B的前面。
    // 特点:
    // 稳定性排序算法能够保留元素的原始顺序信息，这在某些特定应用场景下非常重要。
    // 常见的稳定性排序算法包括冒泡排序、插入排序、归并排序和基数排序等。
    // 非稳定性排序:
    // 定义:非稳定性排序算法在排序过程中，不保证相等元素的原始相对位置不变。
    // 即，如果两个元素在排序前是相等的，排序后它们的相对位置可能会发生变化。
    // 特点:
    // 非稳定性排序算法在排序过程中可能会改变相等元素的相对位置，这可能会在某些应用场景下导致问题。
    // 常见的非稳定性排序算法包括选择排序、快速排序、希尔排序和堆排序等。
    // 实际应用中的考虑:
    // 1.如果排序的内容是复杂对象的多个属性，且需要保持对象之间的原始顺序
    // (例如，先按价格排序，再按销量排序，同时希望相同销量的对象保持价格排序的顺序)，则应该选择稳定性排序算法。
    // 2.如果只是简单的数字排序，或者排序的内容不需要保持原始顺序，那么可以选择非稳定性排序算法，因为通常具有更好的性能。

    // 比较型排序算法
    // bubble_sort(&mut nums); // 冒泡排序(超时),稳定性排序
    // insert_sort(&mut nums); // 插入排序(1712ms, 2.74mb),稳定性排序
    // select_sort(&mut nums); // 选择排序(超时),非稳定性排序

    // 比较型排序算法(进阶版)
    merge_sort(&mut nums); // 归并排序(31ms, 2.76mb),时间复杂度:O(n log n),空间复杂度:O(n),稳定性排序
    // quick_sort(&mut nums); // 快速排序(40ms, 2.88mb),时间复杂度:O(n log n,可能退化到O(n^2)),空间复杂度:O(1),非稳定性排序
    // heap_sort(&mut nums); // 堆排序(53ms, 2.70mb),时间复杂度:O(n log n),空间复杂度:O(1),非稳定性排序

    // 非比较型整数排序算法
    // nums = counting_sort(nums); // 计数排序(25ms, 2.73mb),时间复杂度:O(n+k),空间复杂度:O(k),稳定性排序
    // radix_sort(&mut nums); // 基数排序(19ms, 3.22mb),时间复杂度:O(d*(n+k)),空间复杂度:O(n+k),稳定性排序

    // rust
    // nums.sort_unstable(); // 23ms, 2.72mb, 非稳定性排序
    // nums.sort(); // 34ms, 2.80mb, 稳定性排序

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

/// 2.插入排序,时间复杂度:O(n^2),空间复杂度:O(1),稳定性排序
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

/// 3.选择排序,时间复杂度:O(n^2),空间复杂度:O(1),非稳定性排序
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

/// 4.归并排序,时间复杂度:O(n log n),空间复杂度:O(n),稳定性排序,属于分治法(Divide and Conquer)
#[allow(unused)]
fn merge_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    merge(nums, 0, n);
}
fn merge(nums: &mut Vec<i32>, start: usize, end: usize) {
    // println!("1. nums:{:?}, start:{}, end:{}", nums, start, end);
    if start + 1 >= end { return; }

    let mid = (start + end) / 2;
    // println!("2. start:{}, mid:{}, end:{}", start, mid, end);
    merge(nums, start, mid);
    // println!("3. nums:{:?}, start:{}, mid:{}, end:{}\n", nums, start, mid, end);
    merge(nums, mid, end);
    // println!("4. nums: {:?}, start: {}, mid: {}, end: {}", nums, start, mid, end);
    // start:0, mid:1, end:2
    // start:3, mid:4, end:5
    // start:2, mid:3, end:5
    // start:0, mid:2, end:5
    // start:6, mid:7, end:8
    // start:5, mid:6, end:8
    // start:9, mid:10, end:11
    // start:8, mid:9, end:11
    // start:5, mid:8, end:11
    // start:0, mid:5, end:11
    sort(nums, start, mid, end);
}
fn sort(nums: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    let nums2 = nums[mid..end].to_vec();
    let (mut index1, mut index2, mut index) = (mid - 1, nums2.len() - 1, end - 1);

    // println!("nums2:{:?}", nums2);
    loop {
        // println!("sort. nums: {nums:?}, index1: {index1}, index2: {index2}, index: {index}");
        // println!("loop: {}, {}, {}", nums[index1], nums2[index2], nums[index]);
        if nums[index1] > nums2[index2] {
            nums[index] = nums[index1];
            index -= 1;
            if index1 == start {
                // for i in 0..=index2 { nums[i + start] = nums2[i]; } // 作用同下
                // 用于将一个切片(slice)的内容复制到另一个切片中,这个函数通常用于处理字节数据或者需要在两个切片之间复制数据
                // let src = [1, 2, 3, 4];
                // let mut dst = [0; 4]; // 初始化一个全为0的切片
                // dst.copy_from_slice(&src); // 将 src 的内容复制到 dst 中
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
    // println!("5. nums:{:?}\n", nums);
}

/// 5.快速排序,时间复杂度:O(n log n),空间复杂度:O(1),非稳定性排序
#[allow(unused)]
fn quick_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    quick(nums, 0, n);
}
fn quick(nums: &mut Vec<i32>, start: usize, end: usize) {
    if start + 1 >= end { return; }

    // 平均时间复杂度为O(n log n),最坏情况是时间复杂度退化到O(n^2),这通常发生在已经接近排序完成或完全未排序的情况下。
    // 通过随机选择基准值或使用三数取中等方法可以减小这种情况发生的概率。
    // 随机选取一个点
    let pivot = rand::rng().random_range(start..end); // 执行效率相比下面的更高
    // let pivot = (start + end) / 2; //执行效率较低
    nums.swap(start, pivot);
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

/// 6.堆排序,时间复杂度:O(n log n),空间复杂度:O(1),非稳定性排序
#[allow(unused)]
fn heap_sort(nums: &mut Vec<i32>) {
    let n = nums.len();

    // 1.建立大顶堆
    for i in (0..n / 2).rev() {
        // println!("1.nums:{:?}, index:{}", nums, i);
        heap_fy(nums, i, n);
    }

    // 2.此时nums[0]就是最大元素
    for i in (1..n).rev() {
        // println!("2.nums:{:?}", nums);
        nums.swap(0, i);
        // println!("2.nums:{:?}, index:{}", nums, i);
        heap_fy(nums, 0, i);
    }
}
fn heap_fy(nums: &mut Vec<i32>, mut index: usize, n: usize) {
    loop {
        let mut max_index = index;
        let left_index = index * 2 + 1;
        let right_index = left_index + 1;
        // println!("heap_fy. left_index:{}, right_index:{}, max_index:{}", left_index, right_index, max_index);

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

/// 7.计数排序,时间复杂度:O(n+k),空间复杂度:O(k),稳定性排序
// n是待排序数组的长度，k是数据的范围
// 注意:只能用于排序一定范围内的整数,且当数据范围k很大时,会消耗大量内存
#[allow(unused)]
fn counting_sort(mut nums: Vec<i32>) -> Vec<i32> {
    // let n = nums.len();
    let (mut min, mut max) = (i32::MAX, i32::MIN);
    for num in &nums {
        min = min.min(*num);
        max = max.max(*num);
    }
    let mut cnts = vec![0; (max - min + 1) as usize];
    for num in &nums {
        cnts[(*num - min) as usize] += 1;
    }
    nums.clear();
    cnts.into_iter().enumerate().for_each(|(index, cnt)| {
        for _ in 0..cnt {
            nums.push(i32::try_from(index).expect("i32 error") + min);
        }
    });

    nums
}

/// 8.基数排序,时间复杂度:O(d*(n+k)),空间复杂度:O(n+k),稳定性排序
// d 是数字的位数，n 是排序元素的总数，k 是基数(如对于十进制数，k=10)
// 通常情况下,基数排序的时间复杂度优于传统的比较排序算法(如快速排序、归并排序等)，特别是当待排序的数是多位数且分布均匀时。
// 然而,基数排序需要额外的空间来存储临时数据,且对于数据范围很大的情况可能不太适用。
#[allow(unused)]
fn radix_sort(nums: &mut Vec<i32>) {
    if nums.is_empty() { return; }

    const BASE: u32 = 256; // 基,大于等于2才有意义
    let num_min = nums.iter().min().copied().unwrap();
    // .wrapping_sub()是整数类型(如i32)上的一个方法，它执行减法操作，如果结果超出类型的表示范围，则它会“回绕”到范围的另一端。
    // 例如，在i32中，(-1).wrapping_sub(1)的结果是std::i32::MAX（即2^31 - 1），而不是引发溢出。
    // .wrapping_add()也是整数类型上的一个方法，它执行加法操作，并处理可能的溢出.
    // .log(BASE as f64):对f64类型的值调用 log() 方法,计算以其参数(即 BASE 转换为f64)为底的对数。
    // .ceil():对结果调用 ceil() 方法，是将浮点数向上舍入到最接近的整数
    // 目的计算以BASE为底，(最大值 - 最小值 + 1)的对数的向上取整值。可用于多种场景，例如:确定对数刻度上的刻度数量、计算需要的循环轮次 等。
    let n_round = (nums.iter().max().unwrap().wrapping_sub(num_min).wrapping_add(1) as u32 as f64)
        .log(BASE as f64).ceil() as u32;
    // println!("1. n_round:{n_round}");
    let mut counts = [0; BASE as usize];
    for i in 0..n_round {
        for num in nums.iter().copied() {
            counts[((num.wrapping_sub(num_min) as u32) / BASE.pow(i) % BASE) as usize] += 1;
            // println!("2. counts:{counts:?}");
        }
        for j in 1..BASE as usize {
            // 得到前缀和
            counts[j] += counts[j - 1];
            // println!("3. counts:{counts:?}");
        }
        let mut temp = vec![0; nums.len()];
        for num in nums.iter().rev().copied() {
            let v = ((num.wrapping_sub(num_min) as u32) / BASE.pow(i) % BASE) as usize;
            counts[v] -= 1;
            temp[counts[v]] = num;
            // println!("4. temp:{temp:?}");
        }
        nums.copy_from_slice(&temp);
        // println!("radix_sort: {nums:?}");
        // 对 counts 全部元素填充 0
        counts.fill(0);
    }
}
//-----------------------------------------------------