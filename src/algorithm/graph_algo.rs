use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// 841.钥匙和房间(dfs,bfs,图)
// 有 n 个房间,房间按从 0 到 n - 1 编号。最初,除 0 号房间外的其余所有房间都被锁住。目标是进入所有的房间,但不能在没有获得钥匙的时候进入锁住的房间。
// 当进入一个房间,可能会在里面找到一套不同的钥匙,每把钥匙上都有对应的房间号,即表示钥匙可以打开的房间。可以拿上所有钥匙去解锁其他房间。
// 给定一个数组 rooms 其中 rooms[i] 是进入 i 号房间可以获得的钥匙集合。如果能进入 所有 房间返回 true,否则返回 false。
pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
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

/* pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
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

// 并查集(Disjoint-Set)是一种数据结构,主要用于管理一组元素的分组情况,并提供合并(Union)和查找(Find)两种基本操作。
// 这种数据结构主要用于解决连通性问题,例如:判断元素是否在同一集合中,并在需要时合并两个集合。用于处理元素分组和连通性问题.

/// 547.省份数量(并查集,图,bfs,dfs)
// 解法二:上面的优化版
pub fn find_circle_num2(is_connected: Vec<Vec<i32>>) -> i32 {
    // Find
    fn find(mut i: usize, par: &[usize]) -> usize {
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
    // .iter_mut() 是用于可变引用集合(例: Vec<T> 或 &mut [T])的方法,它返回一个迭代器,该迭代器产生集合中每个元素的可变引用。
    // .iter_mut() 用于在迭代过程中修改集合的元素,返回的是可变引用迭代器,之后vec集合可以继续使用。
    // let mut vec = vec![1, 2, 3];
    // for item in vec.iter_mut() { *item *= 2; }
    // println!("{:?}", vec); // 输出 [2, 4, 6]
    // .into_iter() 是一个消费性方法,用于将集合转换为所有权转移给迭代器的元素。这通常用于当你不再需要原始集合,并希望将其元素作为迭代器使用时。
    // .into_iter() 用于将集合的所有权转移到迭代器中,通常用于当你不再需要原始集合时。调用.into_iter()后,原始vec集合将不再可用,因为其所有权已被转移到迭代器中。
    // let iter = vec.into_iter();
    // for item in iter { println!("{}", item); }
    // vec 在这里不再可用,因为所有权已转移到 iter

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
    i32::try_from(answer).expect("i32 error")
}
//-----------------------------------------------------

/// 1466.重新规划路线(图,dfs,bfs)
/// 重新规划路线(深度优先搜索)
// n 座城市,从 0 到 n-1 编号,其间共有 n-1 条路线。因此,要想在两座不同城市之间旅行只有唯一一条路线可供选择(路线网形成一棵树)。
// 去年,交通运输部决定重新规划路线,以改变交通拥堵的状况。
// 路线用 connections 表示,其中 connections[i] = [a, b] 表示从城市 a 到 b 的一条有向路线。
// 今年,城市 0 将会举办一场大型比赛,很多游客都想前往城市 0
// 请你帮助重新规划路线方向,使每个城市都可以访问城市 0 。返回需要变更方向的最小路线数。
// 题目数据:保证每个城市在重新规划路线方向后都能到达城市 0
// n = 6, connections: [[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]
pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    fn dfs(a: usize, fa: i32, g: &Vec<Vec<(i32, i32)>>) -> i32 {
        let mut answer = 0;
        for &(b, c) in &g[a] {
            if b != fa {
                answer += c + dfs(b as usize, a as i32, g);
            }
        }

        answer
    }

    let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
    for e in &connections {
        let (a, b) = (e[0] as usize, e[1] as usize);
        g[a].push((b as i32, 1));
        g[b].push((a as i32, 0));
    }

    dfs(0, -1, &g)
}
//-----------------------------------------------------

/// 399.除法求值(dfs,bfs,并查集,图,数组,字符串,最短路径)
// 给定一个变量对数组 equations 和一个实数值数组 values 作为已知条件,其中 equations[i] = [Ai, Bi] 和 values[i] 共同表示等式 Ai / Bi = values[i] 。每个 Ai 或 Bi 是一个表示单个变量的字符串。
// 另有一些以数组 queries 表示的问题,其中 queries[j] = [Cj, Dj] 表示第 j 个问题,请根据已知条件找出 Cj / Dj = ? 的结果作为答案。
// 返回 所有问题的答案。如果存在某个无法确定的答案,则用 -1.0 替代这个答案。如果问题中出现了给定的已知条件中没有出现的字符串,也需要用 -1.0 替代这个答案。
// 注意:输入总是有效的。可以假设除法运算中不会出现除数为 0 的情况,且不存在任何矛盾的结果。
// 注意:未在等式列表中出现的变量是未定义的,因此无法确定它们的答案。
pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    /*
    // graph1:将图转化为邻接矩阵
    let mut graph1 = HashMap::new();
    let n = equations.len();
    for i in 0..n {
        graph1.entry(equations[i][0].clone()).or_insert(HashMap::new()).entry(equations[i][1].clone()).or_insert(values[i]);
        graph1.entry(equations[i][1].clone()).or_insert(HashMap::new()).entry(equations[i][0].clone()).or_insert(1_f64 / values[i]);
    }

    // graph2:每个变量的分组情况,不同组之间没有通路
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
// 1.自动排序:当你向堆中插入元素时,堆会自动重新排序以确保堆的性质(父节点的值总是大于或等于(最大堆)或小于或等于(最小堆)其子节点的值)得到维护。
// 2.快速访问最高(或最低)优先级元素:堆的根节点(在 BinaryHeap 中,这通常是第一个元素)总是具有最高(或最低,取决于堆的类型)的优先级。因此,可以快速地获取或删除这个元素。
// 3.性能:插入和删除堆顶元素的平均时间复杂度是 O(log n),其中 n 是堆中元素的数量。这使得 BinaryHeap 在处理大量数据时非常高效。
// 4.泛型:BinaryHeap 是泛型的,即是可以用它来存储任何实现了 Ord trait(即可以排序)的类型。

/// 1926.迷宫中离入口最近的出口(图,bfs)
/// 迷宫出口(BFS广度优先搜索)
// maze[i][j] 要么是 '.' ,要么是 '+'
pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let dir = [-1, 0, 1, 0, -1]; // 方向
    let entrance = (entrance[0], entrance[1]); // 入口位置
    let n = i32::try_from(maze.len()).expect("i32 error");    // 行数
    let m = i32::try_from(maze[0].len()).expect("i32 error"); // 列数
    // BinaryHeap(二叉堆),主要用于处理那些需要优先队列特性的场景。
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
                if curr == entrance { continue; }
                return -cnt;
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
pub fn _nearest_exit2(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
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

/// 994.腐烂的橘子(bfs,数组,矩阵)
pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut queue = VecDeque::new();
    let mut cnt = 0;
    // 遍历一遍整个网格,统计出新鲜橘子的数量,记为 cnt,并且将所有腐烂的橘子的坐标加入队列 queue 中。
    // 相比使用grid[i][j]的操作方式执行效率更高,内存消耗更低
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
    // 每一轮(每分钟)搜索,将队列中的所有腐烂的橘子向四个方向腐烂新鲜橘子,直到队列为空或者新鲜橘子的数量为 0 为止。
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

    // 如果新鲜橘子的数量为 0,则返回当前的轮数,否则返回 −1
    if cnt > 0 { return -1; }
    answer
}
//-----------------------------------------------------

/// LCR 115.序列重建(图,拓扑排序,数组)
// 给定一个长度为 n 的整数数组 nums,其中 nums 是范围为 [1, n] 的整数的排列。
// 还提供了一个 2D 整数数组 sequences ,其中 sequences[i] 是 nums 的子序列。
// 检查 nums 是否是唯一的最短 超序列。最短 超序列 是 长度最短 的序列,并且所有序列 sequences[i] 都是它的子序列。
// 对于给定的数组 sequences,可能存在多个有效的 超序列。
// 例如,对于 sequences = [[1, 2], [1, 3]] ,有两个最短的 超序列, [1, 2, 3] 和 [1, 3, 2] 。
// 而对于 sequences = [[1, 2], [1, 3], [1, 2, 3]] ,唯一可能的最短 超序列 是 [1, 2, 3] 。[1, 2, 3, 4] 是可能的超序列,但不是最短的。
// 如果 nums 是序列的唯一最短 超序列,则返回 true,否则返回 false 。
// 子序列 是一个可以通过从另一个序列中删除一些元素或不删除任何元素,而不改变其余元素的顺序的序列。
// 输入: nums = [1, 2, 3], sequences = [[1, 2], [1, 3], [2, 3]]
// 输出: true
// 解释: 最短可能的超序列为[1, 2, 3]
// 序列 [1, 2] 是它的一个子序列: [1, 2, 3]
// 序列 [1, 3] 是它的一个子序列: [1, 2, 3]
// 序列 [2, 3] 是它的一个子序列: [1, 2, 3]
// 因为 nums 是唯一最短的超序列,所以返回true。
pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<[i32; 2]>) -> bool {
    let mut map = HashMap::new();
    for seq in sequences {
        // .windows() 返回一个迭代器,遍历所有长度大小的连续窗口。窗口重叠。如果切片小于大小,迭代器将不返回任何值。
        // .windows() 是 Iterator trait 的一个扩展方法,用于在切片（slice）或类似容器的迭代器上。
        // 它返回一个新的迭代器,该迭代器遍历原始数据中的所有连续窗口（或子切片）。窗口的大小由 windows() 方法的参数指定,并且窗口是重叠的。
        // 对于 sequences 中的每个元素(这里是一个 [i32; 2] 数组,即包含两个元素的数组),
        // windows(2) 会生成一个迭代器,该迭代器遍历这个数组的所有可能长度为2的连续窗口。
        // 由于数组长度恰好为2, windows(2) 实际上只会生成一个窗口,即整个数组本身。但是,这种方式使得代码在处理更长的序列时更加通用和灵活。
        // 例如,如果 seq 是 [1, 2, 3],那么 seq.windows(2) 将生成一个迭代器,依次返回 &[1, 2] 和 &[2, 3]。
        // 但在你的例子中,因为 seq 总是 [i32; 2], 所以 windows(2) 实际上只返回 &[seq[0], seq[1]]。
        seq.windows(2).for_each(|win| {
            map.entry(win[0]).or_insert(HashSet::new()).insert(win[1]);
        });
    }

    // 对于 nums(一个 Vec<i32>), windows(2) 遍历所有长度为2的连续子切片。
    // 即如果 nums 是 [1, 2, 3, 4],那么 nums.windows(2) 将生成一个迭代器,依次返回 &[1, 2]、&[2, 3] 和 &[3, 4]
    nums.windows(2).all(|win| map.get(&win[0]).map_or(false, |set| set.contains(&win[1])))
}
//-----------------------------------------------------

/// 753.破解保险箱(DFS,图,欧拉回路)
// 有一个需要密码才能打开的保险箱。密码是 n 位数, 密码的每一位都是范围 [0, k - 1] 中的一个数字。
// 保险箱有一种特殊的密码校验方法,可以随意输入密码序列,保险箱会自动记住 最后 n 位输入,如果匹配,则能够打开保险箱。
// 例如,正确的密码是 "345", 并且你输入的是 "012345" :
// 输入 0 之后,最后 3 位输入是 "0",不正确。
// 输入 1 之后,最后 3 位输入是 "01",不正确。
// 输入 2 之后,最后 3 位输入是 "012",不正确。
// 输入 3 之后,最后 3 位输入是 "123",不正确。
// 输入 4 之后,最后 3 位输入是 "234",不正确。
// 输入 5 之后,最后 3 位输入是 "345",正确,打开保险箱。
// 在只知道密码位数 n 和范围边界 k 的前提下,请你找出并返回确保在输入的 某个时刻 能够打开保险箱的任一 最短 密码序列。
// 输入: n = 2, k = 2
// 输出: "01100"
// 解释: 对于每种可能的密码：
// - "00" 从第 4 位开始输入
// - "01" 从第 1 位开始输入
// - "10" 从第 3 位开始输入
// - "11" 从第 2 位开始输入
// 因此 "01100" 可以确保打开保险箱。"01100"、"10011" 和 "11001" 也可以确保打开保险箱。
pub fn crack_safe(n: i32, k: i32) -> String {
    //Hierholzer 算法可以在一个欧拉图中找出欧拉回路
    fn dfs(node: i32, highest: i32, k: i32, seen: &mut HashSet<i32>, ans: &mut String) {
        for x in 0..k {
            let nei = node * 10 + x;
            if !seen.contains(&nei) {
                seen.insert(nei);
                dfs(nei % highest, highest, k, seen, ans);
                ans.push((u8::try_from(x).expect("u8 error") + b'0') as char);
                // println!("{:?}",ans);
            }
        }
    }

    let mut seen = HashSet::new();
    let mut ans = String::new();
    let highest = 10_i32.pow(n as u32 - 1);
    dfs(0, highest, k, &mut seen, &mut ans);
    ans.push_str(&"0".repeat((n - 1) as usize));

    ans
}
//-----------------------------------------------------