use std::cmp;
use std::collections::HashSet;

fn main() {
    println!("------ 交替合并字符串 ------");
    let word1 = String::from("abcde");
    let word2 = String::from("xyz");
    let result = merge_alternately(word1, word2);
    println!("merge_alternately: {result}");

    println!("------ 字符串的最大公因子 ------");
    let str1 = String::from("ABABAB");
    let str2 = String::from("AB");
    let result = gcd_of_strings(str1, str2);
    println!("gcd_of_strings: {result}");

    println!("------ 拥有最多糖果的孩子 ------");
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let result = kids_with_candies(candies, extra_candies);
    println!("kids_with_candies: {:?}", result);

    println!("------ 种花问题 ------");
    // let flowerbed = vec![1, 0, 0, 0, 0, 1];
    let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
    // let flowerbed = vec![0, 1, 0];
    let n = 2;
    let result = can_place_flowers(flowerbed, n);
    println!("can_place_flowers {n}: {}", result);

    println!("------ 反转字符串中的元音字母 ------");
    let s = "leetcode".to_string();
    // let s = "hello".to_string();
    let result = reverse_vowels(s);
    println!("reverse_vowels: {}", result);

    println!("------ 移动零 ------");
    let mut nums = vec![0, 1, 0, 3, 12];
    // let mut nums = vec![4, 1, 5, 3, 12];
    move_zeroes(&mut nums);

    println!("------ 判断子序列 ------");
    let s = "ace";
    let t = "abcde";
    println!("Is '{}' a sub of '{}'? {}", s, t, is_subsequence(s.to_string(), t.to_string()));

    println!("------ 子数组最大平均数 ------");
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let result = find_max_average(nums, k);
    println!("find_max_average: {result}");

    println!("------ 找到最高海拔 ------");
    let gain = vec![-5, 1, 5, 0, -7];
    // let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    let result = largest_altitude(gain);
    println!("largest_altitude: {result}");

    println!("------ 寻找数组的中心下标 ------");
    let nums = vec![1, 7, 3, 6, 5, 6];
    let result = pivot_index(nums);
    println!("pivot_index: {result}");

    println!("------ 找出两数组的不同 ------");
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 2, 1, 2, 4];
    let result = find_difference(nums1, nums2);
    println!("find_difference: {result:?}");
}

/// 交替合并字符串
fn merge_alternately(word1: String, word2: String) -> String {
    // let len1 = word.len();
    // let len2 = word.chars().count();
    // word.len(): 这个方法直接返回字符串中字节的数量。在Rust中，String是一个UTF-8编码的字符串，所以len()方法返回的是字节的数量。
    // 如果字符串只包含ASCII字符，那么字节和字符的数量是相同的。
    // 但是，如果字符串包含非ASCII字符（如中文字符或其他Unicode字符），一个字符可能由多个字节表示。因此，len()返回的可能不是字符的实际数量。
    // word.len()的执行效率非常高，因为它只需要读取字符串的内部长度字段，不需要遍历整个字符串。
    //
    // word.chars().count(): 这个方法首先将字符串转换为一个字符迭代器，然后计算迭代器的长度。
    // 即是它需要遍历整个字符串来计算字符的数量。因此，它的执行效率通常比len()低，特别是当字符串很长时。
    // 如果需要知道字符串中字符的实际数量，无论它们是否由多少个字节表示，那么word.chars().count()才是正确的方法，尽管它的执行效率相对较低。

    let len1 = word1.chars().count();
    let len2 = word2.chars().count();
    let mut result = String::with_capacity(len1 + len2);

    // 使用zip()将两个等长的Vec组合成一个Vec，其中元素是一个元组，包含原来两个Vec(向量)中对应位置的元素。
    for (ch1, ch2) in word1.chars().zip(word2.chars()) {
        result.push(ch1);
        result.push(ch2);
    }

    // .iter().skip(n):从迭代器中跳过前 n 个元素
    for ch in word1.chars().skip(len2) {
        result.push(ch);
    }

    for ch in word2.chars().skip(len1) {
        result.push(ch);
    }

    result
}
//-----------------------------------------------------

// 字符串的最大公因子
fn can_divide(s1: &str, s2: &str) -> bool {
    // .chunks_exact(s2.len()) 将 s1 的字节切片分割成长度为 s2.len() 的块。
    // 如果 s1 的长度不是 s2.len() 的整数倍，这个函数会抛出一个 panic。但由于s1.len() % s2.len() == 0，所以这里不会有问题。
    // .all(|chunk| chunk == s2.as_bytes()) 对所有分割出的块执行检查每个块是否都与 s2 的字节切片相等。
    // 如果所有块都相等，那么 s1 是由 s2 重复构成的，函数返回 true,否则返回 false。
    s1.len() % s2.len() == 0 && s1.as_bytes().chunks_exact(s2.len()).all(|chunk| chunk == s2.as_bytes())
}

/// 字符串的最大公因子
fn gcd_of_strings(str1: String, str2: String) -> String {
    let len1 = str1.len();
    let len2 = str2.len();

    // 求两个字符串长度的最大公约数
    let gec_len = (1..cmp::min(len1, len2) + 1).rev()
        .find(|&i| len1 % i == 0 && len2 % i == 0).unwrap_or_else(|| cmp::min(len1, len2));

    // let cd1 = can_divide(&str1, &str1[0..gec_len]);
    // let cd2 = can_divide(&str2, &str1[0..gec_len]);
    // println!("cd1: {cd1}, cd2: {cd2}");

    if can_divide(&str1, &str1[0..gec_len]) && can_divide(&str2, &str1[0..gec_len]) {
        return str1[0..gec_len].to_string();
    }

    "".to_string()
}
//-----------------------------------------------------

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    // 通过遍历candies并比较每个孩子的糖果数量加上extra_candies之后是否大于或等于数组中的最大值。
    let max_candies = *candies.iter().max().unwrap_or(&0);

    // .iter(): 遍历向量中的每一个元素。迭代器是一个可以记住遍历的位置并在之后再次访问这些位置的对象。
    // .enumerate(): 这个方法附加在迭代器之后，它会改变迭代器产生的内容。
    // 原本迭代器只产生向量中的元素，但调用enumerate()后，迭代器现在产生的是元组(Tuple),
    // 每个元组包含两个元素：第一个是元素的索引(从0开始)，第二个是元素的值。
    /*for (i, &candy) in candies.iter().enumerate() {
        if candy + extra_candies >= *max_candies {
            result[i] = true;
        }
    }*/

    // .map(|&candy| candy + extra_candies >= max_candies)
    // 对迭代器中的每个元素(使用模式匹配|&candy|来借用每个candy的值，避免不必要的复制)应用一个函数。
    // 这个函数计算后会返回一个bool，true表示当前孩子的糖果加上额外的糖果后至少和最大的糖果数量一样多，false则表示不够。
    // .collect()方法调用，将map步骤返回的迭代器中的所有布尔值收集到一个新的(Vec<bool>)中
    candies.iter().map(|&candy| candy + extra_candies >= max_candies).collect()
}
//-----------------------------------------------------

// n: 是否可以种的花数量
fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let len = flowerbed.len();
    let mut n = n;
    let mut i = 0;

    while i < len {
        // 检查头尾&相邻项的问题
        if flowerbed[i] == 0 && (i == 0 || flowerbed[i - 1] == 0) && (i == len - 1 || flowerbed[i + 1] == 0) {
            n -= 1;
            /*if n == 0 {
                return true;
            }*/

            i += 2; // 下个位置肯定不能种花，可以跳过
        } else {
            i += 1;
        }
    }

    n <= 0
}
//-----------------------------------------------------

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn reverse_vowels(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    // 双指针操作
    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        if !is_vowel(chars[i]) {
            i += 1;
            continue;
        }

        if !is_vowel(chars[j]) {
            j -= 1;
            continue;
        }

        // 交换位置
        chars.swap(i, j);
        i += 1;
        j -= 1;
    }

    chars.iter().collect()
}
//-----------------------------------------------------

fn move_zeroes(nums: &mut Vec<i32>) {
    // 双指针操作(快指针&慢指针)
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            // 交换两个索引位置的元素
            nums.swap(i, j);
            /* nums[j] = nums[i];
            if i != j {
                nums[i] = 0;
            } */

            j += 1;
        }
    }

    println!("{:?}", nums);
}
//-----------------------------------------------------

/// 给定字符串 s 和 t,判断 s 是否为 t 的子序列
fn is_subsequence(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let s_len = s_chars.len();
    let t_len = t_chars.len();

    // 双指针操作
    let mut s_index = 0;
    let mut t_index = 0;

    while t_index < t_len {
        if s_index < s_len && s_chars[s_index] == t_chars[t_index] {
            s_index += 1;
        }

        t_index += 1;
    }

    s_index == s_len
}
//-----------------------------------------------------

/// 找出平均数最大且长度为 k 的连续子数组
fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    if nums.len() < k {
        panic!("Array length is less than k");
    }

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

/// 使用归约操作解决前缀和问题
fn largest_altitude(gain: Vec<i32>) -> i32 {
    /*let mut max = 0;
    let mut sum = 0;
    for g in gain {
        sum += g;
        if sum > max {
            max = sum;
        }
    }
    max*/

    // fold() 用于归约操作(将集合中的所有元素组合成一个单一的值)
    // fold() 方法接受一个初始值和一个闭包(或函数)，该闭包定义了如何将集合中的每个元素与累积器(accumulator)的值结合起来。
    // 闭包有两个参数：第一个是累积器的当前值，第二个是集合中的当前元素。
    // let numbers = vec![1, 2, 3, 4, 5];
    // let sum = numbers.iter().fold(0, |accumulator, &number| accumulator + number); // 15
    gain.iter().fold((0, 0), |(highest, curr), g| (highest.max(curr + g), curr + g)).0
}
//-----------------------------------------------------

fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = nums.iter().sum();
    for (i, num) in nums.iter().enumerate() {
        sum -= num;
        if sum == 0 {
            return i as i32;
        }
        sum -= num;
    }

    -1
}
//-----------------------------------------------------

/// 使用哈希集合解决去重复问题
fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    // let set1: HashSet<&i32> = nums1.iter().collect();
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let set2: HashSet<i32> = nums2.into_iter().collect();

    // let unique_in_nums1: Vec<i32> = nums1.into_iter().filter(|&x| !set2.contains(&x)).collect();
    // let unique_in_nums1: Vec<i32> = nums1.iter().filter(|x| !set2.contains(x)).cloned().collect();
    // let unique_in_nums1: Vec<i32> = unique_in_nums1.into_iter().collect::<HashSet<_>>().into_iter().collect();

    // 调用了 HashSet 的 difference() 方法，它返回一个迭代器，其中包含所有在 set1 中但不在 set2 中的元素。
    // set1.difference(&set2).map(|&x| x).collect(); // 这种方式会更高效
    // set1.difference(&set2).cloned().collect();    // 作用同上
    // .cloned() 是一个适配器，它会对迭代器中的每个元素调用 clone() 方法。
    // 使用 map() 方法来克隆每个元素。map() 方法接受一个闭包，该闭包对迭代器中的每个元素进行转换。
    // 在这里，闭包 |&x| x 实际上并没有改变元素，因为它只是借用并返回了元素本身。
    // 因此，这里的 map 操作实际上并没有做额外的工作，它只是简单地返回了元素的引用
    vec![set1.difference(&set2).cloned().collect(), set2.difference(&set1).map(|&x| x).collect()]
}
//-----------------------------------------------------