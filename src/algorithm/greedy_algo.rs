use std::collections::VecDeque;

/// 334.递增的三元子序列(贪心,数组)
// 判断数组nums中是否存在长度为 3 的递增子序列。
// 如果存在这样的三元组下标 (i, j, k) 且满足 i < j < k,使得 nums[i] < nums[j] < nums[k],返回 true;否则,返回 false
// 三元组 (3, 4, 5) 满足题意,因为 nums[3] == 0 < nums[4] == 4 < nums[5] == 6,返回true
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
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

/// 649.Dota2 参议院(贪心+双端队列,字符串)
pub fn predict_party_victory(senate: String) -> String {
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

/// 435.无重叠区间(数组,贪心算法(greedy algorithm),动态规划)
/// 无重叠区间(区间集合问题)
// 给定一个区间的集合 intervals,其中 intervals[i] = [start, end] 返回 需要移除区间的最小数量,使剩余区间互不重叠
// 输入: intervals = [[1, 2], [2, 3], [3, 4], [1, 3]]
// 输出: 1
// 解释: 移除 [1, 3] 后,剩下的区间没有重叠。
pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    // Vec::is_empty() 通常会比 Vec::len() == 0 执行效率稍微快
    if intervals.len() < 2 { return 0; }

    // intervals.sort_unstable();
    // println!("sort_unstable ----> {intervals:?}"); // [[1, 2], [1, 3], [2, 3], [3, 4]]
    intervals.sort_unstable_by_key(|v| v[1]);
    // println!("sort_unstable_by_key ----> {intervals:?}"); // [[1, 2], [2, 3], [1, 3], [3, 4]]
    // sort_unstable() 方法对 intervals 进行不稳定的就地排序。
    // 它按照元素的自然顺序(对于 Vec<Vec<i32>> 类型,就是按照每个子vec的第一个元素,即 v[0],进行比较)进行排序。
    // 不稳定排序意味着相等的元素在排序后的相对顺序可能发生变化。
    // intervals 调用 sort_unstable(),它会按照每个子vec的第一个元素进行排序。
    // 如果 intervals 包含 [[1, 3], [2, 4], [1, 2]],排序后的结果可能是 [[1, 3], [1, 2], [2, 4]] (注意 [1, 3] 和 [1, 2] 的顺序可能互换,因为排序是不稳定的)。
    // sort_unstable_by_key() 方法允许你提供一个闭包(函数对象),它用于提取排序时要使用的键。
    // 在例子中,闭包 |v| v[1] 表示每个子vec的第二个元素(即 v[1]）将用作排序的键。
    // intervals 调用 sort_unstable_by_key(|v| v[1]),它会按照每个子vec的第二个元素进行排序。
    // 如果 intervals 包含 [[1, 3], [2, 4], [1, 2]],排序后的结果将是 [[1, 2], [1, 3], [2, 4]],因为排序是基于每个子vec的第二个元素进行的。

    let mut count = 0;
    let mut end = intervals[0][1];
    for v in intervals.iter().skip(1) {
        if v[0] >= end { end = v[1]; } else { count += 1; }
    }

    count
}
//-----------------------------------------------------

/// 452.用最少数量的箭引爆气球(贪心算法(greedy algorithm),数组,排序)
// 有许多球形气球贴在一堵用 XY 平面表示的墙面上。
// 墙面上的气球记录在整数数组 points,其中points[i] = [Xstart, Xend] 表示水平直径在 Xstart 和 Xend之间的气球。你不知道气球的确切 y 坐标。
// 一支弓箭可以沿着 x 轴从不同点 完全垂直 地射出。
// 在坐标 x 处射出一支箭,若有一个气球的直径的开始和结束坐标为 Xstart, Xend, 且满足 Xstart ≤ X ≤ Xend,则该气球会被 引爆。
// 可以射出的弓箭的数量 没有限制.弓箭一旦被射出之后,可以无限地前进。
// 数组 points, 返回引爆所有气球所必须射出的 最小 弓箭数。
// 输入: points = [[10, 16], [2, 8], [1, 6], [7, 12]]
// 输出: 2
// 解释: 气球可以用2支箭来爆破:
// -在 x = 6 处射出箭,击破气球 [2, 8] 和 [1, 6]
// -在 x = 11 处发射箭,击破气球 [10, 16] 和 [7, 12]
pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    // 解法一:直接将二维数组按照末尾的数字进行排序,然后比较 区间 是否有重叠部分。
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