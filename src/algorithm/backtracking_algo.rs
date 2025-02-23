// 回溯(Backtracking)
// 回溯算法会探索所有可能的解，当某条解的路径行不通时就回溯。当需要找出满足给定约束条件的所有(或部分)解时，例如组合问题(生成排列、组合或子集等)，可使用该模式。
// 示例问题：生成给定数字列表的所有排列。
// 示例：输入：nums = [1, 2, 3] 输出：[[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
// 解释：使用递归来生成排列。对于每个元素，将其包含在当前排列中，并递归地生成剩余元素的排列。当某条路径下的所有排列都生成后，进行回溯。
pub fn backtracking_ex(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(answer: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>, start: usize) {
        if start == nums.len() {
            answer.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i); // 将当前数字移动到排列的起始位置
            backtrack(answer, nums, start + 1); // 递归构建剩余数字的排列
            nums.swap(start, i); // 回溯:将数字移回原位
        }
    }

    let mut answer = Vec::new();
    backtrack(&mut answer, nums, 0);
    answer
}
//-----------------------------------------------------

/// 17.电话号码的字母组合(字符串,哈希表,回溯)
pub fn letter_combinations(digits: &str) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();
    let mut value: Vec<char> = Vec::new();
    /* match digits.is_empty() {
        true => (),
        false => get_letters(&digits, 0, &mut value, &mut answer),
    } */
    if !digits.is_empty() {
        get_letters(digits, 0, &mut value, &mut answer);
    }

    answer
}
/// backtrack(回溯)
// digits:输入的字符串, index:当前的索引, value:用于存储当前字母组合的Vec<char>, answer:用于存储所有结果的Vec<String>
fn get_letters(digits: &str, index: usize, value: &mut Vec<char>, answer: &mut Vec<String>) {
    if index >= digits.len() {
        // let s = String::from_iter(value.iter()); // 将一个字符迭代器转换为一个字符串
        let s = value.iter().collect(); // 将一个字符迭代器转换为一个字符串
        // let value = vec!['a', 'b', 'c'];
        // let s = value.iter().collect(); // "abc"
        answer.push(s);
        return;
    }

    // .iter().nth(n) 返回迭代器的第n个元素
    // 注:所有前面的元素以及返回的元素都将从迭代器中消耗掉。即前面的元素将被丢弃,并且在同一迭代器上多次调用第n(0)个元素将返回不同的元素。
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

/// 216.组合总和Ⅲ(数组,回溯)
// 找出所有相加之和为 n 的 k 个数的组合,且满足下列条件:
// 只使用数字[1, 9] 且 每个数字最多使用一次, 返回 所有可能的有效组合的列表。该列表不能包含相同的组合两次,组合可以以任何顺序返回。
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    // 回溯函数:实现回溯算法。回溯算法常用于解决组合问题,它通过递归和剪枝的方式找出所有可能的解。
    /// answer:用于存储所有满足条件的组合的Vec
    ///   curr:当前正在构建的组合
    ///      i:当前可选取的最大正整数
    ///      k:还需要找出多少个正整数
    ///      n:当前组合还需要凑足的和
    fn backtrace(answer: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>, i: i32, k: i32, n: i32) {
        let c = k - i32::try_from(curr.len()).expect("i32 error");
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