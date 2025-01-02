use std::cell::RefCell;
use std::rc::Rc;

use leet_code::algorithm::{
    array_algo, backtracking_algo, binary_search_algo,
    binary_tree_algo::{BSTIterator, TreeNode},
    bit_operation_algo, divide_and_conquer_algo, dynamic_programming_algo, game_theory_algo,
    graph_algo, greedy_algo, hash_algo,
    heap_algo::{self, SmallestInfiniteSet},
    linked_list_algo::ListNode, min_spanning_tree_algo,
    monotone_stack_algo::{self, StockSpanner},
    prefix_sum_algo, queue_algo,
    randomized_algo::{self, Solution, Solution2},
    scan_line_algo, sliding_window_algo, stack_algo, string_algo,
    trie_tree_algo::{self, Trie},
    two_pointers_algo,
};

// 忽略提示含有大量行的函数,超100行即是超量
#[allow(clippy::too_many_lines, clippy::many_single_char_names, clippy::similar_names)]
fn main() {
    println!("------ 前缀和 ------");
    let nums = vec![1, 2, 3, 4, 5, 6];
    let answer = prefix_sum_algo::prefix_sum_ex(nums, 1, 3);
    println!("prefix_sum_ex: {answer}"); // 9

    println!("------ 双指针 ------");
    let nums = [1, 2, 3, 4, 6];
    let target = 6;
    match two_pointers_algo::two_pointer_ex(&nums, target) {
        None => println!("None."),
        Some((index1, index2)) => println!("target: {}, index: ({}, {})", target, index1, index2),
    } // 1, 3

    println!("------ 滑动窗口 ------");
    let nums = [2, 1, 5, 1, 3, 2];
    let answer = sliding_window_algo::sliding_window_ex(&nums, 3);
    println!("sliding_window_ex: {answer}"); // 9

    println!("------ 单调栈 ------");
    let nums = [2, 1, 2, 4, 3];
    let answer = monotone_stack_algo::monotone_stack_ex(&nums);
    println!("monotone_stack_ex: {answer:?}"); // [4, 2, 4, -1, -1]

    // 用于操作显示信息
    if true { return; }

    println!("------ FizzBuzz game ------");
    // 对1到100之间的每个数字进行以下操作：
    // 如果数字是3的倍数，则输出“Fizz”;如果数字是5的倍数，则输出“Buzz”;如果数字既是3的倍数又是5的倍数，则输出“FizzBuzz”;否则，输出该数字本身。
    for x in 1..=100 {
        let (mod_3, mod_5) = (x % 3, x % 5);

        if mod_3 == 0 && mod_5 == 0 {
            print!("FizzBuzz ");
        } else if mod_3 == 0 {
            print!("Fizz ");
        } else if mod_5 == 0 {
            print!("Buzz ");
        } else {
            print!("{x} ");
        }
    };
    println!();

    println!("------ 28.找出字符串中第一个匹配项的下标(字符串匹配) ------");
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
    let answer = string_algo::str_str(haystack, needle);
    println!("str_str: {answer}"); // 0

    println!("------ 1768.交替合并字符串(字符串,双指针) ------");
    let word1 = String::from("abcde");
    let word2 = String::from("xyz");
    let answer = string_algo::merge_alternately(word1, word2);
    println!("merge_alternately: {answer}"); // axbyczde

    println!("------ 1071.字符串的最大公因子(字符串,数学) ------");
    let str1 = String::from("ABABAB");
    let str2 = String::from("AB");
    let answer = string_algo::gcd_of_strings2(str1, str2);
    println!("gcd_of_strings: {answer}"); // AB

    println!("------ 1431.拥有最多糖果的孩子(数组) ------");
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let answer = array_algo::kids_with_candies(candies, extra_candies);
    println!("kids_with_candies: {answer:?}"); // [true, true, true, false, true]

    println!("------ 605.种花问题(数组,贪心) ------");
    // let flowerbed = vec![1, 0, 0, 0, 0, 1];
    let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
    // let flowerbed = vec![0, 1, 0];
    let n = 3;
    let answer = array_algo::can_place_flowers(flowerbed, n);
    println!("can_place_flowers {n}: {answer}"); // true

    println!("------ 345.反转字符串中的元音字母(字符串,双指针) ------");
    let s = "leetcode".to_string();
    // let s = "hello".to_string();
    let answer = string_algo::reverse_vowels(s);
    println!("reverse_vowels: {answer}"); // leotcede

    println!("------ 283.移动零(数组,双指针) ------");
    let mut nums = vec![0, 1, 0, 3, 12];
    // let mut nums = vec![4, 1, 5, 3, 12];
    two_pointers_algo::move_zeroes(&mut nums); // [1, 3, 12, 0, 0]

    println!("------ 392.判断子序列(字符串,双指针,动态规划) ------");
    let s = "ace";
    let t = "abcde";
    println!("Is '{}' a sub of '{}'? {}", s, t, two_pointers_algo::is_subsequence(s.to_string(), t.to_string())); // true

    println!("------ 643.子数组最大平均数Ⅰ(数组,滑动窗口) ------");
    let nums = vec![1, 12, -5, -6, 50, 3];
    let answer = sliding_window_algo::find_max_average(nums, 4);
    println!("find_max_average: {answer}"); // 12.75

    println!("------ 1732.找到最高海拔(数组,前缀和) ------");
    let gain = vec![-5, 1, 5, 0, -7];
    // let gain = vec![-4, -3, -2, -1, 4, 3, 2];
    let answer = prefix_sum_algo::largest_altitude(gain);
    println!("largest_altitude: {answer}"); // 1

    println!("------ 724.寻找数组的中心下标(数组,前缀和) ------");
    let nums = vec![1, 7, 3, 6, 5, 6];
    let answer = prefix_sum_algo::pivot_index(nums);
    println!("pivot_index: {answer}"); // 3

    println!("------ 2215.找出两数组的不同(数组,哈希表) ------");
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 2, 1, 2, 4];
    let answer = hash_algo::find_difference(nums1, nums2);
    println!("find_difference: {answer:?}"); // [[3], [4]]

    println!("------ 1207.独一无二的出现次数(数组,哈希表) ------");
    let arr = vec![1, 2, 2, 1, 1, 3];
    let answer = hash_algo::unique_occurrences(arr);
    println!("unique_occurrences: {answer}"); // true

    println!("------ 933.最近的请求次数(头尾高效操作的队列,数据流) ------");
    let mut recent_counter = queue_algo::RecentCounter::new();
    let ret_1 = recent_counter.ping(1);
    println!("ping: {ret_1}");        // 1
    let ret_2 = recent_counter.ping(100);
    println!("ping: {ret_2}");        // 2
    let ret_3 = recent_counter.ping(3001);
    println!("ping: {ret_3}");        // 3
    let ret_4 = recent_counter.ping(3002);
    println!("ping: {ret_4}");        // 3

    println!("------ 206.反转链表(递归,链表) ------");
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

    println!("------ 104.二叉树的最大深度(dfs,bfs) ------");
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

    println!("------ 872.叶子相似的树(二叉树,dfs) ------");
    let answer = TreeNode::leaf_similar(Some(Rc::new(RefCell::new(rt))), Some(root.clone()));
    println!("leaf_similar: {answer}"); // true

    println!("------ 700.二叉搜索树(BST)中的搜索(二叉搜索树) ------");
    let val = 20;
    let answer = TreeNode::search_bst(Some(root.clone()), val);
    println!("search_bst: {answer:#?}"); // Some(RefCell { value: TreeNode { val: 20, left: Some(RefCell { value: TreeNode { val: 17, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 36, left: None, right: None } }) } })

    println!("----- 450.删除二叉搜索树中的节点(二叉搜索树) ------");
    let answer = TreeNode::delete_node(Some(root.clone()), val);
    println!("delete_node: {answer:#?}");

    println!("------ 374.猜数字大小(二分查找,交互) ------");
    let pick_num = binary_search_algo::guess_number(10);
    println!("guessNumber: {pick_num}"); // 7

    println!("------ 1137.第N个泰波那契数(记忆化搜索,数学,动态规划) ------");
    let n = 25;
    let answer = dynamic_programming_algo::tribonacci(n);
    println!("tribonacci({n}): {answer}"); // 1389537

    println!("------ 746.使用最小花费爬楼梯(数组,动态规划) ------");
    // let cost = vec![10, 15, 20]; // 15
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]; // 6
    let answer = dynamic_programming_algo::min_cost_climbing_stairs(cost);
    println!("min_cost_climbing_stairs: {answer}"); // 6

    println!("----- 509.斐波那契数(记忆化搜索,动态规划) ------");
    let answer = dynamic_programming_algo::fib(28);
    println!("fib: {answer}"); // 317811

    println!("----- 70.爬楼梯(记忆化搜索,动态规划) ------");
    let answer = dynamic_programming_algo::climb_stairs(3);
    println!("climb_stairs: {answer}"); // 3

    println!("------ 338.比特位计数(位运算,动态规划) ------");
    let n = 5;
    let answer = bit_operation_algo::count_bits(n);
    println!("count_bits({n}): {answer:?}"); // [0, 1, 1, 2, 1, 2]

    println!("------ 136.只出现一次的数字(位运算,数组) ------");
    let nums = vec![4, 1, 2, 1, 2];
    let answer = bit_operation_algo::single_number(nums);
    println!("single_number: {answer}"); // 4

    println!("----- 1025.除数博弈(脑筋急转弯,数学,动态规划,博弈) ------");
    let answer = game_theory_algo::divisor_game(2);
    println!("divisor_game: {answer}"); // true

    println!("----- 1979.找出数组的最大公约数(数组,数学,数论) ------");
    let nums = vec![7, 5, 6, 8, 3];
    let answer = array_algo::find_gcd(nums);
    println!("find_gcd: {answer}"); // 1

    println!("----- 1534.统计好三元组(数组,枚举) ------");
    let arr = vec![3, 0, 1, 1, 9, 7];
    let answer = array_algo::count_good_triplets(arr, 7, 2, 3);
    println!("count_good_triplets: {answer}"); // 4

    println!("----- 3042.统计前后缀下标对 I(字典树,字符串匹配,哈希函数,滚动哈希) ------");
    let words = vec!["a".to_string(), "aba".to_string(), "ababa".to_string(), "aa".to_string()];
    let answer = string_algo::count_prefix_suffix_pairs(words);
    println!("count_prefix_suffix_pairs: {answer}"); // 4

    println!("\n-------------up---------------\n");

    println!("------ 151.反转字符串中的单词(字符串,双指针) ------");
    let s = "  a good   example ".to_string();
    let answer = string_algo::reverse_words(s);
    println!("reverse_words: {answer}"); // example good a

    println!("------ 238.除自身以外数组的乘积(数组,前缀和) ------");
    let nums = vec![1, 2, 3, 4];
    let answer = array_algo::product_except_self(nums);
    println!("product_except_self: {answer:?}"); // [24, 12, 8, 6]

    println!("----- 334.递增的三元子序列(贪心,数组) ------");
    let nums = vec![2, 1, 5, 0, 4, 6];
    let answer = greedy_algo::increasing_triplet(nums);
    println!("increasing_triplet: {answer}"); // true

    println!("------ 443.压缩字符串(字符串,双指针) ------");
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    // let mut chars = vec!['a'];
    // let mut chars = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
    let answer = string_algo::compress(&mut chars);
    println!("compress: {answer}"); // 6  ['a', '2', 'b', '2', 'c', '3']

    println!("------ 11.盛最多水的容器(数组,双指针,贪心) ------");
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    // let height = vec![1, 1];
    let max_area = two_pointers_algo::max_area(height);
    println!("Max water: {max_area}"); // 49

    println!("----- 1679.K和数对的最大数目(数组,哈希表,双指针,排序) ------");
    let nums = vec![3, 1, 3, 4, 3];
    let answer = two_pointers_algo::max_operations(nums, 6);
    println!("max_operations: {answer}"); // 1

    println!("------ 1456.定长子串中元音的最大数目(字符串,滑动窗口) ------");
    let s = "abciiidef".to_string();
    let k = 3;
    let answer = sliding_window_algo::max_vowels(s, k);
    println!("max_vowels: {answer}"); // 3

    println!("----- 1004.最大连续1的个数 III(数组,双指针,前缀和,滑动窗口) ------");
    let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k = 3;
    let answer = sliding_window_algo::longest_ones(nums, k);
    println!("longest_ones: {answer}"); // 10

    println!("----- 1493.删掉一个元素以后全为 1 的最长子数组(数组,滑动窗口,动态规划) ------");
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    let answer = sliding_window_algo::longest_subarray(nums);
    println!("longest_subarray: {answer}"); // 5

    println!("------ 1657.确定两个字符串是否接近(字符串,哈希表,计数) ------");
    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    let answer = hash_algo::close_strings(word1, word2);
    println!("close_strings: {answer}"); // true

    println!("----- 2352.相等行列对(数组,哈希,矩阵,模拟) ------");
    let grid = vec![vec![3, 1, 2, 2], vec![1, 4, 4, 5], vec![2, 4, 2, 2], vec![2, 4, 2, 2]];
    let answer = hash_algo::equal_pairs(grid);
    println!("equal_pairs: {answer}"); // 3

    println!("------ 2390.从字符串中移除星号(栈,字符串) ------");
    let s = "leet**cod*e".to_string(); // lecoe
    // let s = String::from("erase*****"); // ""
    let answer = stack_algo::remove_stars(s);
    println!("remove_stars: {answer}");

    println!("----- 735.小行星碰撞(数组,栈,模拟) ------");
    let asteroids = vec![10, 2, -5];
    let answer = stack_algo::asteroid_collision(asteroids);
    println!("asteroid_collision: {answer:?}"); // [10]

    println!("------ 394.字符串解码(栈,字符串,递归) ------");
    let s = "3[a12[c]]".to_string();  // accccccccccccaccccccccccccacccccccccccc
    // let s = "3[a]2[bc]".to_string(); // aaabcbc
    let answer = stack_algo::decode_string(s);
    println!("decode_string: {answer}"); // accccccccccccaccccccccccccacccccccccccc

    println!("----- 649.Dota2 参议院(贪心,队列,字符串) ------");
    let senate = String::from("RDD");
    let answer = greedy_algo::predict_party_victory(senate);
    println!("predict_party_victory: {answer}"); // Dire

    println!("------ 2095.删除链表的中间节点(链表,双指针) ------");
    let node_head = ListNode::delete_middle(node_rev);
    match node_head.clone() {
        None => (),
        Some(node) => node.print_list(),
    } // 5 4 2 1

    println!("----- 328.奇偶链表(链表) ------");
    let odd_even_head = ListNode::odd_even_list(node_head);
    match odd_even_head.clone() {
        None => (),
        Some(node) => node.print_list(),
    } // 5 2 4 1

    println!("----- 2130.链表最大孪生和(链表,栈,双指针) ------");
    let node_head = odd_even_head.clone();
    let answer = ListNode::pair_sum(node_head);
    println!("ListNode::pair_sum: {answer}"); // 6

    println!("----- 1448.统计二叉树中好节点的数目(dfs,bfs) ------");
    let answer = TreeNode::good_nodes(Some(root));
    println!("good_nodes: {answer}"); // 4

    println!("----- 437.二叉树路径总和Ⅲ(二叉树,dfs) ------");
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

    println!("----- 1372.二叉树中的最长交错路径(dfs,动态规划) ------");
    let answer = TreeNode::longest_zig_zag(root.clone());
    println!("longest_zig_zag: {answer}"); // 2

    println!("----- 236.二叉树的最近公共祖先(DFS) ------");
    let p = Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None })));
    let q = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None })));
    let answer = TreeNode::lowest_common_ancestor(root.clone(), p, q);
    println!("lowest_common_ancestor: {answer:#?}");

    println!("----- 199.二叉树的右视图(dfs,bfs) ------");
    let answer = TreeNode::right_side_view(root.clone());
    println!("right_side_view: {answer:?}"); // [10, -3, 11, 1]

    println!("----- 1161.最大层内元素和(dfs,bfs) ------");
    let answer = TreeNode::max_level_sum(root.clone());
    println!("max_level_sum: {answer}"); // 3

    println!("----- LCR 055.二叉搜索树迭代器(迭代器,栈,树,设计) ------");
    let mut bst_iter = BSTIterator::new(root);
    println!("bst_iter.next(): {}", bst_iter.next()); // 10
    println!("bst_iter.next(): {}", bst_iter.next()); // -1
    println!("bst_iter.has_next(): {}", bst_iter.has_next()); // false
    println!("bst_iter.next(): {}", bst_iter.next()); // -1
    // println!("bst_iter.has_next(): {}", bst_iter.has_next()); // false
    // println!("bst_iter.next(): {}", bst_iter.next()); // -1

    println!("----- 841.钥匙和房间(dfs,bfs,图) ------");
    let rooms = vec![vec![1], vec![2], vec![3], vec![]]; // true
    // let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]; // false
    let answer = graph_algo::can_visit_all_rooms(rooms);
    println!("can_visit_all_rooms: {answer}"); // true

    println!("----- 547.省份数量(并查集,图) ------");
    let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let answer = graph_algo::find_circle_num2(is_connected);
    println!("find_circle_num: {answer}"); // 2

    println!("----- 1466.重新规划路线(图,dfs,bfs) ------");
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    let answer = graph_algo::min_reorder(6, connections);
    println!("min_reorder: {answer}"); // 3

    println!("----- 399.除法求值(dfs,bfs,并查集,图,数组,字符串,最短路径) ------");
    let equations = vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()],
                         vec!["bc".to_string(), "cd".to_string()]];
    let values = vec![1.5, 2.5, 5.0];
    let queries = vec![vec!["a".to_owned(), "c".to_owned()], vec!["c".to_owned(), "b".to_owned()],
                       vec!["bc".to_owned(), "cd".to_owned()], vec!["cd".to_owned(), "bc".to_owned()]];
    let answer = graph_algo::calc_equation(equations, values, queries);
    println!("calc_equation: {answer:?}"); // [3.75, 0.4, 5.0, 0.2]

    println!("----- 1926.迷宫中离入口最近的出口(图,bfs) ------");
    let maze = vec![vec!['+', '+', '.', '+'], vec!['.', '.', '.', '+'], vec!['+', '+', '+', '.']];
    let entrance = vec![1, 2];
    let answer = graph_algo::nearest_exit(maze, entrance);
    println!("nearest_exit: {answer}"); // 1

    println!("----- 994.腐烂的橘子(bfs,数组,矩阵) ------");
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let answer = graph_algo::oranges_rotting(grid);
    println!("oranges_rotting: {answer}"); // 4

    println!("----- 215.数组中的第k个最大元素(数组,分治,快速选择,排序,堆(优先队列)) ------");
    let nums = vec![3, 2, 1, 5, 6, 4];
    // let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let answer = divide_and_conquer_algo::find_kth_largest(nums, 2);
    println!("find_kth_largest: {answer}"); // 5

    println!("----- 2336.无限集中的最小数字(堆/优先队列) ------");
    let mut obj = SmallestInfiniteSet::new();
    let ret_1 = obj.pop_smallest();
    obj.add_back(2);
    println!("pop_smallest: {ret_1}"); // 1

    println!("----- 2542.最大子序列的分数(贪心,数组,排序,堆(优先队列)) ------");
    let num1 = vec![1, 3, 3, 2];
    let num2 = vec![2, 1, 3, 4];
    let answer = heap_algo::max_score(num1, num2, 3);
    println!("max_score: {answer}"); // 12

    println!("----- 2462.雇佣k位工人的总代价(数组,双指针) ------");
    let costs = vec![17, 12, 10, 2, 7, 20, 11, 2, 8];       // 11
    // let costs = vec![17, 12, 10, 2, 7, 20, 11, 2, 8, 28, 11, 28]; // 17
    let k = 3;
    let candidates = 4;
    let answer = heap_algo::total_cost(costs, k, candidates);
    println!("total_cost: {answer}"); // 11

    println!("----- 2300.咒语和药水的成功对数(数组,双指针,二分查找) ------");
    let spells = vec![5, 1, 3];
    let potions = vec![1, 2, 3, 4, 5];
    let success = 7;
    let answer = binary_search_algo::successful_pairs(spells, potions, success);
    println!("successful_pairs: {answer:?}"); // [4, 0, 3]

    println!("----- 162.寻找峰值元素的索引(数组,二分查找) ------");
    let nums = vec![1, 6, 7, 5, 6, 8, 8, 8];
    let answer = binary_search_algo::find_peak_element(nums);
    println!("find_peak_element: {answer}"); // 7

    println!("----- 875.爱吃香蕉的珂珂(数组,二分查找) ------");
    let piles = vec![30, 11, 23, 4, 20];
    let h = 6;
    let answer = binary_search_algo::min_eating_speed(piles, h);
    println!("min_eating_speed: {answer}"); // 23

    println!("----- 17.电话号码的字母组合(字符串,哈希表,回溯) ------");
    let digits = String::from("23");
    let answer = backtracking_algo::letter_combinations(digits);
    println!("letter_combinations: {answer:?}"); // ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]

    println!("----- 216.组合总和Ⅲ(数组,回溯) ------");
    let answer = backtracking_algo::combination_sum3(3, 9);
    println!("combination_sum3: {answer:?}"); // [[6, 2, 1], [5, 3, 1], [4, 3, 2]]

    println!("----- 198.打家劫舍(数组,动态规划) ------");
    let nums = vec![2, 7, 9, 3, 1];
    let answer = dynamic_programming_algo::rob(nums);
    println!("rob: {answer}"); // 12

    println!("----- 790.多米诺和托米诺平铺(动态规划) ------");
    let answer = dynamic_programming_algo::num_tilings(3);
    println!("num_tilings: {answer}"); // 5

    println!("----- 62.不同路径(组合数学,动态规划) ------");
    let answer = dynamic_programming_algo::unique_paths(3, 7);
    println!("unique_paths: {answer}"); // 28

    println!("----- 1143.最长公共子序列(字符串,动态规划) ------");
    let answer = dynamic_programming_algo::longest_common_subsequence("abcde".to_string(), "ace".to_string());
    println!("longest_common_subsequence: {answer}"); // 3

    println!("----- 714.买卖股票的最佳时机含手续费(数组,贪心,动态规划) ------");
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee = 2;
    // 解释:能够达到的最大利润:
    // 在此处买入 prices[0] = 1
    // 在此处卖出 prices[3] = 8
    // 在此处买入 prices[4] = 4
    // 在此处卖出 prices[5] = 9
    // 总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8
    let answer = dynamic_programming_algo::max_profit(prices, fee);
    println!("max_profit: {answer}"); // 8

    println!("----- 72.编辑距离(字符串,动态规划) ------");
    let word1 = "intention".to_string();
    let word2 = "execution".to_string();
    let answer = dynamic_programming_algo::min_distance(word1, word2);
    println!("min_distance: {answer}"); // 5

    println!("----- 1318.或运算的最小翻转次数(位运算) ------");
    let answer = bit_operation_algo::min_flips(2, 6, 5);
    println!("min_flips: {answer}"); // 3

    println!("----- 208.实现Trie(前缀树,设计,字典树,哈希表,字符串) ------");
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

    println!("----- 1268.搜索推荐系统(数组,字符串,字典树,二分查找,排序,堆(优先队列)) ------");
    let products = vec!["mobile".to_string(), "mouse".to_string(), "moneypot".to_string(),
                        "monitor".to_string(), "mousepad".to_string()];
    let search_word = "mouse".to_string();
    let answer = trie_tree_algo::suggested_products(products, search_word);
    println!("suggested_products: {answer:#?}"); // [["mobile", "moneypot", "monitor"], ["mobile", "moneypot", "monitor"], ["mouse", "mousepad"], ["mouse", "mousepad"], ["mouse", "mousepad"]]

    println!("----- 435.无重叠区间(数组,贪心算法(greedy algorithm),动态规划) ------");
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    let answer = greedy_algo::erase_overlap_intervals(intervals);
    println!("erase_overlap_intervals: {answer}"); // 1

    println!("----- 452.用最少数量的箭引爆气球(贪心算法(greedy algorithm),数组,排序) ------");
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let answer = greedy_algo::find_min_arrow_shots(points);
    println!("find_min_arrow_shots: {answer}"); // 2

    println!("----- 739.每日温度(栈,数组,单调栈) ------");
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let answer = monotone_stack_algo::daily_temperatures(temperatures);
    println!("daily_temperatures: {answer:?}"); // [1, 1, 4, 2, 1, 1, 0, 0]

    println!("----- 901.股票价格跨度(单调栈,数据流,设计,栈) ------");
    // 设计一个算法收集某些股票的每日报价,并返回该股票当日价格的 跨度。
    // 当日股票价格的 跨度 被定义为股票价格小于或等于今天价格的最大连续日数(从今天开始往回数,包括今天)。
    // 例:如果未来7天股票的价格是 [100, 80, 60, 70, 60, 75, 85],那么股票跨度将是 [1, 1, 1, 2, 1, 4, 6]
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

    println!("----- 470.用 Rand7() 实现 Rand10() (数学,拒绝采样,概率与统计,随机化) ------");
    let answer = randomized_algo::rand10();
    println!("rand10(): {answer}");

    println!("----- 478.在圆内随机生成点(数学,拒绝采样,几何,随机化) ------");
    let solution = Solution::new(1.0, 0.0, 0.0);
    let answer: Vec<f64> = solution.rand_point();
    println!("solution.rand_point(): {answer:?}"); //随机的 [0.7767803745865258, 0.4342820953752826]

    println!("----- LCR 057.存在重复元素 III(数组,桶排序,有序集合,滑动窗口) ------");
    let nums = vec![1, 5, 9, 1, 5, 9];
    let answer = sliding_window_algo::contains_nearby_almost_duplicate(nums, 2, 3);
    println!("contains_nearby_almost_duplicate: {answer}"); // false

    println!("----- 526.优美的排列(位运算,动态规划,回溯,状态压缩) ------");
    let answer = bit_operation_algo::count_arrangement(2);
    println!("count_arrangement: {answer}"); // 2

    println!("----- LCR 115.序列重建(图,拓扑排序,数组) ------");
    let nums = vec![1, 2, 3];
    let sequences = vec![[1, 2], [1, 3], [2, 3]];
    let answer = graph_algo::sequence_reconstruction(nums, sequences);
    println!("sequence_reconstruction: {answer}"); // true

    println!("----- 912.排序数组(桶排序,分治,数组,基数排序,计数排序,归并排序,堆(优先队列)) ------");
    let nums = vec![5, 1, 1, 8, 2, 18, 11, 0, 0, 4, 28];
    let answer = array_algo::sort_array(nums);
    println!("sort_array: {answer:?}"); // [0, 0, 1, 1, 2, 4, 5, 8, 11, 18, 28]

    println!("----- 1584.连接所有点的最小费用(并查集,图,最小生成树) ------");
    let points = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
    let answer = min_spanning_tree_algo::min_cost_connect_points(points);
    println!("min_cost_connect_points: {answer}"); // 18

    println!("----- 398.随机数索引(水塘抽样,数学,随机化,哈希表) ------");
    // 给定一个可能含有 重复元素 的整数数组 nums,请你随机输出给定的目标数字 target 的索引。你可以假设给定的数字一定存在于数组中。
    // 实现 Solution2 类:
    // Solution2(int[] nums) 用数组 nums 初始化对象。
    // int pick(int target) 从 nums 中选出一个满足 nums[i] == target 的随机索引 i 。如果存在多个有效的索引,则每个索引的返回概率应当相等。
    // 输入
    // ["Solution2", "pick", "pick", "pick"]
    // [[[1, 2, 3, 3, 3]], [3], [1], [3]]
    // 输出
    // [null, 4, 0, 2]
    // 解释:
    // Solution2 solution = new Solution2([1, 2, 3, 3, 3]);
    // solution.pick(3); // 随机返回索引 2, 3 或者 4 之一。每个索引的返回概率应该相等。
    // solution.pick(1); // 返回 0 。因为只有 nums[0] 等于 1 。
    // solution.pick(3); // 随机返回索引 2, 3 或者 4 之一。每个索引的返回概率应该相等。
    let nums = vec![1, 2, 3, 3, 3, 2];
    let solution2 = Solution2::new(nums);
    println!("solution2.pick(3): {}", solution2.pick(3)); // 2 or 3 or 4
    println!("solution2.pick(1): {}", solution2.pick(1)); // 0
    println!("solution2.pick(3): {}", solution2.pick(3)); // 2 or 3 or 4
    println!("solution2.pick(2): {}", solution2.pick(2)); // 1 or 5

    println!("\n-------------up---------------\n");

    println!("----- 149.直线上最多的点数(几何,数学,数组,哈希表) ------");
    let points = vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
    let answer = hash_algo::max_points(points);
    println!("max_points: {answer}"); // 4

    println!("----- 753.破解保险箱(DFS,图,欧拉回路) ------");
    let answer = graph_algo::crack_safe(2, 2);
    println!("crack_safe: {answer}"); // 01100

    println!("----- 391.完美矩形(数组,扫描线) ------");
    let rectangles = vec![vec![1, 1, 3, 3], vec![3, 1, 4, 2], vec![3, 2, 4, 4], vec![1, 3, 2, 4], vec![2, 3, 3, 4]];
    let answer = scan_line_algo::is_rectangle_cover(rectangles);
    println!("is_rectangle_cover: {answer}"); // true

    println!("----- 218.天际线问题(树状数组,线段树,分治,有序集合,堆(优先队列),扫描线) ------");
    let buildings = vec![vec![2, 9, 10], vec![3, 7, 15], vec![5, 12, 12], vec![15, 20, 10], vec![19, 24, 8]];
    let answer = scan_line_algo::get_skyline(buildings);
    println!("get_skyline: {answer:?}"); // [[2, 10], [3, 15], [7, 12], [12, 0], [15, 10], [20, 8], [24, 0]]
}
