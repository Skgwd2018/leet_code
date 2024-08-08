use std::collections::{BTreeMap, HashSet};

/// 391.完美矩形(数组,扫描线)
// 给定一个数组 rectangles,其中 rectangles[i] = [xi, yi, ai, bi] 表示一个坐标轴平行的矩形。
// 这个矩形的左下顶点是 (xi, yi) ,右上顶点是 (ai, bi)。
// 如果所有矩形一起精确覆盖了某个矩形区域,则返回 true;否则,返回 false
// 输入: rectangles = [[1, 1, 3, 3], [3, 1, 4, 2], [3, 2, 4, 4], [1, 3, 2, 4], [2, 3, 3, 4]]
// 输出: true
// 解释: 5 个矩形一起可以精确地覆盖一个矩形区域
pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
    let (x1, y1, x2, y2, a, set) = rectangles.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN, 0, HashSet::new()),
        |(x1, y1, x2, y2, a, mut set), rect| {
            for p in [(rect[0], rect[1]), (rect[2], rect[1]), (rect[0], rect[3]), (rect[2], rect[3])] {
                if !set.remove(&p) {
                    set.insert(p);
                }
            }

            (x1.min(rect[0]), y1.min(rect[1]), x2.max(rect[2]), y2.max(rect[3]), a + (rect[2] - rect[0]) * (rect[3] - rect[1]), set)
        });

    (x2 - x1) * (y2 - y1) == a && set == HashSet::from([(x1, y1), (x1, y2), (x2, y1), (x2, y2)])
}
//-----------------------------------------------------

/// 218.天际线问题(树状数组,线段树,分治,有序集合,堆(优先队列),扫描线)
// 城市的 天际线 是从远处观看该城市中所有建筑物形成的轮廓的外部轮廓。给你所有建筑物的位置和高度,请返回 由这些建筑物形成的 天际线。
// 每个建筑物的几何信息由数组 buildings 表示,其中三元组 buildings[i] = [lefti, righti, heighti] 表示：
// lefti 是第 i 座建筑物左边缘的 x 坐标。
// righti 是第 i 座建筑物右边缘的 x 坐标。
// heighti 是第 i 座建筑物的高度。
// 可以假设所有的建筑都是完美的长方形,在高度为 0 的绝对平坦的表面上。
// 天际线 应该表示为由 “关键点” 组成的列表,格式 [[x1, y1], [x2, y2], ...],并按 x 坐标 进行 排序。关键点是水平线段的左端点。
// 列表中最后一个点是最右侧建筑物的终点,y 坐标始终为 0 ,仅用于标记天际线的终点。此外,任何两个相邻建筑物之间的地面都应被视为天际线轮廓的一部分。
// 注意:输出天际线中不得有连续的相同高度的水平线。例如 [...[2 3], [4 5], [7 5], [11 5], [12 7]...] 是不正确的答案；
// 三条高度为 5 的线应该在最终输出中合并为一个: [...[2 3], [4 5], [12 7], ...]
// 输入: buildings = [[2, 9, 10], [3, 7, 15], [5, 12, 12], [15, 20, 10], [19, 24, 8]]
// 输出: [[2, 10], [3, 15], [7, 12], [12, 0], [15, 10], [20, 8], [24, 0]]
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    /*let mut max_heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

    let mut build_edges = vec![];
    for build in buildings.iter() {
        build_edges.push(build[0]);
        build_edges.push(build[1]);
    }
    build_edges.sort_unstable();

    let n = buildings.len();
    let mut index = 0_usize;
    let mut result: Vec<Vec<i32>> = vec![];
    for &edge in build_edges.iter() {
        while index < n && buildings[index][0] <= edge {
            max_heap.push((buildings[index][2], buildings[index][1]));
            index += 1;
        }
        while let Some(&(_, edge_end)) = max_heap.peek() {
            if edge_end <= edge {
                max_heap.pop();
            } else { break; }
        }

        let curr_max_height = match max_heap.peek() {
            None => 0,
            Some(&(height, _)) => height,
        };
        if result.is_empty() || result.last().unwrap()[1] != curr_max_height {
            result.push(vec![edge, curr_max_height]);
        }
    }
    result*/

    // 解法二:
    let mut build_edges = Vec::new();
    for building in buildings {
        build_edges.push((building[0], -building[2])); // 左端点,用h正负表示
        build_edges.push((building[1], building[2])); // 右端点
    }
    build_edges.sort_unstable(); // 按照横坐标排序

    let mut ans: Vec<Vec<i32>> = Vec::new(); // 关键点
    let mut map = BTreeMap::new(); //最大堆存放高度记录个数,有重复时去掉,为0时删除,模拟MultiSet
    let mut prev = 0; // 记录上一个延展线的最大高度
    map.insert(0, 1);
    for &(x, h) in &build_edges {
        match h {
            i32::MIN..=-1 => {
                map.entry(-h).and_modify(|e| *e += 1).or_insert(1); // 左端点,高度出堆
            }
            0..=i32::MAX => {
                *map.get_mut(&h).unwrap() -= 1;
                if map.get(&h).unwrap() == &0 {
                    map.remove(&h);
                }
            }
        }
        // 取出最高高度,如果和前一个矩形的顶部延展线不重合,可以记录
        let cur = *map.last_entry().unwrap().key();
        if cur != prev {
            ans.push(vec![x, cur]);
            prev = cur;
        }
    }

    ans
}
//-----------------------------------------------------