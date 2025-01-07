// 动态规划模式(Dynamic Programming Patterns)
// 动态规划(DP)是将问题分解为更小的子问题，并使用自底向上或自顶向下的方法来解决。在处理具有重叠子问题和最优子结构特性的问题时，可使用该模式。
// 动态规划本身包含多个子模式，其中一些最重要的子模式如下：斐波那契数列, 0/1背包, 最长公共子序列(LCS), 最长递增子序列(LIS), 子集和, 矩阵链乘法等问题.

/// 509.斐波那契数(记忆化搜索,动态规划)
// 示例问题：计算第n个斐波那契数。
// 示例：输入：n = 6 输出：8 (前六个斐波那契数是0，1，1，2，3，5，8)
// 解释：使用自底向上的方法来计算第n个斐波那契数。从最初的两个数(0 和 1)开始，通过迭代计算后续的数，如 dp[i] = dp[i - 1] + dp[i - 2]
// 斐波那契数(通常用 F(n) 表示)形成的序列称为 斐波那契数列。该数列由 0 和 1 开始,后面的每一项数字都是前面两项数字的和。
// F(0) = 0, F(1) = 1
// F(n) = F(n - 1) + F(n - 2), 其中 n > 1
// 给定 n, 请计算 F(n)
pub fn fib(n: i32) -> i32 {
    if n < 2 { return n; }

    // let n = n as usize;
    // let mut dp = vec![0; n + 1];
    // dp[0] = 0;
    // dp[1] = 1;
    // for i in 2..=n {
    //     dp[i] = dp[i - 1] + dp[i - 2];
    // }
    // dp[n]

    (2..=n).fold((0, 1), |(dp1, dp2), _| (dp2, dp1 + dp2)).1
}
//-----------------------------------------------------

/// 1137.第N个泰波那契数(记忆化搜索,数学,动态规划)
/// 泰波那契序列 Tn 定义如下:
/// T0 = 0, T1 = 1, T2 = 1, 且在 n >= 0 的条件下 Tn+3 = Tn + Tn+1 + Tn+2
/// 给你整数 n,请返回第 n 个泰波那契数 Tn 的值。
pub fn tribonacci(n: i32) -> i32 {
    // 递归方式:
    /*match n {
        0 => 0,
        1 | 2 => 1,
        _ => tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3),
    }*/

    // 解法二:
    // 动态规划(Dynamic Programming,DP)避免重复计算。
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

/// 746.使用最小花费爬楼梯(数组,动态规划)
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let (p1, p2) = (2..cost.len()).fold((cost[0], cost[1]), |(c1, c2), i| (c2, c1.min(c2) + cost[i]));

    p1.min(p2)
}
//-----------------------------------------------------

/// 70.爬楼梯(记忆化搜索,动态规划)
// 假设你正在爬楼梯,需要 n 阶才能到达楼顶。
// 每次可以爬 1 或 2 个台阶。有多少种不同的方法可以爬到楼顶?
// 输入: n = 3
// 输出: 3
// 解释:有三种方法可以爬到楼顶
// 1. 1 阶 + 1 阶 + 1 阶
// 2. 1 阶 + 2 阶
// 3. 2 阶 + 1 阶
pub fn climb_stairs(n: i32) -> i32 {
    // 解法一: 递归 + 记忆化
    /*fn dfs(i: usize, memo: &mut Vec<i32>) -> i32 {
        if i <= 1 { // 递归边界
            return 1;
        }

        if memo[i] != 0 { // 之前计算过
            return memo[i];
        }

        let res = dfs(i - 1, memo) + dfs(i - 2, memo);
        memo[i] = res; // 记忆化

        res
    }
    let n = n as usize;
    let mut memo = vec![0; n + 1];
    dfs(n, &mut memo)*/

    // 解法二: 动态规划
    if n <= 2 { return n; };

    (2..n).fold((1, 2), |(c1, c2), _| (c2, c1 + c2)).1
}
//-----------------------------------------------------

/// 198.打家劫舍(数组,动态规划)
// 假设你是一个专业的小偷,计划偷窃沿街的房屋。每间房内都藏有一定的现金,影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统,
// 如果两间相邻的房屋在同一晚上被小偷闯入,系统会自动报警。
// 给定一个代表每个房屋存放金额的非负整数数组,计算 不触动警报装置的情况下,一夜之内能够偷窃到的最高金额。
// 输入: [2, 7, 9, 3, 1]
// 输出: 12
// 解释: 偷窃 1 号房屋(金额 = 2), 偷窃 3 号房屋(金额 = 9), 接着偷窃 5 号房屋(金额 = 1)
//      偷窃到的最高金额 = 2 + 9 + 1 = 12
pub fn rob(nums: Vec<i32>) -> i32 {
    nums.iter().skip(1).fold((nums[0], 0), |dp, &n| (dp.0, dp.0).max((dp.1 + n, dp.0))).0
}
//-----------------------------------------------------

/// `790.多米诺和托米诺平铺(动态规划_一维)`
// 有两种形状的瓷砖:一种是 2 x 1 的多米诺形,另一种是形如 "L" 的托米诺形。两种形状都可以旋转。
// 给定整数 n, 返回可以平铺 2 x n 的面板的方法的数量。返回对 10的9次方 + 7 取模 的值。
// 平铺指的是每个正方形都必须有瓷砖覆盖。两个平铺不同,当且仅当面板上有四个方向上的相邻单元中的两个,使得恰好有一个平铺有一个瓷砖占据两个正方形。
pub fn num_tilings(n: i32) -> i32 {
    (1..n).fold((0, 1, 1, 1e9 as i32 + 7), |(a, b, c, m), _| (b, c, (2 * c % m + a) % m, m)).2
}
//-----------------------------------------------------

/// `62.不同路径(组合数学,动态规划_多维_网格路径),矩阵dp空间优化`
// 一个机器人位于一个 m x n 网格的最左上角(标记为 "Start")。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的最右下角(标记为 "Finish")。
// 问总共有多少条不同的路径?
pub fn unique_paths(m: i32, n: i32) -> i32 {
    // 动态规划方法:这里利用了问题的子问题重叠性质,通过计算并保存子问题的解来避免重复计算,从而提高了效率。
    // 对于较大的 m 和 n 值,这种方法比直接计算组合数更加高效。
    // dp关系: dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
    let n = n as usize;
    // 初始化一个长度为 n 的一维数组 dp, 其中 dp[j] 表示到达第 j 列的最后一行(即第 m-1 行)有多少种不同的路径。
    // 由于只能向右或向下移动,到达第一行的每个点都只有一条路径(即一直向右移动),所以 dp 数组的所有元素初始值都设为 1
    let mut dp = vec![1; n];
    // 通过两层循环来计算到达每个点的不同路径数。
    // 外层循环遍历每一行(从第 1 行到第 m-1 行),内层循环遍历每一列(从第 1 列到第 n-1 列)。
    // 对于每个点 (i, j),其路径数等于其上方点 (i-1, j) 的路径数加上其左方点 (i, j-1) 的路径数,即 dp[j] = dp[j] + dp[j - 1]
    // 注:第一列的每个点的路径数始终为 1(因为只能一直向下移动),所以在内层循环开始前,先将 dp[0] 设为 1
    /*for _ in 1..(m as usize) {
        dp[0] = 1;
        for j in 1..n { dp[j] += dp[j - 1]; }
    }*/
    // 注:使用 for_each() 可能会略微增加一些额外的开销,因为闭包的创建和调用通常会比直接的for循环略微慢。
    // 但这个差异通常是非常微小的,除非在性能非常关键的场景下,否则这种差异通常可以忽略不计。
    // 此外,Rust编译器的优化器也会对这两种形式进行相似的优化,使得它们在实际运行时的性能非常接近。
    (1..m as usize).for_each(|_| {
        dp[0] = 1;
        (1..n).for_each(|j| dp[j] += dp[j - 1]);
    });

    dp[n - 1]

    // 解法二:
    // 当 m 和 n 值较小时采用计算组合数方式更高效。
    // 使用数学公式计算组合数学中的组合数(Combination),
    // 从 (0, 0) 到 (m-1, n-1) 的路径,总共需要走 m+n-2 步,其中 m-1 步是向右，n-1 步是向下。
    // 因此,问题转化为从 m+n-2 步中选择 m-1 步向右走,剩下的自然是向下走。
    // let n = n as u64 - 1;
    // (1..m as u64).fold(1, |cnt, x| cnt * (n + x) / x) as i32
}
//-----------------------------------------------------

/// `1143.最长公共子序列(字符串,动态规划_多维)`
// 给定两个字符串 text1 和 text2,返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列,返回 0
// 一个字符串的 子序列 是指这样一个新的字符串:它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
// 例如: "ace" 是 "abcde" 的子序列,但 "aec" 不是 "abcde" 的子序列。
// 两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    /*let (m, n) = (text1.len(), text2.len());
    // 因为dp[i][j] 是表示下标(i-1, j-1) 的 最长公共子序列,由于 i / j == 0 都是无意义的,可以初始化为0
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
            // text1.bytes() 会返回一个迭代器,它逐个产生text1中每个字符的字节表示。
            // .nth(i - 1) 会获取迭代器中第i - 1个元素的值。如果i - 1超出了迭代器的范围，它将返回None。
            // text1.as_bytes() 会返回一个指向字符串内部字节数组的slice,这个slice是原始字符串的直接视图,没有额外的迭代器开销。
            // .get(i - 1) 会尝试获取切片中索引为i - 1的元素的可变引用,如果这个索引是有效的,那么它就会返回一个指向该元素的引用。
            // .copied() 会将这个引用转换为对应元素的值(如果存在的话),并产生一个 Option<u8>
            // 在性能上, text1.as_bytes().get(i - 1).copied() 通常会比 text1.bytes().nth(i - 1) 更快,
            // 因为 as_bytes() 是直接访问字符串的内部数据,而 bytes() 则需要在每次调用时生成一个新的迭代器。
            // 迭代器每次调用 nth() 时都需要从当前位置开始重新计算到目标位置,这增加了额外的开销。
            // 因此优先使用 text1.as_bytes().get(i - 1).copied() 来访问字符串的字节。
            dp[i][j] = if text1.as_bytes().get(i - 1).copied() == text2.as_bytes().get(j - 1).copied() {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        });
    });

    dp[m - 1][n - 1]
}
//-----------------------------------------------------

/// `714.买卖股票的最佳时机含手续费(数组,贪心,动态规划_多维)`
/// 动态规划(最大收益问题)
// 给定一个整数数组 prices,其中 prices[i]表示第 i 天的股票价格; 整数 fee 代表了交易股票的手续费用。
// 可以无限次地完成交易,但是每笔交易都需要付手续费。如果已经购买了一个股票,在卖出它之前就不能再继续购买股票了,返回获得利润的最大值。
// 注意:这里的一笔交易指买入持有并卖出股票的整个过程,每笔交易你只需要为支付一次手续费。
pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    prices.iter().fold((0, -prices[0]), |(sell, buy), p| (sell.max(buy + p - fee), buy.max(sell - p))).0
}
//-----------------------------------------------------

/// `72.编辑距离(字符串,动态规划_多维)`
// 给定两个单词 word1 和 word2,请返回将 word1 转换成 word2 所使用的最少操作数。
// 可以对一个单词进行这三种操作:插入一个字符 or 删除一个字符 or 替换一个字符
// 输入: word1 = "intention", word2 = "execution"
// 输出: 5
// 解释:
// intention -> inention (删除 't')
// inention -> enention (将 'i' 替换为 'e')
// enention -> exention (将 'n' 替换为 'x')
// exention -> exection (将 'n' 替换为 'c')
// exection -> execution (插入 'u')
pub fn min_distance(word1: String, word2: String) -> i32 {
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
            // 由于 if 语句是表达式,可以实现类似三元运算符的效果
            dp[i + 1][j + 1] = if c1 == c2 {
                dp[i][j]
            } else {
                (dp[i][j] + 1).min(dp[i + 1][j] + 1).min(dp[i][j + 1] + 1)
            };
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