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