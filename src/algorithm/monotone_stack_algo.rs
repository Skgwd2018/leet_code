// 单调栈（monotone stack）是一种数据结构,其特性是栈内元素(从栈底到栈顶)是单调递增或单调递减的。
// 当新的元素入栈时,会移除栈顶破坏单调性的元素,以确保栈内元素保持单调性。这些出栈的元素在后续操作中不会再次入栈。
// 由于每个元素至多入栈和出栈各一次,因此单调栈的维护时间复杂度是O(n)。
// 单调栈有两种类型：单调递增栈和单调递减栈。单调递增栈意味着栈内元素从栈底到栈顶是递增的,而单调递减栈则是递减的。
// 单调栈常用于解决一些需要找到某个元素左边或右边第一个比它大或小的问题,
// 例如:柱状图中最大的矩形、最长递增子序列等问题。它也可以用于优化某些动态规划问题的求解过程。
// 特别是在解决一些涉及数组或序列的问题时。单调栈的应用广泛,通过合理的设计和使用,可以有效地提高算法的效率。
// 需要注意的是,单调栈中存储的元素可以是数组的值,也可以是数组的下标。

/// 739.每日温度(栈,数组,单调栈)
/// 每日温度(单调栈(monotone stack))
// 给定一个整数数组 temperatures ,表示每天的温度,返回一个数组 answer,其中 answer[i] 是指对于第 i 天,下一个更高温度出现在几天后。
// 如果气温在这之后都不会升高,请在该位置用 0 来代替。
// 输入: temperatures = [73, 74, 75, 71, 69, 72, 76, 73]  // 8
// 输出:                [1, 1, 4, 2, 1, 1, 0, 0]
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut answer = vec![0; temperatures.len()];
    let mut stack = Vec::new();
    for (i, &t) in temperatures.iter().enumerate() {
        // *stack.last().unwrap()
        // stack[stack.len() - 1] 这个运行会稍微快
        while !stack.is_empty() && t > temperatures[stack[stack.len() - 1]] {
            let j = stack.pop().unwrap();
            answer[j] = i32::try_from(i - j).expect("i32 error");
        }
        stack.push(i);
    }

    answer
}
//-----------------------------------------------------

/// 901.股票价格跨度(单调栈,数据流,设计,栈)
/// 股票跨度
#[derive(Default)]
pub struct StockSpanner {
    stack: Vec<(i32, i32)>,
    curr_day: i32,
}

impl StockSpanner {
    pub fn new() -> Self {
        Self {
            // 这样无需判断栈为空的情况
            stack: vec![(-1, i32::MAX)],
            // 第一个 next 调用算作第 0 天
            curr_day: -1,
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        // 栈顶数据后面不会再用到了,因为 price 更大
        while price >= self.stack.last().unwrap().1 { self.stack.pop(); }
        self.curr_day += 1;
        let result = self.curr_day - self.stack.last().unwrap().0;
        self.stack.push((self.curr_day, price));

        result
    }
}
//-----------------------------------------------------