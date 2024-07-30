use std::cell::RefCell;
use std::cmp::{self, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use leet_code::{ListNode, RecentCounter, SmallestInfiniteSet, StockSpanner, TreeNode, Trie};

// 忽略提示含有大量行的函数
#[allow(clippy::too_many_lines, clippy::many_single_char_names, clippy::similar_names)]
fn main() {
    println!("------ 1768. 交替合并字符串(字符串,双指针) ------");
    let word1 = String::from("abcde");
    let word2 = String::from("xyz");
    let answer = merge_alternately(word1, word2);
    println!("merge_alternately: {answer}"); // axbyczde

    println!("------ 1071. 字符串的最大公因子(字符串,数学) ------");
    let str1 = String::from("ABABAB");
    let str2 = String::from("AB");
    let answer = gcd_of_strings2(str1, str2);
    println!("gcd_of_strings: {answer}"); // AB

    println!("------ 1431. 拥有最多糖果的孩子(数组) ------");
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let answer = kids_with_candies(candies, extra_candies);
    println!("kids_with_candies: {answer:?}"); // [true, true, true, false, true]

    println!("------ 605. 种花问题(数组,贪心) ------");
    // let flowerbed = vec![1, 0, 0, 0, 0, 1];
    let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
    // let flowerbed = vec![0, 1, 0];
    let n = 3;
    let answer = can_place_flowers(flowerbed, n);
    println!("can_place_flowers {n}: {answer}"); // true

    println!("------ 345. 反转字符串中的元音字母(字符串,双指针) ------");
    let s = "leetcode".to_string();
    // let s = "hello".to_string();
    let answer = reverse_vowels(s);
    println!("reverse_vowels: {answer}"); // leotcede

    println!("------ 283. 移动零(数组,双指针) ------");
    let mut nums = vec![0, 1, 0, 3, 12];
    // let mut nums = vec![4, 1, 5, 3, 12];
    move_zeroes(&mut nums); // [1, 3, 12, 0, 0]

    println!("------ 392. 判断子序列(字符串,双指针,动态规划) ------");
    let s = "ace";
    let t = "abcde";
    println!("Is '{}' a sub of '{}'? {}", s, t, is_subsequence(s.to_string(), t.to_string())); // true

    println!("------ 643. 子数组最大平均数Ⅰ (数组,滑动窗口) ------");
    let nums = vec![1, 12, -5, -6, 50, 3];
    let answer = find_max_average(nums, 4);
    println!("find_max_average: {answer}"); // 12.75

    println!("------ 1732. 找到最高海拔(数组,前缀和) ------");
    let gain = vec![-5, 1, 5, 0, -7];
    // let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    let answer = largest_altitude(gain);
    println!("largest_altitude: {answer}"); // 1

    println!("------ 724. 寻找数组的中心下标(数组,前缀和) ------");
    let nums = vec![1, 7, 3, 6, 5, 6];
    let answer = pivot_index(nums);
    println!("pivot_index: {answer}"); // 3

    println!("------ 2215. 找出两数组的不同(数组,哈希表) ------");
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 2, 1, 2, 4];
    let answer = find_difference(nums1, nums2);
    println!("find_difference: {answer:?}"); // [[3], [4]]

    println!("------ 1207. 独一无二的出现次数(数组,哈希表) ------");
    let arr = vec![1, 2, 2, 1, 1, 3];
    let answer = unique_occurrences(arr);
    println!("unique_occurrences: {answer}"); // true

    println!("------ 933. 最近的请求次数(头尾高效操作的队列,数据流) ------");
    let mut recent_counter = RecentCounter::new();
    let ret_1 = recent_counter.ping(1);
    println!("ping: {ret_1}");        // 1
    let ret_2 = recent_counter.ping(100);
    println!("ping: {ret_2}");        // 2
    let ret_3 = recent_counter.ping(3001);
    println!("ping: {ret_3}");        // 3
    let ret_4 = recent_counter.ping(3002);
    println!("ping: {ret_4}");        // 3

    println!("------ 206. 反转链表(递归,链表) ------");
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
        None => (), //元组类型的单元值即空元组, 告诉 Rust 不需要运行任何代码。
        Some(node) => {
            node.print_list(); // 5 4 3 2 1
        }
    }

    println!("------ 104. 二叉树的最大深度(dfs,bfs) ------");
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

    let answer = TreeNode::max_depth(Some(root.clone()));
    println!("max_depth: {answer}"); // 3

    println!("------ 872. 叶子相似的树(二叉树,dfs) ------");
    let answer = TreeNode::leaf_similar(Some(Rc::new(RefCell::new(rt))), Some(root.clone()));
    println!("leaf_similar: {answer}"); // true

    println!("------ 700. 二叉搜索树(BST)中的搜索(二叉搜索树) ------");
    let val = 20;
    let answer = TreeNode::search_bst(Some(root.clone()), val);
    println!("search_bst: {answer:?}"); // Some(RefCell { value: TreeNode { val: 20, left: Some(RefCell { value: TreeNode { val: 17, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 36, left: None, right: None } }) } })

    println!("----- 450. 删除二叉搜索树中的节点(二叉搜索树) ------");
    let answer = TreeNode::delete_node(Some(root.clone()), val);
    println!("delete_node: {answer:?}");

    println!("------ 374. 猜数字大小(二分查找,交互) ------");
    let pick_num = guess_number(10);
    println!("guessNumber: {pick_num}"); // 7

    println!("------ 1137. 第N个泰波那契数(记忆化搜索,数学,动态规划) ------");
    let n = 25;
    let answer = tribonacci(n);
    println!("tribonacci({n}): {answer}"); // 1389537

    println!("------ 746. 使用最小花费爬楼梯(数组,动态规划) ------");
    // let cost = vec![10, 15, 20]; // 15
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]; // 6
    let answer = min_cost_climbing_stairs(cost);
    println!("min_cost_climbing_stairs: {answer}"); // 6

    println!("------ 338. 比特位计数(位运算,动态规划) ------");
    let n = 5;
    let answer = count_bits(n);
    println!("count_bits({n}): {answer:?}"); // [0, 1, 1, 2, 1, 2]

    println!("------ 136. 只出现一次的数字(位运算,数组) ------");
    let nums = vec![4, 1, 2, 1, 2];
    let answer = single_number(nums);
    println!("single_number: {answer}"); // 4

    println!("----- 1318. 或运算的最小翻转次数(位运算) ------");
    let answer = min_flips(2, 6, 5);
    println!("min_flips: {answer}"); // 3

    println!("\n-------------up---------------\n");

    println!("------ 151. 反转字符串中的单词(字符串,双指针) ------");
    let s = "  a good   example ".to_string();
    let answer = reverse_words(s);
    println!("reverse_words: {answer}"); // example good a

    println!("------ 238. 除自身以外数组的乘积(数组,前缀和) ------");
    let nums = vec![1, 2, 3, 4];
    let answer = product_except_self(nums);
    println!("product_except_self: {answer:?}"); // [24, 12, 8, 6]

    println!("----- 334. 递增的三元子序列(贪心,数组) ------");
    // 判断数组nums中是否存在长度为 3 的递增子序列。
    // 如果存在这样的三元组下标 (i, j, k) 且满足 i < j < k,使得 nums[i] < nums[j] < nums[k],返回 true;否则,返回 false
    // 三元组 (3, 4, 5) 满足题意，因为 nums[3] == 0 < nums[4] == 4 < nums[5] == 6,返回true
    let nums = vec![2, 1, 5, 0, 4, 6];
    let answer = increasing_triplet(nums);
    println!("increasing_triplet: {answer}"); // true

    println!("------ 443. 压缩字符串(字符串,双指针) ------");
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    // let mut chars = vec!['a'];
    // let mut chars = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
    let answer = compress(&mut chars);
    println!("compress: {answer}"); // 6  ['a', '2', 'b', '2', 'c', '3']

    println!("------ 11. 盛最多水的容器(数组,双指针,贪心) ------");
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    // let height = vec![1, 1];
    let max_area = max_area(height);
    println!("Max water: {max_area}"); // 49

    println!("----- 1679. K和数对的最大数目(数组,哈希表,双指针，排序) ------");
    // 整数数组 nums 和整数 k ,每一步操作中，需要从数组中选出和为 k 的两个整数，并将它们移出数组。返回你可以对数组执行的最大操作数。
    let nums = vec![3, 1, 3, 4, 3];
    let k = 6;
    let answer = max_operations(nums, k);
    println!("max_operations: {answer}"); // 1

    println!("------ 1456. 定长子串中元音的最大数目(字符串,滑动窗口) ------");
    let s = "abciiidef".to_string();
    let k = 3;
    let answer = max_vowels(s, k);
    println!("max_vowels: {answer}"); // 3

    println!("----- 1004.最大连续1的个数 III(数组,双指针,前缀和,滑动窗口) ------");
    // 二进制数组 nums 和整数 k，如果可以翻转最多 k 个 0 ，则返回 数组中连续 1 的最大个数 。
    let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k = 3;
    let answer = longest_ones(nums, k);
    println!("longest_ones: {answer}"); // 10

    println!("----- 1493. 删掉一个元素以后全为 1 的最长子数组(数组,动态规划,滑动窗口) ------");
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    let answer = longest_subarray(nums);
    println!("longest_subarray: {answer}");

    println!("------ 1657. 确定两个字符串是否接近(字符串,哈希表,计数) ------");
    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    let answer = close_strings(word1, word2);
    println!("close_strings: {answer}"); // true

    println!("----- 2352. 相等行列对(数组,哈希,矩阵,模拟) ------");
    // 给你一个下标从 0 开始、大小为 n x n 的整数矩阵 grid ，返回满足 Ri 行和 Cj 列相等的行列对 (Ri, Cj) 的数目。
    // 如果行和列以相同的顺序包含相同的元素（即相等的数组），则认为二者是相等的。
    let grid = vec![vec![3, 1, 2, 2], vec![1, 4, 4, 5], vec![2, 4, 2, 2], vec![2, 4, 2, 2]];
    // 存在三对相等行列对：
    // - (第 0 行，第 0 列)：[3,1,2,2]
    // - (第 2 行, 第 2 列)：[2,4,2,2]
    // - (第 3 行, 第 2 列)：[2,4,2,2]
    let answer = equal_pairs(grid);
    println!("equal_pairs: {answer}"); // 3

    println!("------ 2390. 从字符串中移除星号(栈,字符串) ------");
    let s = "leet**cod*e".to_string(); // lecoe
    // let s = String::from("erase*****"); // ""
    let answer = remove_stars(s);
    println!("remove_stars: {answer}");

    println!("----- 735. 小行星碰撞(数组,栈,模拟) ------");
    // 整数数组 asteroids，表示在同一行的小行星。
    // 对于数组中的每一个元素，其绝对值表示小行星的大小，正负表示小行星的移动方向（正表示向右移动，负表示向左移动）。每一颗小行星以相同的速度移动。
    // 找出碰撞后剩下的所有小行星。
    // 碰撞规则：两个小行星相互碰撞，较小的小行星会爆炸。如果两颗小行星大小相同，则两颗小行星都会爆炸。两颗移动方向相同的小行星，永远不会发生碰撞。
    let asteroids = vec![10, 2, -5];
    let answer = asteroid_collision(asteroids);
    println!("asteroid_collision: {answer:?}");

    println!("------ 394. 字符串解码(栈,字符串,递归) ------");
    let s = "3[a12[c]]".to_string();  // accccccccccccaccccccccccccacccccccccccc
    // let s = "3[a]2[bc]".to_string(); // aaabcbc
    let answer = decode_string(s);
    println!("decode_string: {answer}"); // accccccccccccaccccccccccccacccccccccccc

    println!("----- 649. Dota2 参议院(贪心,队列,字符串) ------");
    let senate = String::from("RDD");
    let answer = predict_party_victory(senate);
    println!("predict_party_victory: {answer}");

    println!("------ 2095. 删除链表的中间节点(链表,双指针) ------");
    let node_head = ListNode::delete_middle(node_rev);
    match node_head.clone() {
        None => (),
        Some(node) => node.print_list(),
    } // 5 4 2 1

    println!("----- 328. 奇偶链表(链表) ------");
    let odd_even_head = ListNode::odd_even_list(node_head);
    match odd_even_head.clone() {
        None => (),
        Some(node) => node.print_list(),
    } // 5 2 4 1

    println!("----- 2130. 链表最大孪生和(链表,栈,双指针) ------");
    let node_head = odd_even_head.clone();
    let answer = ListNode::pair_sum(node_head);
    println!("ListNode::pair_sum: {answer}"); // 6

    println!("----- 1448. 统计二叉树中好节点的数目(dfs,bfs) ------");
    let answer = TreeNode::good_nodes(Some(root));
    println!("good_nodes: {answer}"); // 4

    println!("----- 437. 二叉树路径总和Ⅲ(二叉树,dfs) ------");
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None }))),
                right: Some(Rc::new(RefCell::new(TreeNode { val: -2, left: None, right: None }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode { val: 11, left: None, right: None }))),
        }))),
    })));
    let answer = TreeNode::path_sum(root.clone(), 8);
    println!("path_sum: {answer}"); // 3

    println!("----- 1372. 二叉树中的最长交错路径(dfs,动态规划) ------");
    let answer = TreeNode::longest_zig_zag(root.clone());
    println!("longest_zig_zag: {answer}"); // 2

    println!("----- 199. 二叉树的右视图(dfs,bfs) ------");
    let answer = TreeNode::right_side_view(root.clone());
    println!("right_side_view: {answer:?}"); // [10, -3, 11, 1]

    println!("----- 1161. 最大层内元素和(dfs,bfs) ------");
    let answer = TreeNode::max_level_sum(root);
    println!("max_level_sum: {answer}"); // 3

    println!("----- 841. 钥匙和房间(dfs,bfs,图) ------");
    let rooms = vec![vec![1], vec![2], vec![3], vec![]]; // true
    // let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]; // false
    let answer = can_visit_all_rooms(rooms);
    println!("can_visit_all_rooms: {answer}");

    println!("----- 547. 省份数量(并查集,图) ------");
    // 有 n 个城市，其中一些彼此相连，另一些没有相连。如果城市 a 与城市 b 直接相连，且城市 b 与城市 c 直接相连，那么城市 a 与城市 c 间接相连。
    // 省份 是一组直接或间接相连的城市，组内不含其他没有相连的城市。
    // 给你一个 n x n 的矩阵 isConnected ，其中 isConnected[i][j] = 1 表示第 i 个城市和第 j 个城市直接相连，而 isConnected[i][j] = 0 表示二者不直接相连。
    // 返回矩阵中 省份 的数量。
    let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let answer = find_circle_num2(is_connected);
    println!("find_circle_num: {answer}"); // 2

    println!("----- 1466. 重新规划路线(图,dfs,bfs) ------");
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    let answer = min_reorder(6, connections);
    println!("min_reorder: {answer}"); // 3

    println!("----- 399. 除法求值(dfs,bfs,并查集,图,数组,字符串,最短路径) ------");
    let equations = vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()],
                         vec!["bc".to_string(), "cd".to_string()]];
    let values = vec![1.5, 2.5, 5.0];
    let queries = vec![vec!["a".to_owned(), "c".to_owned()], vec!["c".to_owned(), "b".to_owned()],
                       vec!["bc".to_owned(), "cd".to_owned()], vec!["cd".to_owned(), "bc".to_owned()]];
    let answer = calc_equation(equations, values, queries);
    println!("calc_equation: {answer:?}"); // [3.75, 0.4, 5.0, 0.2]

    println!("----- 1926. 迷宫中离入口最近的出口(图,bfs) ------");
    let maze = vec![vec!['+', '+', '.', '+'], vec!['.', '.', '.', '+'], vec!['+', '+', '+', '.']];
    let entrance = vec![1, 2];
    let answer = nearest_exit(maze, entrance);
    println!("nearest_exit: {answer}"); // 1

    println!("----- 994. 腐烂的橘子(bfs,数组,矩阵) ------");
    // 在给定的 m x n 网格 grid 中，每个单元格可以有以下三个值之一：
    // 0: 空单元格; 1: 新鲜橘子; 2: 腐烂的橘子。
    // 每分钟，腐烂的橘子 周围 4 个方向上相邻 的新鲜橘子都会腐烂。
    // 返回 直到单元格中没有新鲜橘子为止所必须经过的最小分钟数。如果不可能，返回 -1 。
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let answer = oranges_rotting(grid);
    println!("oranges_rotting: {answer}");

    println!("----- 215. 数组中的第k个最大元素(数组,分治,快速选择,排序(堆/优先队列)) ------");
    let nums = vec![3, 2, 1, 5, 6, 4];
    // let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let answer = find_kth_largest(nums, 2);
    println!("find_kth_largest: {answer}"); // 5

    println!("----- 2336. 无限集中的最小数字(堆/优先队列) ------");
    let mut obj = SmallestInfiniteSet::new();
    let ret_1 = obj.pop_smallest();
    obj.add_back(2);
    println!("pop_smallest: {ret_1}"); // 1

    println!("----- 2542. 最大子序列的分数(贪心,数组,排序,堆(优先队列)) ------");
    let num1 = vec![1, 3, 3, 2];
    let num2 = vec![2, 1, 3, 4];
    let answer = max_score(num1, num2, 3);
    println!("max_score: {answer}"); // 12

    println!("----- 2462. 雇佣k位工人的总代价(数组,双指针) ------");
    let costs = vec![17, 12, 10, 2, 7, 20, 11, 2, 8];       // 11
    // let costs = vec![17, 12, 10, 2, 7, 20, 11, 2, 8, 28, 11, 28]; // 17
    let k = 3;
    let candidates = 4;
    let answer = total_cost(costs, k, candidates);
    println!("total_cost: {answer}"); // 11

    println!("----- 2300. 咒语和药水的成功对数(数组,双指针,二分查找) ------");
    let spells = vec![5, 1, 3];
    let potions = vec![1, 2, 3, 4, 5];
    let success = 7;
    let answer = successful_pairs(spells, potions, success);
    println!("successful_pairs: {answer:?}"); // [4, 0, 3]

    println!("----- 162. 寻找峰值元素的索引(数组,二分查找) ------");
    let nums = vec![1, 6, 7, 5, 6, 8, 8, 8];
    let answer = find_peak_element(nums);
    println!("find_peak_element: {answer}"); // 7

    println!("----- 875. 爱吃香蕉的珂珂(数组,二分查找) ------");
    // 这里有 n 堆香蕉，第 i 堆中有 piles[i] 根香蕉。警卫已经离开了，将在 h 小时后回来。
    // 珂珂可以决定她吃香蕉的速度 k (单位:根/小时)。每个小时，她将会选择一堆香蕉，从中吃掉 k 根。
    // 如果这堆香蕉少于 k 根，她将吃掉这堆的所有香蕉，然后这一小时内不会再吃更多的香蕉。
    // 珂珂喜欢慢慢吃，但仍然想在警卫回来前吃掉所有的香蕉。
    // 返回她可以在 h 小时内吃掉所有香蕉的最小速度 k(k 为整数)。
    let piles = vec![30, 11, 23, 4, 20];
    let h = 6;
    let answer = min_eating_speed(piles, h);
    println!("min_eating_speed: {answer}"); // 23

    println!("----- 17. 电话号码的字母组合(字符串,哈希表,回溯) ------");
    let digits = String::from("23");
    let answer = letter_combinations(digits);
    println!("letter_combinations: {answer:?}"); // ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]

    println!("----- 216. 组合总和Ⅲ(数组,回溯) ------");
    let answer = combination_sum3(3, 9);
    println!("combination_sum3: {answer:?}"); // [[6, 2, 1], [5, 3, 1], [4, 3, 2]]

    println!("----- 198. 打家劫舍(数组,动态规划) ------");
    let nums = vec![2, 7, 9, 3, 1];
    let answer = rob(nums);
    println!("rob: {answer}");

    println!("----- 790. 多米诺和托米诺平铺(动态规划) ------");
    let answer = num_tilings(3);
    println!("num_tilings: {answer}"); // 5

    println!("----- 62. 不同路径(组合数学,动态规划) ------");
    let answer = unique_paths(3, 7);
    println!("unique_paths: {answer}"); // 28

    println!("----- 1143. 最长公共子序列(字符串,动态规划) ------");
    let answer = longest_common_subsequence("abcde".to_string(), "ace".to_string());
    println!("longest_common_subsequence: {answer}"); // 3

    println!("----- 714. 买卖股票的最佳时机含手续费(数组,贪心,动态规划) ------");
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee = 2;
    // 解释：能够达到的最大利润:
    // 在此处买入 prices[0] = 1
    // 在此处卖出 prices[3] = 8
    // 在此处买入 prices[4] = 4
    // 在此处卖出 prices[5] = 9
    // 总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8
    let answer = max_profit(prices, fee);
    println!("max_profit: {answer}"); // 8

    println!("----- 72. 编辑距离(字符串,动态规划) ------");
    let word1 = "intention".to_string();
    let word2 = "execution".to_string();
    let answer = min_distance(word1, word2);
    println!("min_distance: {answer}"); // 5

    println!("----- 208. 实现Trie(前缀树) ------");
    let mut trie = Trie::new();
    let word = "apple".to_string();
    trie.insert(word.clone());
    let ret_2 = trie.search(word);
    let prefix = "app".to_string();
    let ret_3 = trie.search(prefix.clone());
    let ret_4 = trie.starts_with(prefix.clone());
    trie.insert(prefix.clone());
    let ret_6 = trie.search(prefix);
    println!("ret_2: {ret_2}, ret_3: {ret_3}, ret_4: {ret_4}, ret_6: {ret_6}"); // true, false, true, true

    println!("----- 1268. 搜索推荐系统(数组,字符串,字典树) ------");
    let products = vec!["mobile".to_string(), "mouse".to_string(), "moneypot".to_string(),
                        "monitor".to_string(), "mousepad".to_string()];
    let search_word = "mouse".to_string();
    let answer = suggested_products(products, search_word);
    println!("suggested_products: {answer:?}"); // [["mobile", "moneypot", "monitor"], ["mobile", "moneypot", "monitor"], ["mouse", "mousepad"], ["mouse", "mousepad"], ["mouse", "mousepad"]]

    println!("----- 435. 无重叠区间(数组,贪心,动态规划) ------");
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    let answer = erase_overlap_intervals(intervals);
    println!("erase_overlap_intervals: {answer}"); // 1

    println!("----- 452. 用最少数量的箭引爆气球(贪心,数组,排序) ------");
    // 有许多球形气球贴在一堵用 XY 平面表示的墙面上。
    // 墙面上的气球记录在整数数组 points ，其中points[i] = [Xstart, Xend] 表示水平直径在 Xstart 和 Xend之间的气球。你不知道气球的确切 y 坐标。
    // 一支弓箭可以沿着 x 轴从不同点 完全垂直 地射出。
    // 在坐标 x 处射出一支箭，若有一个气球的直径的开始和结束坐标为 Xstart，Xend，且满足 Xstart ≤ X ≤ Xend，则该气球会被 引爆 。
    // 可以射出的弓箭的数量 没有限制 。弓箭一旦被射出之后，可以无限地前进。
    // 数组 points ,返回引爆所有气球所必须射出的 最小 弓箭数。
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let answer = find_min_arrow_shots(points);
    println!("find_min_arrow_shots: {answer}"); // 2

    println!("----- 739. 每日温度(单调栈) ------");
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let answer = daily_temperatures(temperatures);
    println!("daily_temperatures: {answer:?}"); // [1, 1, 4, 2, 1, 1, 0, 0]

    println!("----- 901. 股票价格跨度(单调栈,数据流) ------");
    // 设计一个算法收集某些股票的每日报价，并返回该股票当日价格的 跨度。
    // 当日股票价格的 跨度 被定义为股票价格小于或等于今天价格的最大连续日数(从今天开始往回数，包括今天)。
    // 例:如果未来7天股票的价格是 [100,80,60,70,60,75,85]，那么股票跨度将是 [1,1,1,2,1,4,6]。
    let mut stock_spanner = StockSpanner::new();
    let ret_1 = stock_spanner.next(100);
    println!("stock_spanner.next(100): {ret_1}");  // 1
    let ret_1 = stock_spanner.next(80);
    println!("stock_spanner.next(80): {ret_1}");   // 1
    let ret_1 = stock_spanner.next(60);
    println!("stock_spanner.next(60): {ret_1}");   // 1
    let ret_1 = stock_spanner.next(70);
    println!("stock_spanner.next(70): {ret_1}");   // 2
    let ret_1 = stock_spanner.next(60);
    println!("stock_spanner.next(60): {ret_1}");   // 1
    let ret_1 = stock_spanner.next(75);
    println!("stock_spanner.next(75): {ret_1}");   // 4
    let ret_1 = stock_spanner.next(85);
    println!("stock_spanner.next(85): {ret_1}");   // 6
}

/// 交替合并字符串
fn merge_alternately(word1: String, word2: String) -> String {
    // let len1 = word.len();
    // let len2 = word.chars().count();
    // word.len():这个方法直接返回字符串中字节的数量。在Rust中,String是一个UTF-8编码的字符串,所以len()方法返回的是字节的数量。
    // 如果字符串只包含ASCII字符,那么字节和字符的数量是相同的。
    // 但是,如果字符串包含非ASCII字符(如中文字符或其他Unicode字符),一个字符可能由多个字节表示。因此,len()返回的可能不是字符的实际数量。
    // word.len()的执行效率非常高,因为它只需要读取字符串的内部长度字段,不需要遍历整个字符串。
    //
    // word.chars().count():这个方法首先将字符串转换为一个字符迭代器,然后计算迭代器的长度。
    // 即是它需要遍历整个字符串来计算字符的数量。因此,它的执行效率通常比 len() 低,特别是当字符串很长时。
    // 当需要知道字符串中字符的实际数量,无论它们是否由多少个字节表示,则 word.chars().count() 才是正确的做法。

    let len1 = word1.chars().count();
    let len2 = word2.chars().count();
    let mut answer = String::with_capacity(len1 + len2);
    // 使用zip()将两个等长的Vec组合成一个Vec,其中元素是一个元组,包含原来两个Vec中对应位置的元素。
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        answer.push(c1);
        answer.push(c2);
    }

    // .iter().skip(n):从迭代器中跳过前 n 个元素
    for c in word1.chars().skip(len2) {
        answer.push(c);
    }
    for c in word2.chars().skip(len1) {
        answer.push(c);
    }

    answer
}
//-----------------------------------------------------

/// 字符串的最大公因子
// 题目要求:字符串中的字符全是字母
fn _gcd_of_strings(str1: String, str2: String) -> String {
    let len1 = str1.len();
    let len2 = str2.len();

    // 求两个字符串长度的最大公约数
    // .find()：用于查找单个元素,返回满足条件的第一个元素(如果存在)。返回类型为 Option<T>。
    // .filter()：返回一个新迭代器，包含所有满足条件的元素。返回类型为实现了 Iterator<Item=T> 的类型。
    let gec_len = (1..=cmp::min(len1, len2)).rev()
        .find(|&i| len1 % i == 0 && len2 % i == 0).unwrap_or_else(|| cmp::min(len1, len2));

    fn can_divide(s1: &str, s2: &str) -> bool {
        // .chunks_exact(s2.len()) 将 s1 的字节切片分割成长度为 s2.len() 的块。
        // 如果 s1 的长度不是 s2.len() 的整数倍,这个函数会抛出一个 panic。但由于s1.len() % s2.len() == 0，所以这里不会有问题。
        // .all(|chunk| chunk == s2.as_bytes()) 对所有分割出的块执行检查每个块是否都与 s2 的字节切片相等。
        // 如果所有块都相等,那么 s1 是由 s2 重复构成的,函数返回 true,否则返回 false。
        s1.len() % s2.len() == 0 && s1.as_bytes().chunks_exact(s2.len()).all(|chunk| chunk == s2.as_bytes())
    }
    // let cd1 = can_divide(&str1, &str1[0..gec_len]);
    // let cd2 = can_divide(&str2, &str1[0..gec_len]);
    // println!("cd1: {cd1}, cd2: {cd2}");
    if can_divide(&str1, &str1[0..gec_len]) && can_divide(&str2, &str1[0..gec_len]) {
        return str1[0..gec_len].to_string();
    }

    // 创建空字符串推荐使用 String::new() 的方式
    // "".to_string()
    String::new()
}

/// 字符串的最大公因子
/// 解法二:使用欧几里得算法
// 欧几里得算法即辗转相除法，指用于计算两个非负整数a，b的最大公约数。计算公式gcd(a,b) = gcd(b, a mod b)。
// 两个整数的最大公约数等于其中较小的数和两数相除余数的最大公约数
// 如果两个字符串交替相加后，值仍然相等，即str1 + str2 == str2 + str1时，就可以认为存在公因子字符串。
// 当一定存在公因子时，最大公因子字符的长度一定就是两个字符串长度的最大公因数。
// 公因子字符串也就是str1或str2的前缀下标。范围为:[0，最大公因数]
fn gcd_of_strings2(str1: String, str2: String) -> String {
    // let s1 = str1.clone() + &str2;  // 消耗内存稍高，运行稍快
    // let s2 = str2.clone() + &str1;
    let s1 = format!("{str1}{str2}"); // 消耗内存稍低，但是运行稍慢
    let s2 = format!("{str2}{str1}");
    if s1 != s2 {
        return String::new();
    }

    fn get_gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            get_gcd(b, a % b)
        }
    }

    str1[0..get_gcd(str1.len(), str2.len())].to_string()

    // 解法二:
    /*if str1.len() < str2.len() {
        return gcd_of_strings2(str2, str1);
    }
    if str2.is_empty() {
        return str1;
    }

    return if str1.starts_with(&str2) {
        gcd_of_strings2(str1[str2.len()..].to_string(), str2)
    } else {
        "".to_string()
    };*/
}
//-----------------------------------------------------

// 通过遍历candies并比较每个孩子的糖果数量加上extra_candies之后是否大于或等于数组中的最大值。
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
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

// 题目要求:每朵花的旁边都不能种花，所以种花必须是间隔种1朵
// n:是否可以种的花数量
fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
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

/// 是否是元音字母
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn reverse_vowels(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    // 双指针操作
    let (mut i, mut j) = (0, chars.len() - 1);
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

// 解法二:使用bytes操作
/*fn reverse_vowels2(mut s: String) -> String {
    let vowels = b"aAeEiIoOuU".into_iter().collect::<std::collections::BTreeSet<_>>();
    let bytes = unsafe { s.as_bytes_mut() };
    let (mut i, mut j) = (0, bytes.len() - 1);
    while i < j {
        if !vowels.contains(&bytes[i]) {
            i += 1;
        } else if !vowels.contains(&bytes[j]) {
            j -= 1;
        } else {
            bytes.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    s
}*/
//-----------------------------------------------------

fn move_zeroes(nums: &mut Vec<i32>) {
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

/// 给定字符串 s 和 t,判断 s 是否为 t 的子序列
fn is_subsequence(s: String, t: String) -> bool {
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

/// 找出平均数最大值且长度为 k 的连续子数组
fn find_max_average(nums: Vec<i32>, k: usize) -> f64 {
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

/// 使用归约操作解决前缀和问题
fn largest_altitude(gain: Vec<i32>) -> i32 {
    /*let (mut highest, mut sum) = (0, 0);
    for g in gain {
        sum += g;
        if sum > highest { highest = sum; }
    }
    highest */

    // fold() 用于归约操作(将集合中的所有元素组合成一个单一的值)
    // fold() 方法接受一个初始值和一个闭包(或函数),该闭包定义了如何将集合中的每个元素与累积器(accumulator)的值结合起来。
    // 闭包有两个参数:第一个是累积器的当前值,第二个是集合中的当前元素。
    // let numbers = vec![1, 2, 3, 4, 5];
    // let sum = numbers.iter().fold(0, |accumulator, &num| accumulator + num); // 15
    gain.iter().fold((0, 0), |(highest, sum), g| (highest.max(sum + g), sum + g)).0
}
//-----------------------------------------------------

/// 中心下标
// 数组的中心下标 是数组的一个下标,其左侧所有元素相加的和等于右侧所有元素相加的和。
// 如果中心下标位于数组最左端,那么左侧数之和视为 0,因为在下标的左侧不存在元素。这一点对于中心下标位于数组最右端同样适用。
// 如果数组有多个中心下标,应该返回 最靠近左边 的那一个。如果数组不存在中心下标,返回 -1 。
// #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = nums.iter().sum();
    for (i, v) in nums.iter().enumerate() {
        sum -= v;
        if sum == 0 {
            // return i as i32;
            return i32::try_from(i).unwrap_or_default();
        }
        sum -= v;
    }

    -1
}
//-----------------------------------------------------

/// 使用哈希集合解决去重复问题
fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    // let set1: HashSet<&i32> = nums1.iter().collect();
    let set1: HashSet<i32> = nums1.into_iter().collect(); // 效率更高
    let set2: HashSet<i32> = nums2.into_iter().collect();

    // let unique_in_nums1: Vec<i32> = nums1.into_iter().filter(|&x| !set2.contains(&x)).collect();
    // let unique_in_nums1: Vec<i32> = nums1.iter().filter(|x| !set2.contains(x)).cloned().collect();
    // let unique_in_nums1: Vec<i32> = unique_in_nums1.into_iter().collect::<HashSet<_>>().into_iter().collect();

    // 调用 HashSet 的 difference() 方法,返回一个迭代器,其中包含所有在 set1 中但不在 set2 中的元素。
    // set1.difference(&set2).map(|&x| x).collect(); // 这种方式会更高效
    // set1.difference(&set2).cloned().collect();    // 作用同上
    // .cloned() 是一个适配器,它会对迭代器中的每个元素调用 clone() 方法。
    // 使用 .map() 方法来克隆每个元素。.map() 方法接受一个闭包,该闭包对迭代器中的每个元素进行转换。
    // 在这里，闭包 |&x| x 实际上并没有改变元素，因为它只是借用并返回了元素本身。
    // 因此，这里的 map 操作实际上并没有做额外的工作,只是简单地返回了元素的引用
    // vec![set1.difference(&set2).cloned().collect(), set2.difference(&set1).map(|&x| x).collect()]

    // .copied() 是一个用于复制迭代器中原始元素值的适配器。.copied()的作用和上面的.map(|&x| x)是一样的
    // 它通常用于原始元素是 Copy trait 的实现者的情况,这意味着这些元素可以通过简单的位复制来复制,而不是通过调用 clone() 方法。
    // 这通常比 clone() 克隆更高效，因为位复制通常比调用 clone() 方法更快。
    // .cloned() 用于克隆实现了 Clone trait 的元素。
    // .copied() 用于复制实现了 Copy trait 的元素,这通常比克隆更高效。
    // 在选择使用哪个适配器时:
    // 如果元素是 Copy 的,使用 .copied() 通常是更好的选择。
    // 如果元素不是 Copy 的,但实现了 Clone trait,那应该使用 .cloned()。
    vec![set1.difference(&set2).copied().collect(), set2.difference(&set1).copied().collect()]
}
//-----------------------------------------------------

/// 使用哈希集合解决统计出现次数问题
fn unique_occurrences(arr: Vec<i32>) -> bool {
    // 存储每个数的出现次数的集合
    let mut count_map = HashMap::new();
    /*for num in arr {
        // 使用 entry() 方法检查键 num 是否已经存在于 count_map 中,
        // 如果键 num 不存在,or_insert(0) 会将键 num 插入到哈希映射中,并设置其对应的值为 0。
        // 如果遇到相同的键时,就可以在其现有值的基础上增加计数。
        *count_map.entry(num).or_insert(0) += 1;
    }*/
    // for_each() 方法,用于遍历迭代器中的每个元素,并对每个元素执行一个给定的操作。
    arr.into_iter().for_each(|num| *count_map.entry(num).or_insert(0) += 1);

    // .iter() 方法返回一个对原始数组 arr 的引用迭代器，即它不会消耗或移动 arr 中的数据。
    // .into_iter() 方法将 arr 转换为一个拥有权的迭代器，即它消耗了 arr，并且 arr 在迭代之后不再可用。
    // 如果你在迭代之后仍然需要访问或使用 arr，那么显然应该选择 .iter()，因为 into_iter() 会消耗 arr。
    // 如果迭代之后不再需要 arr，那么理论上 .into_iter() 可能会稍微快一些，因为它避免了引用计数的操作(如果 arr 是一个引用类型的话)。在大多数实际场景中差异都是可以忽略不计。
    // 但是当迭代的数据元素体积比较大时，.iter()的性能反而会更高

    // 存储出现次数的集合
    let mut occurrences = HashSet::new();
    /*for count in count_map.values() {
        // 将值添加到HashSet集合中。
        // 返回值:是否是新插入的。即:如果集合以前不包含此值，则返回true;如果集合已经包含此值，则返回false;并且不修改集合:不替换原始值,并删除作为参数传递的值
        if !occurrences.insert(*count) { return false; }
    }
    true */
    // all() 方法:用于检查迭代器中的所有元素是否都满足给定的条件.检查 count_map 中的每个出现次数(即值)是否都是唯一的。
    // 如果所有出现次数都成功插入到 occurrences 中,all() 方法将返回 true。
    // 如果有任何出现次数已经存在于 occurrences 中,all() 方法将立刻返回 false。
    count_map.values().all(|count| occurrences.insert(count))
}
//-----------------------------------------------------

/// 猜数字大小(二分法查找问题)
// 题目要求:数字范围是[1, n]
fn guess_number(n: i32) -> i32 {
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
    // 这里应该是调用实际的猜数字接口的逻辑,但在这只是模拟一下,假设选中的数字是某个固定的值,比如7
    match num.cmp(&7) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
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

    // 解法二:
    // 动态规划(Dynamic Programming,DP)来避免重复计算。
    // 动态规划是一种算法设计技术，用于解决具有重叠子问题和最优子结构特性的问题。
    // 对于泰波那契数列,使用动态规划来存储已经计算过的值,避免重复计算。
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => {
            (2..n).fold((0, 1, 1), |(p1, p2, p3), _| (p2, p3, p1 + p2 + p3)).2
            /*let (mut p1, mut p2, mut p3) = (0, 1, 1);
            for _ in 2..n { (p1, p2, p3) = (p2, p3, p1 + p2 + p3) }
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
    // count_ones()函数是std::u32::TrailingZeroBits trait的一部分,适用于所有(有/无符号)整数类型(包括u8 ~ u128 & i8 ~ i128)
    (0..=n).map(|x| x.count_ones() as i32).collect()
}
//-----------------------------------------------------

// 题目：给你一个 非空 整数数组 nums,除了某个元素只出现一次以外,其余每个元素均出现两次。找出那个只出现了一次的元素。
// 假设数组中重复的元素为x,只出现一次的元素为y。
// 将数组中的所有元素进行异或运算,由于x出现了两次,所以x和x异或的结果为0,而y只出现了一次,所以最后的结果就是y。
/// 异或（XOR）运算问题。异或运算有一个重要的性质:任何数和0异或都等于它本身,任何数和其自身异或都等于0。
fn single_number(nums: Vec<i32>) -> i32 {
    /*let mut single = 0;
    nums.iter().for_each(|num| single ^= num);
    single*/

    // 解法二:
    // reduce() 和 fold() 在功能上是相似的,但它们的初始值和参数的顺序是不同的。
    // 两者都可以用来累积一个值,通过对集合中的元素应用某种操作。
    // 对于 reduce() 方法,它接受一个二元操作函数,并将集合中的元素两两组合起来,直到只剩下一个元素。
    // reduce() 不需要初始值,因为它使用集合中的第一个元素作为初始值。
    // 注:reduce() 使用数组中的第一个元素作为初始值进行异或操作;
    // 如果数组为空, reduce() 会返回一个 None,需要使用 unwrap() 来获取结果,这可能会导致运行时错误(如果数组为空)。
    // 在这种情况下, reduce() 运行更高效
    // nums.into_iter().reduce(|x, y| x ^ y).unwrap()

    nums.iter().fold(0, |single, num| single ^ num)
}
//-----------------------------------------------------

// 给定三个正整数 a、b 和 c；可以对 a 和 b 的二进制表示进行位翻转操作，返回能够使按位或运算 a OR b == c 成立的最小翻转次数。
// 「位翻转操作」是指将一个数的二进制表示任何单个位上的 1 变成 0 或者 0 变成 1
// 输入：a = 2, b = 6, c = 5
// 输出：3
// 解释：翻转后 a = 1 , b = 4 , c = 5 使得 a OR b == c
fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    /*let mut answer = 0;
    for i in 0..32 {
        let (a_i, b_i, c_i) = ((a >> i) & 1, (b >> i) & 1, (c >> i) & 1);
        if c_i == 0 {
            answer += a_i + b_i;
        } else if a_i + b_i == 0 {
            answer += 1;
        }
    }
    answer*/

    // 解法二:
    /*let mut answer = 0;
    for i in 0..(32 - c.leading_zeros()).max(32 - a.leading_zeros()).max(32 - b.leading_zeros()) {
        if ((a | b) >> i) & 1 != (c >> i & 1) {
            answer += if c >> i & 1 == 1 { 1 } else { (a >> i & 1) + (b >> i & 1) }
        }
    }
    answer*/

    // 解法三:
    (((a | b) ^ c).count_ones() + ((a & b) & !c).count_ones()) as i32
}
//-----------------------------------------------------

// 给你一个字符串 s,请你反转字符串中 单词 的顺序。
// 单词 是由非空格字符组成的字符串。s 中使用至少一个空格将字符串中的 单词 分隔开。
// 返回 单词 顺序颠倒且 单词 之间用单个空格连接的结果字符串。
// 题目要求:输入的字符串s中可能会存在前导空格、尾随空格或者单词间的多个空格。返回的结果字符串中,单词间应当仅用单个空格分隔,且不包含任何额外的空格。
fn reverse_words(s: String) -> String {
    /*let mut words: VecDeque<String> = VecDeque::new();
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
    if !curr_word.is_empty() { words.push_front(curr_word); }
    words.into_iter().collect::<Vec<_>>().join(" ")*/

    // 解法二:
    // .split_ascii_whitespace():将字符串s按照ASCII空白字符(如空格、制表符、换行符等)进行分割,返回一个迭代器,其中每个元素都是原始字符串中的一个单词
    // .rev():将迭代器中所有元素的顺序反转
    // ::<T> 是Rust中用于指定泛型参数或返回类型的语法,也被称为类型提示。此处用于告诉 collect() 方法要收集元素到一个 Vec<&str> 类型的vec中
    // .collect::<Vec<&str>>():将反转后的迭代器元素收集到一个新的vec中并指定了它的返回类型。且每个元素都是一个指向原始字符串中单词的切片(&str)
    // .join(" "):将vec中的所有切片用空格连接起来,形成一个新的字符串
    s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
//-----------------------------------------------------

// 给你一个整数数组 nums，返回 数组 answer ，其中 answer[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积，且不能使用除法
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
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

// 如果存在这样的三元组下标 (i, j, k) 且满足 i < j < k,使得 nums[i] < nums[j] < nums[k],返回 true;否则,返回 false
fn increasing_triplet(nums: Vec<i32>) -> bool {
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

// 题目要求:chars不为空
/// 压缩字符串
fn compress(chars: &mut [char]) -> i32 {
    let n = chars.len();
    if n <= 1 { return n as i32; }

    let (mut idx, mut count) = (0, 1);
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

    // idx as i32
    i32::try_from(idx).unwrap_or_default()
}
//-----------------------------------------------------

fn max_area(height: Vec<i32>) -> i32 {
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

// nums = [3, 1, 3, 4, 3], k = 6
fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
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

// 题目要求:s 由小写英文字母组成且非空
fn max_vowels(s: String, k: usize) -> i32 {
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

// nums = [0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], K = 3
fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
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

// 给定一个二进制数组 nums，需要从中删掉一个元素。请在删掉元素的结果数组中，返回最长的且只包含 1 的非空子数组的长度。
// 如果不存在这样的子数组，请返回 0
fn longest_subarray(nums: Vec<i32>) -> i32 {
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

// 题目要求:字符串中的内容全是小写字母
// 如果可以使用以下操作从一个字符串得到另一个字符串，则认为两个字符串'接近':
// 操作1:交换任意两个 现有 字符。
// 例: abcde -> aecdb
// 操作2:将一个 现有 字符的每次出现转换为另一个 现有 字符，并对另一个字符执行相同的操作。
// 例如，aacabb -> bbcbaa (所有 a 转化为 b,而所有的 b 转换为 a)
// 你可以根据需要对任意一个字符串多次使用这两种操作。
// 给你两个字符串，word1 和 word2。如果 word1 和 word2 接近,就返回 true;否则，返回 false
// 输入：word1 = "cabbba", word2 = "abbccc"
// 输出：true
// 解释：3 次操作从 word1 获得 word2 。
// 执行操作 1："cabbba" -> "caabbb"
// 执行操作 2："caabbb" -> "baaccc"
// 执行操作 2："baaccc" -> "abbccc"
// 这个函数的目标是判断两个字符串是否“接近”。
// 具体来说即是：两个字符串长度相同，并且对于每个字母，两个字符串中该字母的出现次数相同（无论顺序如何）
fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() { return false; }

    let mut word1_cnt = [0; 26]; // 字符计数:用于存储字符串中每个字母的出现次数
    // 显式循环:编译器可能更容易对其进行优化,因为它直接反映了循环的意图，没有额外的抽象层。
    /*for c in word1.as_bytes() {
        word1_cnt[(c - b'a') as usize] += 1;
    }*/
    // 迭代器链:利用了 Rust 的迭代器抽象,使得代码更加函数式。
    // 虽然迭代器链提供了更多的灵活性(即可以很容易地添加额外的操作到链中)，但也可能引入一些微小的运行时开销，因为每次调用迭代器方法时都可能涉及到一些额外的函数调用。
    // 在某些情况下，编译器可能不如处理显式循环那样优化迭代器链。
    // 总结:迭代器链方式的内存消耗会相对较少，显式循环的运行会较快，但差异非常微小
    word1.as_bytes().iter().for_each(|c| word1_cnt[(c - b'a') as usize] += 1);
    let mut word2_cnt = [0; 26]; // 字符计数
    word2.as_bytes().iter().for_each(|c| word2_cnt[(c - b'a') as usize] += 1);
    // 检查零计数，当含有不同字母时就退出
    for i in 0..26 {
        if (word1_cnt[i] == 0) != (word2_cnt[i] == 0) { return false; }
    }

    // sort:对切片进行稳定的排序，即如果两个元素相等，它们在排序后的相对顺序会保持不变。
    // 由于它保证了稳定性，所以通常比 sort_unstable 慢一些，因为它需要额外的内存来保持元素的相对顺序。
    // sort_unstable:对切片进行排序，如果两个元素相等，它们在排序后的相对顺序可能会改变。
    // 由于它不需要保证稳定性，所以通常比 sort 快一些，因为它可以采用更高效的排序算法。
    word1_cnt.sort_unstable();
    word2_cnt.sort_unstable();
    word1_cnt == word2_cnt
}
//-----------------------------------------------------

fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let (mut cnt, mut cache_map) = (0, HashMap::new());
    // 遍历行,将其作为 key，行出现的次数为 value 存入 HashMap
    grid.iter().for_each(|g| *cache_map.entry(g).or_insert(0) += 1);
    // 遍历列,找到与之匹配的行，累加对应的计数
    for i in 0..grid.len() {
        let curr: Vec<i32> = (0..grid.len()).map(|j| grid[j][i]).collect();
        cnt += cache_map.get(&curr).unwrap_or(&0);
    }
    cnt

    // 解法二:
    /*let n = grid.len();
    let mut column_vec = vec![0; n];
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            column_vec[j] = grid[j][i];
        }
        cnt += grid.iter().filter(|&x| x == &column_vec).count();
    }
    cnt as i32*/
}
//-----------------------------------------------------

// 给你一个包含若干星号 * 的字符串 s
// 在一步操作中，可以:选中 s 中的一个星号。
// 移除星号 左侧 最近的那个 非星号 字符,并移除该星号自身。返回移除 所有 星号之后的字符串。
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

    // 解法二:
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

// asteroids = [10,2,-5]
// 输出：[10]
// 解释：2 和 -5 发生碰撞后剩下 -5 。10 和 -5 发生碰撞后剩下 10 。
fn asteroid_collision(mut asteroids: Vec<i32>) -> Vec<i32> {
    // 数组原地模拟，使用index表示当前最右(也就是最先接受碰撞的节点索引)行星，只需要寻找可能发生碰撞的行星，即运动方向不同的,
    // 遍历数组：
    // 1.如果当前行星向右(尝试找下一个向右的行星)，或者index位于初始位置，或者当前行星和最右行星都向左，将index + 1，并将行星位置前移;
    // 2.否则，判断最右行星与当前行星的负值大小(必须向左)，如果：
    //   二者相等，则双双抵消，最右行星索引移动到前一个数组位置即可;
    //   最右行星的正数值大，相当于碰撞无效，不需要替换行星，不做处理即可;
    //   当前行星的负值较大，说明碰撞后留下的是当前行星，将当前索引和最右行星同时回拨;
    // 最后返回原数组的[0, (index + 1).max(0)]范围内的元素。
    let (mut index, mut i): (i32, usize) = (-1, 0);
    while i < asteroids.len() {
        if asteroids[i] > 0 || index == -1 || asteroids[index as usize] < 0 {
            index += 1;
            asteroids[index as usize] = asteroids[i];
        } else if asteroids[index as usize] <= -asteroids[i] {
            if asteroids[index as usize] < -asteroids[i] {
                i -= 1;
            }
            index -= 1;
        }
        i += 1;
    }

    asteroids[0..(index + 1).max(0) as usize].to_vec()
}
//-----------------------------------------------------

// 题目要求:原始数据不包含数字,所有的数字只表示重复的次数 k,例:不会出现像 3a 或 2[4] 的输入
//         s 中所有整数的取值范围为 [1, 300]
fn decode_string(s: String) -> String {
    // 栈操作
    let mut stack = Vec::new();
    let mut curr_num = 0;
    let mut curr_str = String::new();
    // let (mut curr_num, mut curr_str) = (0, String::new()); // 内存比较(完全相同)

    // "3[a12[c]]" ----> "accccccccccccaccccccccccccacccccccccccc"
    for c in s.chars() {
        match c {
            '0'..='9' => {
                // curr_num = curr_num * 10 + (c as u8 - '0' as u8) as usize; // '0' as u8 是 48
                curr_num = curr_num * 10 + (c as usize - '0' as usize);
            }
            '[' => {
                stack.push((curr_str, curr_num));
                curr_str = String::new();
                // 这个操作创建了一个新的空字符串,并将curr_str的引用更新为指向这个新字符串。
                // 这涉及到内存分配(尽管分配的是一个非常小的字符串)，并且如果之前的String是堆上分配的，那么它的内存也会被回收。
                // 这种方式的优点是它确保了curr_str不再保留任何不必要的内存，但缺点是涉及到内存分配和可能的垃圾回收，这通常比简单的标记为空要慢一些。

                // curr_str.clear();
                // 这个操作仅仅将字符串的内部缓冲区标记为空,它并不会释放分配的内存。
                // 即如果字符串之前占用了大量内存,那么即使调用clear()之后,该内存仍然被String保留。
                // 这样做的优点是操作很快，因为不涉及任何内存分配或释放。
                // 如果之后String又需要存储数据,它可以在已经分配的内存上进行操作,这通常比重新分配内存要快。
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

fn predict_party_victory(senate: String) -> String {
    /*let mut radiant = VecDeque::new();
    let mut dire = VecDeque::new();
    for (i, ch) in senate.char_indices() {
        match ch {
            'R' => radiant.push_back(i),
            _ => dire.push_back(i),
        }
    }
    let n = senate.len();
    loop {
        match (radiant.pop_front(), dire.pop_front()) {
            (Some(x), Some(y)) => match x < y {
                true => radiant.push_back(x + n),
                false => dire.push_back(y + n),
            },
            (Some(_), None) => return "Radiant".to_string(),
            _ => return "Dire".to_string(),
        }
    }*/

    // 解法二:上面的优化版
    let mut rd = [VecDeque::new(), VecDeque::new()];
    for (i, c) in senate.chars().enumerate() {
        rd[usize::from(c == 'D')].push_back(i);
    }
    let n = senate.len();
    loop {
        match (rd[0].pop_front(), rd[1].pop_front()) {
            (Some(r), Some(d)) => rd[usize::from(r > d)].push_back(if r > d { r } else { d } + n),
            (Some(_r), None) => break "Radiant".to_string(),
            (None, Some(_d)) => break "Dire".to_string(),
            _ => ()
        }
    }
}
//-----------------------------------------------------

// 有 n 个房间，房间按从 0 到 n - 1 编号。最初，除 0 号房间外的其余所有房间都被锁住。目标是进入所有的房间，但不能在没有获得钥匙的时候进入锁住的房间。
// 当进入一个房间，可能会在里面找到一套不同的钥匙，每把钥匙上都有对应的房间号，即表示钥匙可以打开的房间。可以拿上所有钥匙去解锁其他房间。
// 给定一个数组 rooms 其中 rooms[i] 是进入 i 号房间可以获得的钥匙集合。如果能进入 所有 房间返回 true，否则返回 false。
fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    // bfs解法
    let mut queue = VecDeque::new();
    let mut room_map = vec![false; rooms.len()];
    queue.push_back(0);
    room_map[0] = true;
    while let Some(curr) = queue.pop_front() {
        rooms[curr].iter().for_each(|&x| {
            let x = x as usize;
            // 未进入过的房间
            if !room_map[x] {
                room_map[x] = true;
                queue.push_back(x);
            }
        });
    }
    room_map.into_iter().all(|x| x)

    // dfs解法
    /*fn dfs(rooms: &Vec<Vec<i32>>, curr: usize, visited: &mut Vec<bool>) -> i32 {
        if visited[curr] { return 0; }
        visited[curr] = true;
        let mut cnt = 1;
        for &r in &rooms[curr] {
            cnt += dfs(rooms, r as usize, visited);
        }
        cnt
    }

    let mut visited = vec![false; rooms.len()];
    dfs(&rooms, 0, &mut visited) == rooms.len() as i32 */
}
//-----------------------------------------------------

/* fn _find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let p = &mut (0..n).collect();
    fn find(x: usize, p: &mut Vec<usize>) -> usize {
        if p[x] != x {
            p[x] = find(p[x], p);
        }
        p[x]
    }

    // Union
    let mut answer = n;
    for i in 0..n {
        for j in i..n {
            if is_connected[i][j] == 1 {
                let (pi, pj) = (find(i, p), find(j, p));
                if pi != pj {
                    p[pj] = pi;
                    answer -= 1;
                }
            }
        }
    }

    answer as i32
} */

// 并查集（Disjoint-Set）是一种数据结构，主要用于管理一组元素的分组情况，并提供合并（Union）和查找（Find）两种基本操作。
// 这种数据结构主要用于解决连通性问题，例如:判断元素是否在同一集合中，并在需要时合并两个集合。用于处理元素分组和连通性问题.

// 解法二:上面的优化版
fn find_circle_num2(is_connected: Vec<Vec<i32>>) -> i32 {
    // Find
    fn find(i: usize, par: &[usize]) -> usize {
        let mut i = i;
        while par[i] != i {
            i = par[i];
        }
        i
    }

    // Union
    let n = is_connected.len();
    let mut answer = n;
    let mut par = vec![0; n];
    for (i, p) in par.iter_mut().enumerate() { *p = i; }
    // .iter_mut() 是用于可变引用集合(例: Vec<T> 或 &mut [T])的方法，它返回一个迭代器，该迭代器产生集合中每个元素的可变引用。
    // .iter_mut() 用于在迭代过程中修改集合的元素，返回的是可变引用迭代器,之后vec集合可以继续使用。
    // let mut vec = vec![1, 2, 3];
    // for item in vec.iter_mut() { *item *= 2; }
    // println!("{:?}", vec); // 输出 [2, 4, 6]
    // .into_iter() 是一个消费性方法，用于将集合转换为所有权转移给迭代器的元素。这通常用于当你不再需要原始集合，并希望将其元素作为迭代器使用时。
    // .into_iter() 用于将集合的所有权转移到迭代器中，通常用于当你不再需要原始集合时。调用.into_iter()后，原始vec集合将不再可用,因为其所有权已被转移到迭代器中。
    // let iter = vec.into_iter();
    // for item in iter { println!("{}", item); }
    // vec 在这里不再可用，因为所有权已转移到 iter

    let mut size = vec![1; n];
    // 使用enumerate()的操作相比上面的is_connected[i][j]操作方式运行效率更高
    for (i, item) in is_connected.into_iter().enumerate() {
        for (j, ic) in item.iter().enumerate().skip(i) {
            if *ic == 1 {
                let root1 = find(i, &par);
                let root2 = find(j, &par);
                if root1 != root2 {
                    answer -= 1;
                    if size[root1] > size[root2] {
                        par[root2] = root1;
                        size[root1] += 1;
                    } else {
                        par[root1] = root2;
                        size[root2] += 1;
                    }
                }
            }
        }
    }

    // answer as i32
    i32::try_from(answer).expect("Err")
}
//-----------------------------------------------------

/// 重新规划路线(深度优先搜索)
// n 座城市,从 0 到 n-1 编号,其间共有 n-1 条路线。因此，要想在两座不同城市之间旅行只有唯一一条路线可供选择(路线网形成一棵树)。
// 去年，交通运输部决定重新规划路线，以改变交通拥堵的状况。
// 路线用 connections 表示，其中 connections[i] = [a, b] 表示从城市 a 到 b 的一条有向路线。
// 今年，城市 0 将会举办一场大型比赛，很多游客都想前往城市 0 。
// 请你帮助重新规划路线方向，使每个城市都可以访问城市 0 。返回需要变更方向的最小路线数。
// 题目数据:保证每个城市在重新规划路线方向后都能到达城市 0
// n = 6, connections: [[0,1], [1,3], [2,3], [4,0], [4,5]]
fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
    for e in &connections {
        let (a, b) = (e[0] as usize, e[1] as usize);
        g[a].push((b as i32, 1));
        g[b].push((a as i32, 0));
    }

    fn dfs(a: usize, fa: i32, g: &Vec<Vec<(i32, i32)>>) -> i32 {
        let mut answer = 0;
        for &(b, c) in g[a].iter() {
            if b != fa {
                answer += c + dfs(b as usize, a as i32, g);
            }
        }

        answer
    }

    dfs(0, -1, &g)
}
//-----------------------------------------------------

// 给定一个变量对数组 equations 和一个实数值数组 values 作为已知条件，其中 equations[i] = [Ai, Bi] 和 values[i] 共同表示等式 Ai / Bi = values[i] 。每个 Ai 或 Bi 是一个表示单个变量的字符串。
// 另有一些以数组 queries 表示的问题，其中 queries[j] = [Cj, Dj] 表示第 j 个问题，请根据已知条件找出 Cj / Dj = ? 的结果作为答案。
// 返回 所有问题的答案 。如果存在某个无法确定的答案，则用 -1.0 替代这个答案。如果问题中出现了给定的已知条件中没有出现的字符串，也需要用 -1.0 替代这个答案。
// 注意:输入总是有效的。可以假设除法运算中不会出现除数为 0 的情况，且不存在任何矛盾的结果。
// 注意:未在等式列表中出现的变量是未定义的，因此无法确定它们的答案。
fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    /*
    // graph1:将图转化为邻接矩阵
    let mut graph1 = HashMap::new();
    let n = equations.len();
    for i in 0..n {
        graph1.entry(equations[i][0].clone()).or_insert(HashMap::new()).entry(equations[i][1].clone()).or_insert(values[i]);
        graph1.entry(equations[i][1].clone()).or_insert(HashMap::new()).entry(equations[i][0].clone()).or_insert(1_f64 / values[i]);
    }

    // graph2:每个变量的分组情况，不同组之间没有通路
    let mut graph2 = HashMap::new();
    for key in graph1.keys() {
        graph2.insert(key, 0);
    }
    // unchecked:尚未分组的变量数
    let mut unchecked = graph2.len();
    // secnum:组别数量
    let mut secnum = 0;
    // graph3:每个变量的虚拟值(用于计算)
    let mut graph3 = HashMap::new();
    for key in graph1.keys() {
        if graph2.get(key) == Some(&0) {
            graph3.entry(key).or_insert(1_f64);
            secnum += 1;
            graph2.insert(key, secnum);
            let mut keys = vec![key];
            while keys.len() > 0 {
                let mut keys_n = vec![];
                for item in keys.clone() {
                    for key_n in graph1.get(item).unwrap().keys() {
                        if graph2.get(&key_n) == Some(&0) {
                            graph2.insert(key_n, secnum);
                            unchecked -= 1;
                            let r = graph3.get(item).unwrap() * graph1.get(key_n).unwrap().get(item).unwrap();
                            graph3.entry(key_n).or_insert(r);
                            keys_n.push(key_n);
                        }
                    }
                }
                keys = keys_n;
            }
        }
    }

    let mut ans = vec![];
    for query in queries.iter() {
        if graph2.contains_key(&query[0]) && graph2.contains_key(&query[1]) && graph2.get(&query[0]) == graph2.get(&query[1]) {
            ans.push(graph3.get(&query[0]).unwrap() / graph3.get(&query[1]).unwrap());
        } else {
            ans.push(-1_f64);
        }
    }
    ans*/

    let mut graph = HashMap::new();
    let mut ans = Vec::new();
    for i in 0..equations.len() {
        graph.entry(equations[i][0].clone()).or_insert(Vec::new()).push((equations[i][1].clone(), values[i]));
        graph.entry(equations[i][1].clone()).or_insert(Vec::new()).push((equations[i][0].clone(), 1_f64 / values[i]));
    }

    for query in queries {
        if !graph.contains_key(&query[0]) || !graph.contains_key(&query[1]) {
            ans.push(-1_f64);
        } else if query[0] == query[1] {
            ans.push(1_f64);
        } else {
            let mut visited = vec![&query[0]].into_iter().collect::<HashSet<&String>>();
            let mut queue = vec![(&query[0], 1_f64)];
            let mut val = -1_f64;
            while let Some((node, curr)) = queue.pop() {
                let connected_nodes = graph.get(node).unwrap();
                for (conn_node, v) in connected_nodes {
                    if visited.insert(conn_node) {
                        queue.push((conn_node, v * curr));
                        if *conn_node == query[1] {
                            val = v * curr;
                            queue.clear();
                            break;
                        }
                    }
                }
            }

            ans.push(val);
        }
    }

    ans
}
//-----------------------------------------------------

//  BinaryHeap(二叉堆)主要特性:
// 1.自动排序:当你向堆中插入元素时,堆会自动重新排序以确保堆的性质(父节点的值总是大于或等于（最大堆）或小于或等于（最小堆）其子节点的值)得到维护。
// 2.快速访问最高（或最低）优先级元素:堆的根节点(在 BinaryHeap 中，这通常是第一个元素）总是具有最高（或最低，取决于堆的类型)的优先级。因此，可以快速地获取或删除这个元素。
// 3.性能:插入和删除堆顶元素的平均时间复杂度是 O(log n)，其中 n 是堆中元素的数量。这使得 BinaryHeap 在处理大量数据时非常高效。
// 4.泛型:BinaryHeap 是泛型的，即是可以用它来存储任何实现了 Ord trait(即可以排序)的类型。

/// 迷宫出口(BFS广度优先搜索)
// maze[i][j] 要么是 '.' ，要么是 '+'
fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let dir = [-1, 0, 1, 0, -1]; // 方向
    let entrance = (entrance[0], entrance[1]); // 入口位置
    let n = maze.len() as i32;    // 行数
    let m = maze[0].len() as i32; // 列数
    // BinaryHeap(二叉堆)，主要用于处理那些需要优先队列特性的场景。
    // 二叉堆通常用于实现优先队列,其中每个元素都有一个“优先级”,并且队列按照优先级(而不是元素插入的顺序)来对元素进行排序。
    let mut bh = BinaryHeap::new();
    // 将入口位置及其步数 0 推入 bh 队列
    bh.push((0, entrance));

    // 优先级由 cnt(即从入口开始到当前单元格的步数) 决定,用作路径长度的计数器counter
    while let Some((cnt, curr)) = bh.pop() {
        // 尝试往4个方向移动
        for i in 0..4 {
            let x = curr.0 + dir[i];
            let y = curr.1 + dir[i + 1];
            // 如果移动后的位置在迷宫范围外,且当前位置不是入口,则返回当前步数的相反数(因为步数是从0开始的,所以其相反数实际上是负的路径长度,表示无法找到出口)。
            // 如果当前位置是入口,则继续处理其他方向。
            if x < 0 || x >= n || y < 0 || y >= m {
                if curr != entrance { return -cnt; } else { continue; }
            }
            let (xx, yy) = (x as usize, y as usize);
            // 如果移动后的位置在迷宫范围内且是可通过的(即字符为 '.'),则将该位置推入队列,并将其步数减1(表示离入口更近了一步)。
            // 同时将已访问的单元格标记为 '+',以避免重复访问
            if maze[xx][yy] == '.' {
                bh.push((cnt - 1, (x, y)));
                maze[xx][yy] = '+';
            }
        }
    }

    -1
}

// 解法二:适合迷宫规模较小的情况
fn _nearest_exit2(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let mut clones = VecDeque::from_iter([(entrance[0] as usize, entrance[1] as usize)]);
    maze[entrance[0] as usize][entrance[1] as usize] = 'x';
    let mut n_step = 0;
    let mut n_this_clone = 1;
    let mut n_next_clone = 0;

    while let Some((i, j)) = clones.pop_front() {
        if i > 0 && maze[i - 1][j] == '.' {
            if i == 1 || j == 0 || j == maze[i].len() - 1 { return n_step + 1; }

            maze[i - 1][j] = 'x';
            clones.push_back((i - 1, j));
            n_next_clone += 1;
        }

        if i + 1 < maze.len() && maze[i + 1][j] == '.' {
            if i + 1 == maze.len() - 1 || j == 0 || j == maze[i].len() - 1 { return n_step + 1; }

            maze[i + 1][j] = 'x';
            clones.push_back((i + 1, j));
            n_next_clone += 1;
        }

        if j > 0 && maze[i][j - 1] == '.' {
            if j - 1 == 0 || i == 0 || i == maze.len() - 1 { return n_step + 1; }

            maze[i][j - 1] = 'x';
            clones.push_back((i, j - 1));
            n_next_clone += 1;
        }

        if j + 1 < maze[i].len() && maze[i][j + 1] == '.' {
            if j + 1 == maze[i].len() - 1 || i == 0 || i == maze.len() - 1 { return n_step + 1; }

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

fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut queue = VecDeque::new();
    let mut cnt = 0;
    // 遍历一遍整个网格，统计出新鲜橘子的数量，记为 cnt，并且将所有腐烂的橘子的坐标加入队列 queue 中。
    // 相比使用grid[i][j]的操作方式执行效率更高，内存消耗更低
    for (i, item) in grid.iter().enumerate() {
        for (j, g) in item.iter().enumerate() {
            if *g == 1 {
                cnt += 1;
            } else if *g == 2 {
                queue.push_back(vec![i as i32, j as i32]);
            }
        }
    }

    // bfs 操作
    let dirs: [i32; 5] = [-1, 0, 1, 0, -1]; // 4个方向
    let mut answer = 0;
    // 每一轮(每分钟)搜索，将队列中的所有腐烂的橘子向四个方向腐烂新鲜橘子，直到队列为空或者新鲜橘子的数量为 0 为止。
    while !queue.is_empty() && cnt > 0 {
        let q_size = queue.len();
        for _ in 0..q_size {
            let p = queue.pop_front().unwrap();
            for d in 0..4 {
                let x = p[0] + dirs[d];
                let y = p[1] + dirs[d + 1];
                if x >= 0 && x < (m as i32) && y >= 0 && y < (n as i32) && grid[x as usize][y as usize] == 1 {
                    grid[x as usize][y as usize] = 2;
                    queue.push_back(vec![x, y]);
                    cnt -= 1;
                }
            }
        }
        answer += 1;
    }

    // 如果新鲜橘子的数量为 0，则返回当前的轮数，否则返回 −1
    if cnt > 0 { return -1; }
    answer
}
//-----------------------------------------------------

/// 数组中的第k个最大元素(例：k=1(即最大的元素))
fn find_kth_largest(mut nums: Vec<i32>, k: usize) -> i32 {
    // 要找的是第 k 大的元素，即目标位置是排序后的数组长度减去 k
    let target_pos = nums.len() - k;
    // select_nth_unstable() 从重新排序的切片中返回一个三元组：索引前的子切片的引用、索引处的元素的引用 和 索引后的子切片的引用。
    // 注:select_nth_unstable() 方法可能并不会保持原始数组的排序，它只是一个快速选择算法的实现，用于在未排序的数组中查找第 n 个最小元素。
    // 如果你的目的是查找第 k 大的元素，且不在乎算法是否保持排序。
    *nums.select_nth_unstable(target_pos).1
}
//-----------------------------------------------------

// 给定两个下标从 0 开始的整数数组 nums1 和 nums2，两者长度都是 n，再给一个正整数 k。必须从 nums1 中选一个长度为 k 的 子序列 对应的下标。
// 对于选择的下标 i0 ，i1 ，...， ik - 1 ，你的 分数 定义如下：
// nums1 中下标对应元素求和，乘以 nums2 中下标对应元素的 最小值。
// 用公式表示： (nums1[i0] + nums1[i1] +...+ nums1[ik - 1]) * min(nums2[i0] , nums2[i1], ... ,nums2[ik - 1]) 。
// 返回 最大 可能的分数。
// 一个数组的 子序列 下标是集合 {0, 1, ..., n-1} 中删除若干元素得到的剩余集合，也可以不删除任何元素。
// 输入：nums1 = [1,3,3,2], nums2 = [2,1,3,4], k = 3
// 输出：12
// 解释：
// 四个可能的子序列分数为：
// - 选择下标 0 ，1 和 2 ，得到分数 (1+3+3) * min(2,1,3) = 7
// - 选择下标 0 ，1 和 3 ，得到分数 (1+3+2) * min(2,1,4) = 6
// - 选择下标 0 ，2 和 3 ，得到分数 (1+3+2) * min(2,3,4) = 12
// - 选择下标 1 ，2 和 3 ，得到分数 (3+3+2) * min(1,3,4) = 8
// 所以最大分数为 12
fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: usize) -> i64 {
    /*let n = nums1.len();
    let mut ids = (0..n).collect::<Vec<_>>();
    // 对下标排序
    ids.sort_unstable_by(|&i, &j| nums2[j].cmp(&nums2[i]));

    let mut h = BinaryHeap::new();
    let mut sum = 0;
    for i in 0..k {
        // 类型转换:使用 i64::from(x) 比 x as i64 更安全,但是消耗内存略高
        sum += i64::from(nums1[ids[i]]);
        h.push(-nums1[ids[i]]); // 转换成最小堆
    }

    let mut answer = sum * i64::from(nums2[ids[k - 1]]);
    for i in k..n {
        let x = nums1[ids[i]];
        if x > -h.peek().unwrap() {
            sum += i64::from(x + h.pop().unwrap());
            h.push(-x);
            answer = answer.max(sum * i64::from(nums2[ids[i]]));
        }
    }
    answer*/

    // 解法二:
    // 合并nums1、 nums2得到数组vec，并按照nums2元素的大小对vec进行降序排序。
    let mut vec: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
    vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    let mut heap = BinaryHeap::<Reverse<i32>>::new();
    // 按照vec的顺序计算前k个nums1元素和sum，并且将其放入最小堆中，并计算出迭代前初始答案answer
    let mut sum: i64 = vec[..k].iter().map(|p| {
        heap.push(Reverse(p.0));
        i64::from(p.0)
    }).sum();
    let mut answer = sum * i64::from(vec[k - 1].1);

    // 迭代更新answer
    for (x, y) in &vec[k..] {
        if *x > heap.peek().unwrap().0 {
            sum += i64::from(*x - heap.pop().unwrap().0);
            heap.push(Reverse(*x));
            answer = answer.max(sum * i64::from(*y));
        }
    }

    answer
}
//-----------------------------------------------------

/// 堆/优先队列
// 题目:给你一个下标从 0 开始的整数数组 costs,其中 costs[i] 是雇佣第 i 位工人的代价。
// 同时给你两个整数 k 和 candidates。我们想根据以下规则恰好雇佣 k 位工人：
// 总共进行 k 轮雇佣，且每一轮恰好雇佣一位工人。
// 在每一轮雇佣中，从最前面 candidates 和最后面 candidates 人中选出代价最小的一位工人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
// 例: costs = [3,2,7,7,1,2] 且 candidates = 2,第一轮雇佣中，我们选择第 4 位工人，因为他的代价最小 [3,2,7,7,1,2]。
// 第二轮雇佣，我们选择第 1 位工人，因为他们的代价与第 4 位工人一样都是最小代价，而且下标更小，[3,2,7,7,2]。注意每一轮雇佣后，剩余工人的下标可能会发生变化。
// 如果剩余员工数目不足 candidates 人，那么下一轮雇佣他们中代价最小的一人，如果有多位代价相同且最小的工人，选择下标更小的一位工人。
// 一位工人只能被选择一次。
// 返回雇佣恰好 k 位工人的总代价。
fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let (k, candidates) = (k as usize, candidates as usize);
    // println!("costs ----> {costs:?}"); // [17, 12, 10, 2, 7, 2, 11, 20, 8]
    if 2 * candidates + k > n {
        // costs.sort_unstable();
        // costs.select_nth_unstable(k - 1);
        // println!("cost sort ----> {costs:?}"); // [2, 2, 7, 8, 10, 11, 12, 17, 20]
        // return costs.iter().take(k).map(|&x| x as i64).sum(); // [2, 2, 7] 即 11
        let (l, m, _g) = costs.select_nth_unstable(k - 1);
        return l.iter().map(|&x| x as i64).sum::<i64>() + *m as i64;
        // println!("lesser ----> {l:?}");  // [2, 2]
        // println!("median ----> {m}");    // 7
        // println!("greater ----> {g:?}"); // [8, 10, 11, 12, 17, 20]
    }

    // println!("costs ----> {costs:?}"); // [17, 12, 10, 2, 7, 20, 11, 2, 8, 28, 11, 28]
    // 排序后                              // [2, 2, 7, 8, 10, 11, 11, 12, 17, 20, 28, 28]
    // 目的是通过不用完全排序而取出合适的值
    let (mut prev, mut suff) = (BinaryHeap::new(), BinaryHeap::new());
    // Reverse() 用于反转排序顺序的结构体
    // let mut numbers = vec![1, 3, 2];
    // numbers.sort_by_key(|&x| Reverse(x));
    // println!("numbers: {numbers:?}"); // [3, 2, 1]
    // Reverse() 用于逆序存储成本值，可以使 BinaryHeap 按照降序的方式排列，从而可以从堆的顶部取出最大的成本值
    for i in 0..candidates {
        prev.push(Reverse(costs[i])); // 前 candidates 个成本值放入 prev 堆中,并逆序放入(通过 Reverse 结构)
        suff.push(Reverse(costs[n - 1 - i])); // 最后 candidates 个成本值放入 suff 堆中,并逆序放入
    }
    // println!("prev ----> {prev:?}"); // [Reverse(2), Reverse(10), Reverse(12), Reverse(17)]
    // println!("suff ----> {suff:?}"); // [Reverse(8), Reverse(11), Reverse(28), Reverse(28)]
    // 双指针操作
    let (mut i, mut j) = (candidates, n - candidates - 1); // 4 7
    (0..k).map(|_| {
        // .peek() 取出堆中的最大值，由于使用Reverse，所以取出的反而是最小值
        let (p, s) = (prev.peek().unwrap().0, suff.peek().unwrap().0);
        // println!("p ----> {p}"); // 2 7 10
        // println!("s ----> {s}"); // 8 8 8
        if p <= s {
            prev.pop();
            // println!("i: {i}"); // 4 5
            prev.push(Reverse(costs[i]));
            i += 1;
            // println!("i: {i}"); // 5 6
            // println!("prev ----> {prev:?}");
            // [Reverse(7), Reverse(10), Reverse(12), Reverse(17)]
            // [Reverse(10), Reverse(11), Reverse(12), Reverse(17)]
            i64::from(p)
        } else {
            suff.pop();
            // println!("j: {j}"); // 7
            suff.push(Reverse(costs[j]));
            j -= 1;
            // println!("j: {j}"); // 6
            // println!("suff ----> {suff:?}");
            // [Reverse(2), Reverse(11), Reverse(28), Reverse(28)]
            i64::from(s)
        }
    }).sum()
}
//-----------------------------------------------------

/// 成功对数(二分法查找)
fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
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

/// 寻找峰值元素(二分法查找)
// 峰值元素是指其值严格大于左右相邻值的元素。
// 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
fn find_peak_element(nums: Vec<i32>) -> i32 {
    // 双指针
    let (mut left, mut right) = (0, nums.len() - 1);
    // 二分搜索
    while left < right {
        // (right - left >> 1) 将这个宽度右移一位，相当于将宽度除以2(在二进制中,右移一位等同于除以2的整数部分).
        // 这里的作用是找到搜索范围的中间点，同时避免了整数除法运算，从而提高了效率。
        let middle = left + ((right - left) >> 1); // 计算中间索引
        match nums[middle] > nums[middle + 1] {
            true => right = middle,
            false => left = middle + 1,
        }
    }
    // 当left和right相等，此时就找到了峰值元素的索引。
    i32::try_from(left).unwrap_or_default()

    // 解法二:
    // max_by_key() 返回指定函数中给出最大值的元素。如果多个元素的最大值相等,则返回最后一个元素。如果迭代器为空,则返回None。
    // max_by_key(|(_, &v)| v) 元组的第一个元素（即索引），并返回元组的第二个元素（即值）的引用。
    // nums.iter().enumerate().max_by_key(|(_, &v)| v).unwrap().0 as i32
}
//-----------------------------------------------------

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
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
    let (mut left, mut right) = (1, 1000000000); // 假设一个二分法的上界
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

/// 回溯问题
fn letter_combinations(digits: String) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();
    let mut value: Vec<char> = Vec::new();
    match digits.is_empty() {
        true => (),
        false => get_letters(&digits, 0, &mut value, &mut answer),
    }

    answer
}

/// backtrack(回溯操作)
// digits:输入的字符串, index:当前的索引, value:用于存储当前字母组合的Vec<char>, answer:用于存储所有结果的Vec<String>
fn get_letters(digits: &String, index: usize, value: &mut Vec<char>, answer: &mut Vec<String>) {
    if index >= digits.len() {
        // let s = String::from_iter(value.iter()); // 将一个字符迭代器转换为一个字符串
        let s = value.iter().collect(); // 将一个字符迭代器转换为一个字符串
        // let value = vec!['a', 'b', 'c'];
        // let s = value.iter().collect(); // "abc"
        answer.push(s);
        return;
    }
    // .iter().nth(n) 返回迭代器的第n个元素
    // 注:所有前面的元素以及返回的元素都将从迭代器中消耗掉。即前面的元素将被丢弃，并且在同一迭代器上多次调用第n(0)个元素将返回不同的元素。
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
        get_letters(digits, index + 1, value, answer);
        value.pop();
    }
}
//-----------------------------------------------------

// 找出所有相加之和为 n 的 k 个数的组合，且满足下列条件：
// 只使用数字[1, 9] 且 每个数字最多使用一次
// 返回 所有可能的有效组合的列表。该列表不能包含相同的组合两次，组合可以以任何顺序返回。
fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    // 回溯函数:实现回溯算法。回溯算法常用于解决组合问题，它通过递归和剪枝的方式找出所有可能的解。
    /// answer:用于存储所有满足条件的组合的向量。
    ///   curr:当前正在构建的组合。
    ///      i:当前可选取的最大正整数。
    ///      k:还需要找出多少个正整数。
    ///      n:当前组合还需要凑足的和。
    fn backtrace(answer: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>, i: i32, k: i32, n: i32) {
        let c = k - curr.len() as i32;
        // 剪枝条件:用于提前终止递归,这个条件基于组合数学中的公式,用于确定当前情况下是否还有可能找到一个满足条件的组合。
        if n < 0 || n > (i * 2 - c + 1) * c / 2 { return; }
        // 递归终止条件
        if c == 0 {
            answer.push(curr.clone());
            return;
        }
        // 回溯过程
        for j in (1..=i).rev() {
            if j < c { break; }
            curr.push(j);
            backtrace(answer, curr, j - 1, k, n - j);
            curr.pop();
        }
    }

    let mut answer = vec![];
    backtrace(&mut answer, &mut vec![], 9, k, n);
    answer
}
//-----------------------------------------------------

// 假设你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，
// 如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
// 给定一个代表每个房屋存放金额的非负整数数组，计算 不触动警报装置的情况下，一夜之内能够偷窃到的最高金额。
// 输入：[2, 7, 9, 3, 1]
// 输出：12
// 解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
//      偷窃到的最高金额 = 2 + 9 + 1 = 12 。
fn rob(nums: Vec<i32>) -> i32 {
    nums.iter().skip(1).fold((nums[0], 0), |dp, &n| (dp.0, dp.0).max((dp.1 + n, dp.0))).0
}
//-----------------------------------------------------

/// 多米诺和托米诺平铺(动态规划_一维)
// 有两种形状的瓷砖:一种是 2 x 1 的多米诺形，另一种是形如 "L" 的托米诺形。两种形状都可以旋转。
// 给定整数 n ，返回可以平铺 2 x n 的面板的方法的数量。返回对 10的9次方 + 7 取模 的值。
// 平铺指的是每个正方形都必须有瓷砖覆盖。两个平铺不同，当且仅当面板上有四个方向上的相邻单元中的两个，使得恰好有一个平铺有一个瓷砖占据两个正方形。
fn num_tilings(n: i32) -> i32 {
    (1..n).fold((0, 1, 1, 1e9 as i32 + 7), |(a, b, c, m), _| (b, c, (2 * c % m + a) % m, m)).2
}
//-----------------------------------------------------

/// 不同路径(动态规划_多维_网格路径),矩阵dp空间优化
// 一个机器人位于一个 m x n 网格的最左上角（标记为 “Start” ）。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的最右下角（标记为 “Finish” ）。
// 问总共有多少条不同的路径？
fn unique_paths(m: i32, n: i32) -> i32 {
    // 动态规划方法:这里利用了问题的子问题重叠性质，通过计算并保存子问题的解来避免重复计算，从而提高了效率。
    // 对于较大的 m 和 n 值，这种方法比直接计算组合数更加高效。
    // dp关系: dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
    let n = n as usize;
    // 初始化一个长度为 n 的一维数组 dp，其中 dp[j] 表示到达第 j 列的最后一行（即第 m-1 行）有多少种不同的路径。
    // 由于只能向右或向下移动，到达第一行的每个点都只有一条路径（即一直向右移动），所以 dp 数组的所有元素初始值都设为 1。
    let mut dp = vec![1; n];
    // 通过两层循环来计算到达每个点的不同路径数。
    // 外层循环遍历每一行（从第 1 行到第 m-1 行），内层循环遍历每一列（从第 1 列到第 n-1 列）。
    // 对于每个点 (i, j)，其路径数等于其上方点 (i-1, j) 的路径数加上其左方点 (i, j-1) 的路径数，即 dp[j] = dp[j] + dp[j - 1]。
    // 注:第一列的每个点的路径数始终为 1（因为只能一直向下移动），所以在内层循环开始前，先将 dp[0] 设为 1。
    /*for _ in 1..(m as usize) {
        dp[0] = 1;
        for j in 1..n { dp[j] += dp[j - 1]; }
    }*/
    // 注:使用 for_each() 可能会略微增加一些额外的开销，因为闭包的创建和调用通常会比直接的for循环略慢。
    // 但这个差异通常是非常微小的，除非在性能非常关键的场景下，否则这种差异通常可以忽略不计。
    // 此外，Rust编译器的优化器也会对这两种形式进行相似的优化，使得它们在实际运行时的性能非常接近。
    (1..m as usize).for_each(|_| {
        dp[0] = 1;
        (1..n).for_each(|j| dp[j] += dp[j - 1]);
    });
    dp[n - 1]

    // 解法二:
    // 当 m 和 n 值较小时采用计算组合数方式更高效。
    // 使用数学公式计算组合数学中的组合数（Combination）。
    // 从 (0, 0) 到 (m-1, n-1) 的路径，总共需要走 m+n-2 步，其中 m-1 步是向右，n-1 步是向下。
    // 因此，问题转化为从 m+n-2 步中选择 m-1 步向右走，剩下的自然是向下走。
    // let n = n as u64 - 1;
    // (1..m as u64).fold(1, |cnt, x| cnt * (n + x) / x) as i32
}
//-----------------------------------------------------

/// 动态规划
// 给定两个字符串 text1 和 text2，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。
// 一个字符串的 子序列 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
// 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。
// 两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。
fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    /*let (m, n) = (text1.len(), text2.len());
    // 因为dp[i][j] 是表示下标(i-1, j-1) 的 最长公共子序列，由于 i / j == 0 都是无意义的,可以初始化为0
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
            // text1.bytes() 会返回一个迭代器，它逐个产生text1中每个字符的字节表示。
            // .nth(i - 1) 会获取迭代器中第i - 1个元素的值。如果i - 1超出了迭代器的范围，它将返回None。
            // text1.as_bytes() 会返回一个指向字符串内部字节数组的slice，这个slice是原始字符串的直接视图，没有额外的迭代器开销。
            // .get(i - 1) 会尝试获取切片中索引为i - 1的元素的可变引用，如果这个索引是有效的，那么它就会返回一个指向该元素的引用。
            // .copied() 会将这个引用转换为对应元素的值（如果存在的话），并产生一个 Option<u8>
            // 在性能上，text1.as_bytes().get(i - 1).copied() 通常会比 text1.bytes().nth(i - 1) 更快，
            // 因为 as_bytes() 是直接访问字符串的内部数据，而 bytes() 则需要在每次调用时生成一个新的迭代器。
            // 迭代器每次调用 nth() 时都需要从当前位置开始重新计算到目标位置，这增加了额外的开销。
            // 因此优先使用 text1.as_bytes().get(i - 1).copied() 来访问字符串的字节。
            dp[i][j] = match text1.as_bytes().get(i - 1).copied() == text2.as_bytes().get(j - 1).copied() {
                true => dp[i - 1][j - 1] + 1,
                false => dp[i - 1][j].max(dp[i][j - 1]),
            }
        })
    });
    dp[m - 1][n - 1]
}
//-----------------------------------------------------

/// 动态规划(最大收益问题)
// 给定一个整数数组 prices，其中 prices[i]表示第 i 天的股票价格; 整数 fee 代表了交易股票的手续费用。
// 你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。
// 返回获得利润的最大值。
// 注意:这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。
fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices.iter().fold((0, -prices[0]), |(sell, buy), p| (sell.max(buy + p - fee), buy.max(sell - p))).0
}
//-----------------------------------------------------

// 给定两个单词 word1 和 word2，请返回将 word1 转换成 word2 所使用的最少操作数。
// 可以对一个单词进行这三种操作：插入一个字符 or 删除一个字符 or 替换一个字符
// 输入：word1 = "intention", word2 = "execution"
// 输出：5
// 解释：
// intention -> inention (删除 't')
// inention -> enention (将 'i' 替换为 'e')
// enention -> exention (将 'n' 替换为 'x')
// exention -> exection (将 'n' 替换为 'c')
// exection -> execution (插入 'u')
fn min_distance(word1: String, word2: String) -> i32 {
    let (m, n) = (word1.len(), word2.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];
    // 初始化 dp 的第一行和第一列为其索引值
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    // 动态规划计算编辑距离
    for (i, c1) in word1.chars().enumerate() {
        for (j, c2) in word2.chars().enumerate() {
            // 由于 if 语句是表达式，可以实现类似三元运算符的效果
            dp[i + 1][j + 1] = if c1 == c2 { dp[i][j] } else { (dp[i][j] + 1).min(dp[i + 1][j] + 1).min(dp[i][j + 1] + 1) };
        }
    }

    i32::try_from(dp[m][n]).unwrap_or_default()

    // 解法二:
    /*let word1_chars = word1.as_bytes();
    let word2_chars = word2.as_bytes();
    let (m, n) = (word1_chars.len(), word2_chars.len());
    let mut result = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                result[i][j] = j as i32;
            } else if j == 0 {
                result[i][j] = i as i32;
            } else {
                result[i][j] = *[result[i - 1][j] + 1, result[i][j - 1] + 1, result[i - 1][j - 1] + if word1_chars[i - 1] == word2_chars[j - 1] { 0 } else { 1 }].iter().min().unwrap();
            }
        }
    }
    result[m][n]*/
}
//-----------------------------------------------------

/// 推荐商品(字典树)
// 给你一个商品数组 products 和一个字符串 searchWord ，products  数组中每个商品都是一个字符串。
// 请你设计一个推荐系统，在依次输入单词 searchWord 的每一个字母后，推荐 products 数组中前缀与 searchWord 相同的最多三个产品。
// 如果前缀相同的可推荐产品超过三个，请按字典序返回最小的三个。
// 请你以二维数组的形式，返回在输入 searchWord 每个字母后相应的推荐商品的列表。
fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut answer = vec![];
    products.sort_unstable();
    // 遍历搜索词的所有可能前缀
    for i in 1..=search_word.len() {
        // let vec = vec![1, 2, 3, 4, 5];
        // let filtered_vec: Vec<i32> = vec.iter().filter(|&x| x % 2 == 0).collect(); // // 保留偶数
        // println!("{:?}", filtered_vec); // 输出 [2, 4], 且原始vec集合保持不变
        // let mut vec = vec![1, 2, 3, 4, 5];
        // vec.retain(|&x| x % 2 == 0); // 保留偶数
        // println!("{:?}", vec); // 输出 [2, 4], 直接修改原始vec集合
        // 修改性: .filter() 创建了一个新的迭代器，不修改原始集合；而 .retain() 直接修改原始集合，删除不满足条件的元素。
        // 返回值：.filter() 返回一个迭代器，你可以用它来创建新的集合；.retain() 没有返回值，因为它直接修改了原始集合。
        // 性能：由于 .retain() 直接在原始集合上操作，避免了创建新集合的开销，因此在某些情况下可能更高效。然而，这也意味着你不能保留原始集合的完整状态。
        // 使用场景：如果需要保留原始集合不变，并创建一个新的集合来包含满足条件的元素，那么应该使用 .filter()。
        //          如果希望直接修改原始集合，删除不满足条件的元素，那么应该使用 .retain()。

        // .retain() 方法用于过滤集合(如vec、slice等)中的元素。即只保留满足特定条件的元素
        // 遍历集合中的每个元素，并根据提供的闭包（或函数）的返回值来决定是否保留该元素。如果闭包返回 true，则保留;如果返回 false，则不要。
        // retain() 方法的一个重要特性是就地操作，即直接在原始vec上修改元素，而不是创建一个新的vec。
        // 这通常比创建一个新vec更高效，尤其是当处理大型数据集时。然而，调用 retain() 后，原始vec将被修改，可能不再包含之前所有的元素。
        // 注意:由于 retain() 方法可能会改变vec的长度，因此调用 retain() 之后，任何依赖于原始vec长度的代码都应该小心处理。
        // 此外,如果闭包内部使用了vec的引用或迭代器，并且这些引用或迭代器在 retain() 调用期间可能变得无效，可能会导致未定义的行为.
        // 此处由于闭包只使用了字符串的本地副本，因此没有这个问题
        products.retain(|s| s.starts_with(search_word.get(0..i).unwrap()));
        // 对过滤后的 products,使用 iter().take(3).cloned().collect() 获取前三个元素（如果存在的话）并将其添加到 answer 中。
        answer.push(products.iter().take(3).cloned().collect());
    }

    answer
}
//-----------------------------------------------------

/// 无重叠区间(区间集合问题)
// 给定一个区间的集合 intervals,其中 intervals[i] = [start, end] 返回 需要移除区间的最小数量,使剩余区间互不重叠
// 输入: intervals = [[1, 2], [2, 3], [3, 4], [1, 3]]
// 输出: 1
// 解释: 移除 [1, 3] 后,剩下的区间没有重叠。
fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    // Vec::is_empty() 通常会比 Vec::len() == 0 执行效率稍微快
    if intervals.len() < 2 { return 0; }

    // intervals.sort_unstable();
    // println!("sort_unstable ----> {intervals:?}"); // [[1, 2], [1, 3], [2, 3], [3, 4]]
    intervals.sort_unstable_by_key(|v| v[1]);
    // println!("sort_unstable_by_key ----> {intervals:?}"); // [[1, 2], [2, 3], [1, 3], [3, 4]]
    // sort_unstable() 方法对 intervals 进行不稳定的就地排序。
    // 它按照元素的自然顺序（对于 Vec<Vec<i32>> 类型，就是按照每个子vec的第一个元素，即 v[0]，进行比较）进行排序。
    // 不稳定排序意味着相等的元素在排序后的相对顺序可能发生变化。
    // intervals 调用 sort_unstable()，它会按照每个子vec的第一个元素进行排序。
    // 如果 intervals 包含 [[1, 3], [2, 4], [1, 2]]，排序后的结果可能是 [[1, 3], [1, 2], [2, 4]]（注意 [1, 3] 和 [1, 2] 的顺序可能互换，因为排序是不稳定的）。
    // sort_unstable_by_key() 方法允许你提供一个闭包（函数对象），它用于提取排序时要使用的键。
    // 在例子中，闭包 |v| v[1] 表示每个子vec的第二个元素（即 v[1]）将用作排序的键。
    // intervals 调用 sort_unstable_by_key(|v| v[1])，它会按照每个子vec的第二个元素进行排序。
    // 如果 intervals 包含 [[1, 3], [2, 4], [1, 2]]，排序后的结果将是 [[1, 2], [1, 3], [2, 4]]，因为排序是基于每个子vec的第二个元素进行的。

    let mut count = 0;
    let mut end = intervals[0][1];
    for v in intervals.iter().skip(1) {
        if v[0] >= end { end = v[1]; } else { count += 1; }
    }

    count
}
//-----------------------------------------------------

// 输入：points = [[10, 16], [2, 8], [1, 6], [7, 12]]
// 输出：2
// 解释：气球可以用2支箭来爆破:
// -在x = 6处射出箭，击破气球[2, 8]和[1, 6]。
// -在x = 11处发射箭，击破气球[10, 16]和[7, 12]。
fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    // 解法一:直接将二维数组按照末尾的数字进行排序，然后比较 区间 是否有重叠部分。
    /*points.sort_unstable_by_key(|p| p[1]);
    points.iter().skip(1).fold((1, points[0][1]), |(cnt, end), x| {
        if x[0] > end {
            (cnt + 1, x[1])
        } else {
            (cnt, end)
        }
    }).0*/

    // 解法二:优化版
    points.sort_unstable_by_key(|p| p[1]);
    let mut cnt = 1;
    let mut p_end = points[0][1];
    for x in points.into_iter().skip(1) {
        let (start, end) = (x[0], x[1]);
        if start > p_end {
            cnt += 1;
            p_end = end;
        }
    }
    cnt
}
//-----------------------------------------------------

// 单调栈（monotone stack）是一种数据结构，其特性是栈内元素（从栈底到栈顶）是单调递增或单调递减的。
// 当新的元素入栈时，会移除栈顶破坏单调性的元素，以确保栈内元素保持单调性。这些出栈的元素在后续操作中不会再次入栈。
// 由于每个元素至多入栈和出栈各一次，因此单调栈的维护时间复杂度是O(n)。
// 单调栈有两种类型：单调递增栈和单调递减栈。单调递增栈意味着栈内元素从栈底到栈顶是递增的，而单调递减栈则是递减的。
// 单调栈常用于解决一些需要找到某个元素左边或右边第一个比它大或小的问题，
// 例如:柱状图中最大的矩形、最长递增子序列等问题。它也可以用于优化某些动态规划问题的求解过程。
// 特别是在解决一些涉及数组或序列的问题时。单调栈的应用广泛，通过合理的设计和使用，可以有效地提高算法的效率。
// 需要注意的是，单调栈中存储的元素可以是数组的值，也可以是数组的下标。

/// 每日温度(单调栈(monotone stack))
// 给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer,其中 answer[i] 是指对于第 i 天，下一个更高温度出现在几天后。
// 如果气温在这之后都不会升高，请在该位置用 0 来代替。
// 输入: temperatures = [73, 74, 75, 71, 69, 72, 76, 73]  // 8
// 输出:                [1, 1, 4, 2, 1, 1, 0, 0]
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut answer = vec![0; temperatures.len()];
    let mut stack = Vec::new();
    for (i, &t) in temperatures.iter().enumerate() {
        // *stack.last().unwrap()
        // stack[stack.len() - 1] 这个运行会稍微快
        while !stack.is_empty() && t > temperatures[stack[stack.len() - 1]] {
            let j = stack.pop().unwrap();
            answer[j] = (i - j) as i32;
        }
        stack.push(i);
    }

    answer
}
//-----------------------------------------------------