/// 1584.连接所有点的最小费用(并查集,图,最小生成树)
// 给定一个points 数组，表示 2D 平面上的一些点，其中 points[i] = [xi, yi]
// 连接点 [xi, yi] 和点 [xj, yj] 的费用为它们之间的 曼哈顿距离 ：|xi - xj| + |yi - yj| ，其中 |val| 表示 val 的绝对值。
// 请返回将所有点连接的最小总费用。只有任意两点之间 有且仅有 一条简单路径时，才认为所有点都已连接。
// 输入：points = [[3,12],[-2,5],[-4,1]]
// 输出：18
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    // Kruskal 算法
    let n = points.len();
    let mut edges = Vec::new();
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            edges.push((i, j, (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()));
        }
    }
    edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut parents = Vec::new();
    for i in 0..n { parents.push(i); }

    let mut ans = 0;
    for edge in edges {
        let (mut a, mut b) = (edge.0, edge.1);
        while a != parents[a] {
            a = parents[a];
        }
        while b != parents[b] {
            b = parents[b];
        }

        if a == b { continue; }

        parents[b] = a;
        ans += edge.2;
    }

    ans
}
//-----------------------------------------------------