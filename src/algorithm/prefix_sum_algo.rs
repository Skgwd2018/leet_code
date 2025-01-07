// 前缀和(Prefix Sum)
// 前缀和模式是对一个数组进行预处理，生成一个新数组，其中索引i处的元素表示从数组开头到i位置的元素总和。这样就能高效地对子数组进行求和查询。
// 当需要对子数组进行多次求和查询，或者需要计算累加和时，可使用该模式。
// 示例问题：给定一个数组nums，回答关于特定区间[i, j]内元素之和的多个查询。
// 示例：输入：nums = [1, 2, 3, 4, 5, 6]，i = 1，j = 3 输出：9
// 解释：预处理数组A以创建前缀和数组：P = [1, 3, 6, 10, 15, 21]。要找出索引i和j之间的元素和，使用公式：P[j] - P[i - 1]。
pub fn prefix_sum_ex(nums: Vec<i32>, i: usize, j: usize) -> i32 {
    // .scan() 接受一个初始状态(在这个例子中是0)和一个闭包(closure)。常用于处理前缀和问题.
    // 这个闭包有两个参数：sum(累加器，初始值为0) 和 &x(当前迭代到的元素的引用，由于.iter()产生的是不可变引用，所以这里也是不可变引用)。
    // 闭包的返回类型是Option<T>，本例中，T是i32类型。每次迭代时，闭包需要返回一个Some(value)，这个value会被.scan()方法收集起来，作为结果迭代器的一个元素。
    let p = nums.iter().scan(0, |sum, &x| {
        *sum += x;
        Some(*sum)
    }).collect::<Vec<i32>>();

    p[j] - p[i - 1]
}
//-----------------------------------------------------

/// 1732.找到最高海拔(数组,前缀和)
/// 使用归约操作解决前缀和问题
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
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

/// 724.寻找数组的中心下标(数组,前缀和)
/// 中心下标
// 数组的中心下标 是数组的一个下标,其左侧所有元素相加的和等于右侧所有元素相加的和。
// 如果中心下标位于数组最左端,那么左侧数之和视为 0,因为在下标的左侧不存在元素。这一点对于中心下标位于数组最右端同样适用。
// 如果数组有多个中心下标,应该返回 最靠近左边 的那一个。如果数组不存在中心下标,返回 -1 。
// #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn pivot_index(nums: Vec<i32>) -> i32 {
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
