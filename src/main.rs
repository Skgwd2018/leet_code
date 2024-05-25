use std::cell::RefCell;
use std::cmp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use leet_code::{ListNode, RecentCounter, SmallestInfiniteSet, TreeNode, Trie};

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

    println!("----- 删除二叉搜索树中的节点 ------");
    let result = TreeNode::delete_node(Some(root.clone()), val);
    println!("delete_node: {result:?}");

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
    println!("decode_string: {result}"); // accccccccccccaccccccccccccacccccccccccc

    println!("------ 删除链表的中间节点 ------");
    let node_head = ListNode::delete_middle(node_rev);
    match node_head.clone() {
        None => {}
        Some(node) => node.print_list(),
    } // 5 4 2 1

    println!("----- 奇偶链表 ------");
    let odd_even_head = ListNode::odd_even_list(node_head);
    match odd_even_head {
        None => {}
        Some(node) => node.print_list(),
    } // 5 2 4 1

    println!("----- 统计二叉树中好节点的数目 ------");
    let result = TreeNode::good_nodes(Some(root));
    println!("good_nodes: {result}"); // 4

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
    let result = TreeNode::path_sum(root.clone(), 8);
    println!("path_sum: {result}"); // 3

    println!("----- 二叉树的右视图 ------");
    let result = TreeNode::right_side_view(root.clone());
    println!("right_side_view: {result:?}"); // [10, -3, 11, 1]

    println!("----- 最大层内元素和 ------");
    let result = TreeNode::max_level_sum(root);
    println!("max_level_sum: {result}"); // 3

    println!("----- 重新规划路线(深度优先搜索) ------");
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    let result = min_reorder(6, connections);
    println!("min_reorder: {result}"); // 3

    println!("----- 迷宫中离入口最近的出口(广度优先搜索) ------");
    let maze = vec![vec!['+', '+', '.', '+'], vec!['.', '.', '.', '+'], vec!['+', '+', '+', '.']];
    let entrance = vec![1, 2];
    let result = nearest_exit(maze, entrance);
    println!("nearest_exit: {result}"); // 1

    println!("----- 数组中的第k个最大元素 ------");
    let nums = vec![3, 2, 1, 5, 6, 4];
    // let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let result = find_kth_largest(nums, 2);
    println!("find_kth_largest: {result}"); // 5

    println!("----- 无限集中的最小数字(堆/优先队列) ------");
    let mut obj = SmallestInfiniteSet::new();
    let ret_1 = obj.pop_smallest();
    obj.add_back(2);
    println!("pop_smallest: {ret_1}"); // 1

    println!("----- 雇佣k位工人的总代价 ------");
    let costs = vec![17, 12, 10, 2, 7, 2, 11, 20, 8];
    let k = 3;
    let candidates = 4;
    let result = total_cost(costs, k, candidates);
    println!("total_cost: {result}"); // 11

    println!("----- 咒语和药水的成功对数 ------");
    let spells = vec![5, 1, 3];
    let potions = vec![1, 2, 3, 4, 5];
    let success = 7;
    let result = successful_pairs(spells, potions, success);
    println!("successful_pairs: {result:?}"); // [4, 0, 3]

    println!("----- 寻找峰值元素 ------");
    let nums = vec![1, 6, 7, 5, 6, 8, 8, 8];
    let result = find_peak_element(nums);
    println!("find_peak_element: {result}"); // 7

    println!("----- 电话号码的字母组合 ------");
    let digits = String::from("23");
    let result = letter_combinations(digits);
    println!("letter_combinations: {result:?}"); // ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]

    println!("----- 组合总和 Ⅲ ------");
    let result = combination_sum3(3, 9);
    println!("combination_sum3: {result:?}"); // [[6, 2, 1], [5, 3, 1], [4, 3, 2]]

    println!("----- 多米诺和托米诺平铺 ------");
    let result = num_tilings(3);
    println!("num_tilings: {result}"); // 5

    println!("----- 不同路径 ------");
    let result = unique_paths(3, 7);
    println!("unique_paths: {result}"); // 28

    println!("----- 最长公共子序列 ------");
    let result = longest_common_subsequence("abcde".to_string(), "ace".to_string());
    println!("longest_common_subsequence: {result}");

    println!("----- 买卖股票的最佳时机含手续费 ------");
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee = 2;
    // 解释：能够达到的最大利润:
    // 在此处买入 prices[0] = 1
    // 在此处卖出 prices[3] = 8
    // 在此处买入 prices[4] = 4
    // 在此处卖出 prices[5] = 9
    // 总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8
    let result = max_profit(prices, fee);
    println!("max_profit: {result}");

    println!("----- 实现Trie(前缀树) ------");
    let mut obj = Trie::new();
    let word = "apple".to_string();
    obj.insert(word.clone());
    let ret_2 = obj.search(word);
    let prefix = "app".to_string();
    let ret_3 = obj.starts_with(prefix);
    println!("ret_2: {ret_2}, ret_3: {ret_3}");

    // println!("----- 搜索推荐系统 ------");

    // println!("----- 无重叠区间 ------");

    // println!("----- 每日温度 ------");

    // println!("----- 股票价格跨度 ------");
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

/// 重新规划路线(深度优先搜索)
// n 座城市，从 0 到 n-1 编号，其间共有 n-1 条路线。因此，要想在两座不同城市之间旅行只有唯一一条路线可供选择（路线网形成一颗树）。
// 去年，交通运输部决定重新规划路线，以改变交通拥堵的状况。
// 路线用 connections 表示，其中 connections[i] = [a, b] 表示从城市 a 到 b 的一条有向路线。
// 今年，城市 0 将会举办一场大型比赛，很多游客都想前往城市 0 。
// 请你帮助重新规划路线方向，使每个城市都可以访问城市 0 。返回需要变更方向的最小路线数。
// 题目数据 保证 每个城市在重新规划路线方向后都能到达城市 0
// n = 6, connections: [[0,1],[1,3],[2,3],[4,0],[4,5]]
fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
    for e in connections.iter() {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push((b as i32, 1));
        g[b].push((a as i32, 0));
    }

    fn dfs(a: usize, fa: i32, g: &Vec<Vec<(i32, i32)>>) -> i32 {
        let mut result = 0;
        for &(b, c) in g[a].iter() {
            if b != fa {
                result += c + dfs(b as usize, a as i32, g);
            }
        }

        result
    }

    dfs(0, -1, &g)
}
//-----------------------------------------------------

//  BinaryHeap(二叉堆)主要特性:
// 1.自动排序：当你向堆中插入元素时，堆会自动重新排序以确保堆的性质（父节点的值总是大于或等于（最大堆）或小于或等于（最小堆）其子节点的值）得到维护。
// 2.快速访问最高（或最低）优先级元素：堆的根节点（在 BinaryHeap 中，这通常是第一个元素）总是具有最高（或最低，取决于堆的类型）的优先级。因此，你可以快速地获取或删除这个元素。
// 3.性能：插入和删除堆顶元素的平均时间复杂度是 O(log n)，其中 n 是堆中元素的数量。这使得 BinaryHeap 在处理大量数据时非常高效。
// 4.泛型：BinaryHeap 是泛型的，这意味着你可以用它来存储任何实现了 Ord trait（即可以排序）的类型。

/// 迷宫出口(bfs广度优先搜索)
// maze[i][j] 要么是 '.' ，要么是 '+'
/*fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let dir = [-1, 0, 1, 0, -1]; // 方向
    let entrance = (entrance[0], entrance[1]); // 入口位置
    let mut maze = maze;
    let n = maze.len() as i32;
    let m = maze[0].len() as i32;
    // BinaryHeap(二叉堆)，主要用于处理那些需要优先队列特性的场景。
    // 二叉堆通常用于实现优先队列，其中每个元素都有一个“优先级”，并且队列按照优先级（而不是元素插入的顺序）来对元素进行排序。
    let mut fifo = std::collections::BinaryHeap::new();
    // 将入口位置及其步数（0）推入 fifo 队列
    fifo.push((0, entrance));

    // 优先级由 cnt（即从入口开始到当前单元格的步数）决定，用作路径长度的计数器
    while let Some((cnt, curr)) = fifo.pop() {
        // 尝试往4个方向移动
        for i in 0..4 {
            let x = curr.0 + dir[i];
            let y = curr.1 + dir[i + 1];
            // 如果移动后的位置在迷宫范围外，且当前位置不是入口，则返回当前步数的相反数（因为步数是从0开始的，所以其相反数实际上是负的路径长度，表示无法找到出口）。
            // 如果当前位置是入口，则继续处理其他方向。
            if x < 0 || x >= n || y < 0 || y >= m {
                if curr != entrance { return -cnt; } else { continue; }
            }
            let (xx, yy) = (x as usize, y as usize);
            // 如果移动后的位置在迷宫范围内且是可通过的（即字符为 '.'），则将该位置推入队列，并将其步数减1（表示离入口更近了一步）。
            // 同时，将已访问的单元格标记为 '+'，以避免重复访问
            if maze[xx][yy] == '.' {
                fifo.push((cnt - 1, (x, y)));
                maze[xx][yy] = '+';
            }
        }
    }

    -1
}*/
fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let mut clones = VecDeque::from_iter([(entrance[0] as usize, entrance[1] as usize)]);
    maze[entrance[0] as usize][entrance[1] as usize] = 'x';
    let mut n_step = 0;
    let mut n_this_clone = 1;
    let mut n_next_clone = 0;

    while let Some((i, j)) = clones.pop_front() {
        if i > 0 && maze[i - 1][j] == '.' {
            if i == 1 || j == 0 || j == maze[i].len() - 1 {
                return n_step + 1;
            }
            maze[i - 1][j] = 'x';
            clones.push_back((i - 1, j));
            n_next_clone += 1;
        }

        if i + 1 < maze.len() && maze[i + 1][j] == '.' {
            if i + 1 == maze.len() - 1 || j == 0 || j == maze[i].len() - 1 {
                return n_step + 1;
            }
            maze[i + 1][j] = 'x';
            clones.push_back((i + 1, j));
            n_next_clone += 1;
        }

        if j > 0 && maze[i][j - 1] == '.' {
            if j - 1 == 0 || i == 0 || i == maze.len() - 1 {
                return n_step + 1;
            }
            maze[i][j - 1] = 'x';
            clones.push_back((i, j - 1));
            n_next_clone += 1;
        }

        if j + 1 < maze[i].len() && maze[i][j + 1] == '.' {
            if j + 1 == maze[i].len() - 1 || i == 0 || i == maze.len() - 1 {
                return n_step + 1;
            }
            maze[i][j + 1] = 'x';
            clones.push_back((i, j + 1));
            n_next_clone += 1;
        }

        n_this_clone -= 1;
        if n_this_clone == 0 {
            n_this_clone = n_next_clone;
            n_next_clone = 0;
            n_step += 1;
        }
    }

    -1
}
//-----------------------------------------------------

/// 数组中的第k个最大元素(例：k=1(即最大的元素))
fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    // 要找的是第 k 大的元素，所以目标位置是排序后的数组长度减去 k
    let target_pos = nums.len() - k as usize;
    // select_nth_unstable() 从重新排序的切片中返回一个三元组：索引前的子切片的引用、索引处的元素的引用 和 索引后的子切片的引用。
    // 注:select_nth_unstable() 方法可能并不会保持原始数组的排序，它只是一个快速选择算法的实现，用于在未排序的数组中查找第 n 个最小元素。
    // 如果你的目的是查找第 k 大的元素，且不在乎算法是否保持排序。
    *nums.select_nth_unstable(target_pos).1
}
//-----------------------------------------------------

/// 堆/优先队列
// 题目:给你一个下标从 0 开始的整数数组 costs ，其中 costs[i] 是雇佣第 i 位工人的代价。
// 同时给你两个整数 k 和 candidates 。我们想根据以下规则恰好雇佣 k 位工人：
// 总共进行 k 轮雇佣，且每一轮恰好雇佣一位工人。
// 在每一轮雇佣中，从最前面 candidates 和最后面 candidates 人中选出代价最小的一位工人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
// 比方说，costs = [3,2,7,7,1,2] 且 candidates = 2 ，第一轮雇佣中，我们选择第 4 位工人，因为他的代价最小 [3,2,7,7,1,2] 。
// 第二轮雇佣，我们选择第 1 位工人，因为他们的代价与第 4 位工人一样都是最小代价，而且下标更小，[3,2,7,7,2] 。注意每一轮雇佣后，剩余工人的下标可能会发生变化。
// 如果剩余员工数目不足 candidates 人，那么下一轮雇佣他们中代价最小的一人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
// 一位工人只能被选择一次。
// 返回雇佣恰好 k 位工人的总代价。
fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let k = k as usize;
    let candidates = candidates as usize;
    if 2 * candidates + k > n {
        costs.select_nth_unstable(k - 1);
        return costs.iter().take(k).map(|&x| x as i64).sum();
    }

    let (mut prev, mut suff) = (BinaryHeap::new(), BinaryHeap::new());
    // Reverse 逆序存储成本值，可以使 BinaryHeap 按照降序的方式排列，从而可以从堆的顶部取出最大的成本值
    for i in 0..candidates {
        prev.push(Reverse(costs[i])); // 前 candidates 个成本值放入 prev 堆中，并逆序放入（通过 Reverse 结构）
        suff.push(Reverse(costs[n - 1 - i])); // 最后 candidates 个成本值放入 suff 堆中，并逆序放入
    }
    // 双指针操作
    let (mut i, mut j) = (candidates, n - candidates - 1);
    (0..k).map(|_| {
        let (p, s) = (prev.peek().unwrap().0, suff.peek().unwrap().0);
        if p <= s {
            prev.pop();
            prev.push(Reverse(costs[i]));
            i += 1;
            p as i64
        } else {
            suff.pop();
            suff.push(Reverse(costs[j]));
            j -= 1;
            s as i64
        }
    }).sum()
}
//-----------------------------------------------------

/// 成功对数(二分法查找)
fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable(); // 升序排列
    let n = potions.len();
    /*for i in spells.into_iter().map(|x| i64::from(x)) {
        let count = n - potions.partition_point(|&y| i * i64::from(y) < success);
        result.push(count as i32);
    }*/
    // partition_point() 内部使用 binary_search_by 进行查找
    // potions.partition_point() 返回符合条件的元素数量
    // let v = [1, 2, 3, 3, 5, 6, 7];
    // let i = v.partition_point(|&x| x < 5);  // 4, 注：目前只提供 < 操作
    spells.iter().map(|&x| (n - potions.partition_point(|&p| (x as i64) * (p as i64) < success)) as i32).collect()
}
//-----------------------------------------------------

/// 寻找峰值元素(二分法查找)
// 峰值元素是指其值严格大于左右相邻值的元素。
// 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
fn find_peak_element(nums: Vec<i32>) -> i32 {
    /*let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let middle = left + (right - left >> 1);
        match nums[middle] > nums[middle + 1] {
            true => right = middle,
            false => left = middle + 1,
        }
    }
    left as i32*/

    // max_by_key() 返回指定函数中给出最大值的元素。如果多个元素的最大值相等，则返回最后一个元素。如果迭代器为空，则返回None。
    // max_by_key(|(_, &v)| v) 元组的第一个元素（即索引），并返回元组的第二个元素（即值）的引用。
    nums.iter().enumerate().max_by_key(|(_, &v)| v).unwrap().0 as i32
}
//-----------------------------------------------------

/// 回溯问题
fn letter_combinations(digits: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut value: Vec<char> = Vec::new();
    match digits.is_empty() {
        true => (),
        false => get_letters(&digits, 0, &mut value, &mut result),
    }

    result
}

/// backtrack(回溯操作)
fn get_letters(digits: &String, index: usize, value: &mut Vec<char>, result: &mut Vec<String>) {
    if index >= digits.len() {
        let s = String::from_iter(value.iter());
        result.push(s);
        return;
    }
    // .iter().nth(n) 返回迭代器的第n个元素
    // 注:所有前面的元素以及返回的元素都将从迭代器中消耗掉。即前面的元素将被丢弃，并且在同一迭代器上多次调用第n（0）个元素将返回不同的元素。
    let dig_list = match digits.chars().nth(index).unwrap() {
        '2' => vec!['a', 'b', 'c'],
        '3' => vec!['d', 'e', 'f'],
        '4' => vec!['g', 'h', 'i'],
        '5' => vec!['j', 'k', 'l'],
        '6' => vec!['m', 'n', 'o'],
        '7' => vec!['p', 'q', 'r', 's'],
        '8' => vec!['t', 'u', 'v'],
        '9' => vec!['w', 'x', 'y', 'z'],
        _ => vec![]
    };

    for c in dig_list {
        value.push(c);
        get_letters(digits, index + 1, value, result);
        value.pop();
    }
}
//-----------------------------------------------------

// 找出所有相加之和为 n 的 k 个数的组合，且满足下列条件：
// 只使用数字1到9
// 每个数字 最多使用一次
// 返回 所有可能的有效组合的列表 。该列表不能包含相同的组合两次，组合可以以任何顺序返回。
fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    // 回溯函数:实现回溯算法。回溯算法常用于解决组合问题，它通过递归和剪枝的方式找出所有可能的解。
    fn backtrace(result: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>, i: i32, k: i32, n: i32) {
        let c = k - curr.len() as i32;
        // 剪枝条件:用于提前终止递归,这个条件基于组合数学中的公式，用于确定当前情况下是否还有可能找到一个满足条件的组合。
        if n < 0 || n > (i * 2 - c + 1) * c / 2 { return; }
        // 递归终止条件
        if c == 0 {
            result.push(curr.clone());
            return;
        }
        // 回溯过程
        for j in (1..=i).rev() {
            if j < c { break; }
            curr.push(j);
            backtrace(result, curr, j - 1, k, n - j);
            curr.pop();
        }
    }

    let mut result = vec![];
    backtrace(&mut result, &mut vec![], 9, k, n);
    result
}
//-----------------------------------------------------

/// 多米诺和托米诺平铺(动态规划_一维)
// 有两种形状的瓷砖：一种是 2 x 1 的多米诺形，另一种是形如 "L" 的托米诺形。两种形状都可以旋转。
// 给定整数 n ，返回可以平铺 2 x n 的面板的方法的数量。返回对 10的9次方 + 7 取模 的值。
// 平铺指的是每个正方形都必须有瓷砖覆盖。两个平铺不同，当且仅当面板上有四个方向上的相邻单元中的两个，使得恰好有一个平铺有一个瓷砖占据两个正方形。
fn num_tilings(n: i32) -> i32 {
    (1..n).fold((0, 1, 1, 1e9 as i32 + 7), |(a, b, c, m), _| (b, c, (2 * c % m + a) % m, m)).2
}
//-----------------------------------------------------

/// 不同路径(动态规划_多维),矩阵dp空间优化
// 一个机器人位于一个 m x n 网格的最左上角（标记为 “Start” ）。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的最右下角（标记为 “Finish” ）。
// 问总共有多少条不同的路径？
fn unique_paths(m: i32, n: i32) -> i32 {
    // dp关系: dp[i][j] = dp[i - 1][j] + dp[i][j - 1];

    /*let n = n as usize;
    let mut dp = vec![1; n];
    for _ in 1..(m as usize) {
        dp[0] = 1;
        for j in 1..n {
            dp[j] += dp[j - 1];
        }
    }
    dp[n - 1]*/

    // let (m, n) = (m as u64, n as u64);
    // (1..m.min(n)).fold(1, |acc, x| acc * (m + n - 1 - x) / x) as i32

    let n = n as u64 - 1;
    (1..m as u64).fold(1, |cnt, x| cnt * (n + x) / x) as i32
}
//-----------------------------------------------------

/// 动态规划
// 给定两个字符串 text1 和 text2，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。
// 一个字符串的 子序列 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
// 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。
// 两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。
fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    /*let (m, n) = (text1.len(), text2.len());
    // 因为dp[i][j] 是表示下标(i-1, j-1) 的 最长公共子序列，所以i/j == 0 都是无意义的,可以初始化为0
    let mut dp = vec![vec![0; n + 1]; m + 1];
    // 状态转移
    for (i, c1) in text1.chars().enumerate() {
        for (j, c2) in text2.chars().enumerate() {
            dp[i + 1][j + 1] = (dp[i][j] + (c1 == c2) as i32).max(dp[i][j + 1].max(dp[i + 1][j]));
        }
    }
    dp[m][n]*/

    let (m, n) = (text1.len() + 1, text2.len() + 1);
    let mut dp = vec![vec![0; n]; m];
    (1..m).for_each(|i| {
        (1..n).for_each(|j| {
            // text1.bytes().nth(i - 1)
            // text1.as_bytes().get(i - 1).copied()
            // text1.bytes()会返回一个迭代器，它逐个产生text1中每个字符的字节表示。.nth(i - 1)方法会尝试获取迭代器中第i - 1个元素的值。如果i - 1超出了迭代器的范围，它将返回None。
            // text1.as_bytes()会返回一个指向字符串内部字节数组的切片（slice），这个切片是原始字符串的直接视图，没有额外的迭代器开销。
            // get(i - 1)方法会尝试获取切片中索引为i - 1的元素的可变引用，如果这个索引是有效的，那么它就会返回一个指向该元素的引用。
            // .copied()会将这个引用转换为对应元素的值（如果存在的话），并产生一个Option<u8>
            // 在性能上，text1.as_bytes().get(i - 1).copied()通常会比text1.bytes().nth(i - 1)更快，
            // 因为as_bytes()是直接访问字符串的内部数据，而bytes()则需要在每次调用时生成一个新的迭代器。
            // 迭代器每次调用nth()时都需要从当前位置开始重新计算到目标位置，这增加了额外的开销。
            // 因此可能的话，应该优先使用text1.as_bytes().get(i - 1).copied()来访问字符串的字节。
            dp[i][j] = match text1.as_bytes().get(i - 1).copied() == text2.as_bytes().get(j - 1).copied() {
                true => dp[i - 1][j - 1] + 1,
                false => (dp[i - 1][j]).max(dp[i][j - 1]),
            }
        })
    });
    dp[m - 1][n - 1]
}
//-----------------------------------------------------

/// 动态规划(最大收益问题)
// 给定一个整数数组 prices，其中 prices[i]表示第 i 天的股票价格 ；整数 fee 代表了交易股票的手续费用。
// 你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。
// 返回获得利润的最大值。
// 注意：这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。
fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices.iter().fold((0, -prices[0]), |(sell, buy), p| (sell.max(buy + p - fee), buy.max(sell - p))).0
}
//-----------------------------------------------------