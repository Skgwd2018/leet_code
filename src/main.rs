use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use leet_code::{ListNode, RecentCounter, TreeNode};

fn main() {
    println!("------ 交替合并字符串 ------");
    let word1 = String::from("abcde");
    let word2 = String::from("xyz");
    let result = merge_alternately(word1, word2);
    println!("merge_alternately: {result}"); // axbyczde

    println!("------ 字符串的最大公因子 ------");
    let str1 = String::from("ABABAB");
    let str2 = String::from("AB");
    let result = gcd_of_strings(str1, str2);
    println!("gcd_of_strings: {result}"); // AB

    println!("------ 拥有最多糖果的孩子 ------");
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let result = kids_with_candies(candies, extra_candies);
    println!("kids_with_candies: {:?}", result); // [true, true, true, false, true]

    println!("------ 种花问题 ------");
    // let flowerbed = vec![1, 0, 0, 0, 0, 1];
    let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
    // let flowerbed = vec![0, 1, 0];
    let n = 3;
    let result = can_place_flowers(flowerbed, n);
    println!("can_place_flowers {n}: {}", result); // true

    println!("------ 反转字符串中的元音字母 ------");
    let s = "leetcode".to_string();
    // let s = "hello".to_string();
    let result = reverse_vowels(s);
    println!("reverse_vowels: {}", result); // leotcede

    //----------------------------

    println!("------ 移动零 ------");
    let mut nums = vec![0, 1, 0, 3, 12];
    // let mut nums = vec![4, 1, 5, 3, 12];
    move_zeroes(&mut nums); // [1, 3, 12, 0, 0]

    println!("------ 判断子序列 ------");
    let s = "ace";
    let t = "abcde";
    println!("Is '{}' a sub of '{}'? {}", s, t, is_subsequence(s.to_string(), t.to_string())); // true

    //----------------------------

    println!("------ 子数组最大平均数 ------");
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let result = find_max_average(nums, k);
    println!("find_max_average: {result}"); // 12.75

    //----------------------------

    println!("------ 找到最高海拔 ------");
    let gain = vec![-5, 1, 5, 0, -7];
    // let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    let result = largest_altitude(gain);
    println!("largest_altitude: {result}"); // 1

    println!("------ 寻找数组的中心下标 ------");
    let nums = vec![1, 7, 3, 6, 5, 6];
    let result = pivot_index(nums);
    println!("pivot_index: {result}"); // 3

    //----------------------------

    println!("------ 找出两数组的不同 ------");
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 2, 1, 2, 4];
    let result = find_difference(nums1, nums2);
    println!("find_difference: {result:?}"); // [[3], [4]]

    println!("------ 独一无二的出现次数 ------");
    let arr = vec![1, 2, 2, 1, 1, 3];
    let result = unique_occurrences(arr);
    println!("unique_occurrences: {result}"); // true

    //----------------------------

    println!("------ 最近的请求次数(头尾高效操作的队列) ------");
    let mut obj = RecentCounter::new();
    let ret_1 = obj.ping(1);
    println!("ping: {ret_1}");        // 1
    let ret_2 = obj.ping(100);
    println!("ping: {ret_2}");        // 2
    let ret_3 = obj.ping(3001);
    println!("ping: {ret_3}");        // 3
    let ret_4 = obj.ping(3002);
    println!("ping: {ret_4}");        // 3

    //----------------------------

    println!("------ 反转链表 ------");
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(2);
    let mut node3 = ListNode::new(3);
    let mut node4 = ListNode::new(4);
    let node5 = ListNode::new(5);
    node4.set_next(Some(Box::new(node5)));
    node3.set_next(Some(Box::new(node4)));
    node2.set_next(Some(Box::new(node3)));
    node1.set_next(Some(Box::new(node2)));
    node1.print_list(); // 1 2 3 4 5
    println!("------ rev ------");
    let node_rev = ListNode::reverse_list(Some(Box::new(node1.clone())));
    match node_rev.clone() {
        None => {}
        Some(node) => {
            node.print_list(); // 5 4 3 2 1
        }
    }

    //----------------------------

    println!("------ 二叉树的最大深度 ------");
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    if let Some(left_node) = root.borrow_mut().right.as_mut() {
        left_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(17))));
    }
    root.borrow_mut().right.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(36))));
    // 作用同上,前提是TreeNode要实现 Clone trait,不建议使用这种方式操作(阅读困难)
    let mut rt = TreeNode::new(4);
    let left = TreeNode::new(9);
    let mut right = TreeNode::new(22);
    let right_left = TreeNode::new(17);
    let right_right = TreeNode::new(36);
    rt.left = Some(Rc::new(RefCell::new(left)));
    right.left = Some(Rc::new(RefCell::new(right_left)));
    right.right = Some(Rc::new(RefCell::new(right_right)));
    rt.right = Some(Rc::new(RefCell::new(right.clone())));

    let result = TreeNode::max_depth(Some(root.clone()));
    println!("max_depth: {result}"); // 3

    println!("------ 叶子相似的树 ------");
    let result = TreeNode::leaf_similar(Some(Rc::new(RefCell::new(rt))), Some(root.clone()));
    println!("leaf_similar: {result}"); // true

    println!("------ 二叉搜索树(BST)中的搜索 ------");
    let val = 20;
    let result = TreeNode::search_bst(Some(root.clone()), val);
    println!("search_bst: {result:?}"); // Some(RefCell { value: TreeNode { val: 20, left: Some(RefCell { value: TreeNode { val: 17, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 36, left: None, right: None } }) } })

    //----------------------------

    println!("------ 猜数字大小 ------");
    let n = 10;
    let pick_num = guess_number(n);
    println!("guessNumber: {pick_num}"); // 7

    //----------------------------

    println!("------ 第N个泰波那契数 ------");
    let n = 25;
    let result = tribonacci(n);
    println!("tribonacci({n}): {result}"); // 1389537

    println!("------ 使用最小花费爬楼梯 ------");
    // let cost = vec![10, 15, 20]; // 15
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]; // 6
    let result = min_cost_climbing_stairs(cost);
    println!("min_cost_climbing_stairs: {result}"); // 6

    //----------------------------

    println!("------ 比特位计数 ------");
    let n = 5;
    let result = count_bits(n);
    println!("count_bits({n}): {result:?}"); // [0, 1, 1, 2, 1, 2]

    println!("------ 只出现一次的数字 ------");
    let nums = vec![4, 1, 2, 1, 2];
    let result = single_number(nums);
    println!("single_number: {result}"); // 4

    println!("\n----------------------------\n");

    println!("------ 反转字符串中的单词 ------");
    let s = "  a good   example ".to_string();
    let result = reverse_words(s);
    println!("reverse_words: {result}"); // example good a

    println!("------ 压缩字符串 ------");
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    // let mut chars = vec!['a'];
    // let mut chars = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
    let result = compress(&mut chars);
    println!("compress: {result}"); // 6  ['a', '2', 'b', '2', 'c', '3']

    println!("------ 盛最多水的容器 ------");
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    // let height = vec![1, 1];
    let max_area = max_area(height);
    println!("Max water: {max_area}"); // 49

    println!("------ 定长子串中元音的最大数目 ------");
    let s = "abciiidef".to_string();
    let k = 3;
    let result = max_vowels(s, k);
    println!("max_vowels: {result}"); // 3

    println!("------ 确定两个字符串是否接近 ------");
    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    let result = close_strings(word1, word2);
    println!("close_strings: {result}"); // true

    println!("------ 从字符串中移除星号 ------");
    let s = "leet**cod*e".to_string(); // lecoe
    // let s = String::from("erase*****"); // ""
    let result = remove_stars(s);
    println!("remove_stars: {result}");

    println!("------ 字符串解码 ------");
    let s = "3[a12[c]]".to_string();  // accaccacc
    // let s = "3[a]2[bc]".to_string(); // aaabcbc
    let result = decode_string(s);
    println!("decode_string: {result}");

    println!("------ 删除链表的中间节点 ------");
    let node_head = ListNode::delete_middle(node_rev);
    match node_head.clone() {
        None => {}
        Some(node) => node.print_list(),
    }

    println!("----- 奇偶链表 ------");
    let odd_even_head = ListNode::odd_even_list(node_head);
    match odd_even_head {
        None => {}
        Some(node) => node.print_list(),
    }

    println!("----- 统计二叉树中好节点的数目 ------");
    let result = TreeNode::good_nodes(Some(root));
    println!("good_nodes: {result}");

    println!("----- 二叉树路径总和Ⅲ ------");
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let result = TreeNode::path_sum(root, 8);
    println!("path_sum: {result}");

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

fn can_divide(s1: &str, s2: &str) -> bool {
    // .chunks_exact(s2.len()) 将 s1 的字节切片分割成长度为 s2.len() 的块。
    // 如果 s1 的长度不是 s2.len() 的整数倍，这个函数会抛出一个 panic。但由于s1.len() % s2.len() == 0，所以这里不会有问题。
    // .all(|chunk| chunk == s2.as_bytes()) 对所有分割出的块执行检查每个块是否都与 s2 的字节切片相等。
    // 如果所有块都相等，那么 s1 是由 s2 重复构成的，函数返回 true,否则返回 false。
    s1.len() % s2.len() == 0 && s1.as_bytes().chunks_exact(s2.len()).all(|chunk| chunk == s2.as_bytes())
}

/// 字符串的最大公因子
fn gcd_of_strings(str1: String, str2: String) -> String {
    // 题目要求:字符串中的字符全是字母
    let len1 = str1.len();
    let len2 = str2.len();

    // 求两个字符串长度的最大公约数
    let gec_len = (1..=cmp::min(len1, len2)).rev()
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

// 通过遍历candies并比较每个孩子的糖果数量加上extra_candies之后是否大于或等于数组中的最大值。
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = *candies.iter().max().unwrap_or(&0);

    // .iter(): 遍历向量中的每一个元素。迭代器是一个可以记住遍历的位置并在之后再次访问这些位置的对象。
    // .enumerate(): 这个方法附加在迭代器之后，它会改变迭代器产生的内容。
    // 原本迭代器只产生向量中的元素，但调用enumerate()后，迭代器现在产生的是元组(Tuple),
    // 每个元组包含两个元素: 第一个是元素的索引(从0开始)，第二个是元素的值。
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

// 题目要求:每朵花的旁边都不能种花，所以种花必须是间隔种1朵
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

/// 是否是元音字母
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
    // vec![set1.difference(&set2).cloned().collect(), set2.difference(&set1).map(|&x| x).collect()]

    // .copied() 是一个用于复制迭代器中原始元素值的适配器。.copied()的作用和上面的.map(|&x| x)是一样的
    // 它通常用于原始元素是 Copy trait 的实现者的情况，这意味着这些元素可以通过简单的位复制来复制，而不是通过调用 clone 方法。
    // 这通常比 clone() 克隆更高效，因为位复制通常比调用 clone 方法更快。
    // .cloned() 用于克隆实现了 Clone trait 的元素。
    // .copied() 用于复制实现了 Copy trait 的元素，这通常比克隆更高效。
    // 在选择使用哪个适配器时:
    // 如果元素是 Copy 的，那么使用 .copied() 通常是更好的选择。
    // 如果元素不是 Copy 的但实现了 Clone，那么你应该使用 .cloned()。
    vec![set1.difference(&set2).copied().collect(), set2.difference(&set1).copied().collect()]
}
//-----------------------------------------------------

/// 使用哈希集合解决统计出现次数问题
fn unique_occurrences(arr: Vec<i32>) -> bool {
    // 存储每个数的出现次数的集合
    let mut count_map = HashMap::new();
    /*for num in arr {
        // 使用 entry() 方法检查键 num 是否已经存在于 count_map 中,
        // 如果键 num 不存在，or_insert(0) 会将键 num 插入到哈希映射中，并设置其对应的值为 0。
        // 如果遇到相同的键时，就可以在其现有值的基础上增加计数。
        *count_map.entry(num).or_insert(0) += 1;
    }*/
    // for_each() 方法，用于遍历迭代器中的每个元素，并对每个元素执行一个给定的操作。在这里，它遍历 arr 中的每个 num
    arr.into_iter().for_each(|num| *count_map.entry(num).or_insert(0) += 1);

    // 存储出现次数的集合
    let mut occurrences = HashSet::new();
    /* for count in count_map.values() {
        // 将值添加到HashSet集合中。
        // 返回值:是否是新插入的。即:如果集合以前不包含此值，则返回true;如果集合已经包含此值，则返回false，并且不修改集合:不替换原始值,并删除作为参数传递的值
        if !occurrences.insert(*count) {
            return false;
        }
    }
    true */
    // 上面的方式性能更高,因为只获取 值集合，而下面的方式是获取 键值集合(entry集合即条目集合)需要解构操作
    // all() 方法，用于检查迭代器中的所有元素是否都满足给定的条件.检查 count_map 中的每个出现次数（即值）是否都是唯一的。
    // 如果所有出现次数都成功插入到 occurrences 中，那么 all() 方法将返回 true。
    // 如果有任何出现次数已经存在于 occurrences 中，all() 方法将立刻返回 false。
    count_map.into_iter().all(|entry| occurrences.insert(entry.1))
}
//-----------------------------------------------------

/// 猜数字大小(二分法查找问题)
// 题目要求:数字范围是[1,n]
fn guess_number(n: i32) -> i32 {
    let (mut low, mut result) = (1, n);
    while low < result {
        let mid = low + (result - low) / 2;
        match guess(mid) {
            1 => low = mid + 1,
            0 => {
                result = mid;
                break;
            }
            _ => result = mid,
        }
    }

    result
}

// 题目提供的
fn guess(num: i32) -> i32 {
    // 这里应该是调用实际的猜数字接口的逻辑，但在这只是模拟一下
    // 假设选中的数字是某个固定的值，比如7
    match num.cmp(&7) {
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Greater => -1,
        cmp::Ordering::Less => 1,
    }
}
//-----------------------------------------------------

/// 使用动态规划避免重复计算
/// 泰波那契序列 Tn 定义如下：
/// T0 = 0, T1 = 1, T2 = 1, 且在 n >= 0 的条件下 Tn+3 = Tn + Tn+1 + Tn+2
/// 给你整数 n，请返回第 n 个泰波那契数 Tn 的值。
fn tribonacci(n: i32) -> i32 {
    // 递归方式:
    /*match n {
        0 => 0,
        1 | 2 => 1,
        _ => tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3),
    }*/

    // 动态规划（Dynamic Programming, DP）来避免重复计算。
    // 动态规划是一种算法设计技术，用于解决具有重叠子问题和最优子结构特性的问题。
    // 对于泰波那契数列，你可以使用动态规划来存储已经计算过的值，从而避免重复计算。
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            (2..n).fold((0, 1, 1), |(p1, p2, p3), _| (p2, p3, p1 + p2 + p3)).2
            /*let (mut p1, mut p2, mut p3) = (0, 1, 1);
            for _ in 2..n {
                (p1, p2, p3) = (p2, p3, p1 + p2 + p3)
            }
            p3*/
        }
    }
}
//-----------------------------------------------------

/// 动态规划
fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let (p1, p2) = (2..cost.len()).fold((cost[0], cost[1]), |(c1, c2), i| (c2, c1.min(c2) + cost[i]));
    p1.min(p2)
}
//-----------------------------------------------------

/// 位运算
// 输入：n = 5
// 输出：[0,1,1,2,1,2]
// 解释：
// 0 --> 0
// 1 --> 1
// 2 --> 10
// 3 --> 11
// 4 --> 100
// 5 --> 101
fn count_bits(n: i32) -> Vec<i32> {
    // count_ones() 方法:计算其二进制表示中1的个数
    // count_ones()函数是std::u32::TrailingZeroBits trait的一部分，适用于所有(有/无符号)整数类型(包括u8 ~ u128 & i8 ~ i128)
    (0..=n).map(|x| x.count_ones() as i32).collect()
}
//-----------------------------------------------------

// 题目：给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
// 假设数组中重复的元素为x，只出现一次的元素为y。
// 将数组中的所有元素进行异或运算，由于x出现了两次，所以x和x异或的结果为0，而y只出现了一次，所以最后的结果就是y。
/// 异或（XOR）运算问题。异或运算有一个重要的性质：任何数和0异或都等于它本身，任何数和其自身异或都等于0。
fn single_number(nums: Vec<i32>) -> i32 {
    /*let mut single = 0;
    nums.iter().for_each(|num| single ^= num);
    single*/

    // reduce() 和 fold() 在功能上是相似的，但它们的初始值和参数的顺序是不同的。
    // 两者都可以用来累积一个值，通过对集合中的元素应用某种操作。
    // 对于 reduce() 方法，它接受一个二元操作函数，并将集合中的元素两两组合起来，直到只剩下一个元素。
    // reduce() 不需要初始值，因为它使用集合中的第一个元素作为初始值。
    // 注:reduce() 使用数组中的第一个元素作为初始值进行异或操作;如果数组为空，reduce 会返回一个 None，需要使用 unwrap() 来获取结果，这可能会导致运行时错误(如果数组为空)。
    // 在这种情况下，reduce运行更高效
    // nums.into_iter().reduce(|a, b| a ^ b).unwrap()

    nums.iter().fold(0, |single, num| single ^ num)
}
//-----------------------------------------------------

// 给你一个字符串 s ，请你反转字符串中 单词 的顺序。
// 单词 是由非空格字符组成的字符串。s 中使用至少一个空格将字符串中的 单词 分隔开。
// 返回 单词 顺序颠倒且 单词 之间用单个空格连接的结果字符串。
// 题目要求:输入的字符串s中可能会存在前导空格、尾随空格或者单词间的多个空格。返回的结果字符串中，单词间应当仅用单个空格分隔，且不包含任何额外的空格。
fn reverse_words(s: String) -> String {
    /* let mut words: VecDeque<String> = VecDeque::new();
    let mut curr_word = String::new();
    for c in s.trim().chars() {
        if c.is_whitespace() {
            if !curr_word.is_empty() {
                words.push_front(curr_word);
                curr_word = String::new();
            }
        } else {
            curr_word.push(c);
        }
    }
    if !curr_word.is_empty() {
        words.push_front(curr_word);
    }
    words.into_iter().collect::<Vec<_>>().join(" ") */

    // .split_ascii_whitespace(): 将字符串s按照ASCII空白字符（如空格、制表符、换行符等）进行分割，返回一个迭代器，其中每个元素都是原始字符串中的一个单词
    // .rev(): 将迭代器中所有元素的顺序反转
    // ::<T> 是Rust中用于指定泛型参数或返回类型的语法，也被称为类型提示。此处用于告诉 collect() 方法要收集元素到一个 Vec<&str> 类型的向量中
    // .collect::<Vec<&str>>(): 将反转后的迭代器元素收集到一个新的向量(Vec)中并指定了它的返回类型。且每个元素都是一个指向原始字符串中单词的切片（&str）
    // .join(" "): 将向量(Vec)中的所有切片用空格连接起来，形成一个新的字符串
    s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
//-----------------------------------------------------

// 题目要求:chars不为空
/// 压缩字符串
fn compress(chars: &mut Vec<char>) -> i32 {
    let n = chars.len();
    if n == 1 {
        return 1;
    }

    let mut idx = 0;
    let mut count = 1;
    for i in 1..n {
        if chars[i - 1] == chars[i] {
            count += 1;
        } else {
            chars[idx] = chars[i - 1];
            idx += 1;
            if count > 1 {
                for c in count.to_string().chars() {
                    chars[idx] = c;
                    idx += 1;
                }
            }
            count = 1;
        }
    }

    chars[idx] = chars[n - 1];
    idx += 1;
    if count > 1 {
        for c in count.to_string().chars() {
            chars[idx] = c;
            idx += 1;
        }
    }

    idx as i32
}
//-----------------------------------------------------

fn max_area(height: Vec<i32>) -> i32 {
    // 双指针操作
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area = 0;
    while left < right {
        // let curr_area = cmp::min(height[left], height[right]) * (right - left) as i32;
        // max_area = cmp::max(curr_area, max_area);
        // 这两种操作方式通常会内联调用且性能非常接近，建议使用下面的方式(易读)
        let curr_area = height[left].min(height[right]) * (right - left) as i32;
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

// 题目要求:s 由小写英文字母组成且非空
fn max_vowels(s: String, k: i32) -> i32 {
    let k = k as usize;
    if s.len() < k { return 0; }
    // let s = s.chars().collect::<Vec<char>>();
    let s = s.as_bytes(); // 操作byte效率更高
    let is_vowel = |x| {
        match x {
            b'a' | b'e' | b'i' | b'o' | b'u' => 1,
            _ => 0
        }
    };

    let mut r = k;
    // 计算第一个区间元音数
    let mut vowels = (s[..k]).iter().map(|&x| is_vowel(x)).sum::<i32>();
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

fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() { return false; }

    let mut word1_cnt = [0; 26];
    // 显式循环: 编译器可能更容易对其进行优化，因为它直接反映了循环的意图，没有额外的抽象层。
    /*for c in word1.as_bytes() {
        word1_cnt[(c - b'a') as usize] += 1;
    }*/
    // 迭代器链: 利用了 Rust 的迭代器抽象，使得代码更加函数式。
    // 虽然迭代器链提供了更多的灵活性(即可以很容易地添加额外的操作到链中)，但也可能引入一些微小的运行时开销，因为每次调用迭代器方法时都可能涉及到一些额外的函数调用。
    // 在某些情况下，编译器可能不如处理显式循环那样优化迭代器链。
    // 总结：迭代器链方式的内存消耗会相对较少，显式循环的运行会较快，但差异非常微小
    word1.as_bytes().iter().for_each(|c| word1_cnt[(c - b'a') as usize] += 1);
    let mut word2_cnt = [0; 26];
    word2.as_bytes().iter().for_each(|c| word2_cnt[(c - b'a') as usize] += 1);

    for i in 0..26 {
        if (word1_cnt[i] == 0) != (word2_cnt[i] == 0) {
            return false;
        }
    }

    // sort: 对切片进行稳定的排序，即如果两个元素相等，它们在排序后的相对顺序会保持不变。
    // 由于它保证了稳定性，所以通常比 sort_unstable 慢一些，因为它需要额外的内存来保持元素的相对顺序。
    // sort_unstable: 对切片进行排序，如果两个元素相等，它们在排序后的相对顺序可能会改变。
    // 由于它不需要保证稳定性，所以通常比 sort 快一些，因为它可以采用更高效的排序算法。
    word1_cnt.sort_unstable();
    word2_cnt.sort_unstable();
    word1_cnt == word2_cnt
}
//-----------------------------------------------------

// 给你一个包含若干星号 * 的字符串 s 。
// 在一步操作中，你可以：选中 s 中的一个星号。
// 移除星号 左侧 最近的那个 非星号 字符，并移除该星号自身。返回移除 所有 星号之后的字符串。
fn remove_stars(s: String) -> String {
    // 栈操作
    let mut stack = Vec::new();
    /*for c in s.chars() {
        match c {
            '*' => if stack.pop().is_some() {},
            _ => stack.push(c),
        }
    }
    stack.iter().collect::<String>()*/
    // 操作byte效率更高,且简单场合中if else 的运行效率比 match 高
    for &b in s.as_bytes() {
        if b == b'*' {
            stack.pop().unwrap();
        } else {
            stack.push(b);
        }
    }
    String::from_utf8(stack).unwrap()
}
//-----------------------------------------------------

// 题目要求:原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入
//         s 中所有整数的取值范围为 [1, 300]
fn decode_string(s: String) -> String {
    // 栈操作
    let mut stack = Vec::new();
    let mut curr_num = 0;
    let mut curr_str = String::new();
    // let (mut curr_num, mut curr_str) = (0, String::new()); // 内存比较(无区别)

    // "3[a12[c]]" ----> "accaccacc"
    for c in s.chars() {
        match c {
            '0'..='9' => {
                // curr_num = curr_num * 10 + (c as u8 - '0' as u8) as usize; // '0' as u8 是 48
                curr_num = curr_num * 10 + (c as usize - '0' as usize);
            }
            '[' => {
                stack.push((curr_str, curr_num));
                curr_str = String::new();
                // 这个操作创建了一个新的空字符串，并将curr_str的引用更新为指向这个新字符串。
                // 这涉及到内存分配（尽管分配的是一个非常小的字符串），并且如果之前的String是堆上分配的，那么它的内存也会被回收。
                // 这种方式的优点是它确保了curr_str不再保留任何不必要的内存，但缺点是涉及到内存分配和可能的垃圾回收，这通常比简单的标记为空要慢一些。

                // curr_str.clear();
                // 这个操作仅仅将字符串的内部缓冲区标记为空，它并不会释放分配的内存。
                // 即如果字符串之前占用了大量内存，那么即使调用clear之后，该内存仍然被String保留。
                // 这样做的优点是操作很快，因为不涉及任何内存分配或释放。
                // 如果之后String又需要存储数据，它可以在已经分配的内存上进行操作，这通常比重新分配内存要快。
                curr_num = 0;
            }
            ']' => {
                if let Some((prev_str, count)) = stack.pop() {
                    let repeated_str = curr_str.repeat(count); // 重复curr_str字符串count次
                    curr_str = prev_str + &repeated_str;
                }
            }
            _ => curr_str.push(c),
        }
    }

    curr_str
}
//-----------------------------------------------------