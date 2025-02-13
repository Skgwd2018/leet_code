use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet};

/// 无限集合
// 使用堆(优先队列解决问题)
// 由于题目要求一个包含所有正整数的无限集合,所有值存储起来很费内存。
// 可以使用一个指针来代替,表示后续所有连续正整数的起点,而新加入的更小的值,如果是集合不再连续,可以存储在一个最小堆中,极大减少内存。
// 判断最小堆是否为空,是则返回堆中最小值,否则使用指针所表示的值,并使指针后移一位
// 加入的值是否使指针连续,是则指针前移,否则判断加入值是否小于当前断点,并且不在堆中,是则加入堆中
/*pub struct SmallestInfiniteSet {
    smallest: i32,
    heap: BinaryHeap<Reverse<i32>>,
}

impl SmallestInfiniteSet {
    // 题目要求:现有一个包含所有正整数的集合 [1, 2, 3, 4, 5, ...] 。无限集合
    // 1 <= num <= 1000
    pub fn new() -> Self {
        SmallestInfiniteSet { smallest: 1, heap: BinaryHeap::new() }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        match self.heap.pop() {
            Some(Reverse(min_value)) => min_value,
            None => {
                self.smallest += 1;
                self.smallest - 1
            }
        }
    }

    pub fn add_back(&mut self, num: i32) {
        if num + 1 == self.smallest {
            self.smallest -= 1;
        } else if num + 1 < self.smallest && !self.heap.iter().any(|&x| x == Reverse(num)) {
            self.heap.push(Reverse(num));
        }
    }
}*/

// BTreeSet 主要特点:
// 1.有序: BTreeSet 会自动对其元素进行排序。当你遍历集合时,元素会按照升序排列。
// 2.不重复: 与所有集合类型一样,BTreeSet 不允许重复的元素。
// 3.快速查找: 由于基于 B 树实现,BTreeSet 提供了快速的查找、插入和删除操作。
// 注意: BTreeSet 的排序是基于元素的默认排序。对于自定义类型,可能需要实现 Ord trait 来定义如何排序这些元素。
#[derive(Default)]
pub struct SmallestInfiniteSet {
    point: i32,
    set: BTreeSet<i32>,
}

impl SmallestInfiniteSet {
    // 题目要求:现有一个包含所有正整数的集合 [1, 2, 3, 4, 5, ...] 。无限集合
    // 1 <= num <= 1000
    pub fn new() -> Self {
        Self { point: 1, set: BTreeSet::new() }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if let Some(&value) = self.set.iter().next() {
            self.set.remove(&value);

            value
        } else {
            self.point += 1;

            self.point - 1
        }
    }

    pub fn add_back(&mut self, num: i32) {
        if num < self.point {
            self.set.insert(num);
        }
    }
}
//-----------------------------------------------------

/// 2542.最大子序列的分数(贪心+排序+堆(优先队列-最小堆),数组)
// 给定两个下标从 0 开始的整数数组 nums1 和 nums2,两者长度都是 n,再给一个正整数 k。必须从 nums1 中选一个长度为 k 的 子序列 对应的下标。
// 对于选择的下标 i0, i1, ..., ik - 1, 你的 分数 定义如下:
// nums1 中下标对应元素求和,乘以 nums2 中下标对应元素的 最小值。
// 用公式表示: (nums1[i0] + nums1[i1] +...+ nums1[ik - 1]) * min(nums2[i0], nums2[i1], ... , nums2[ik - 1])
// 返回 最大 可能的分数。
// 一个数组的 子序列 下标是集合 {0, 1, ..., n-1} 中删除若干元素得到的剩余集合,也可以不删除任何元素。
// 输入: nums1 = [1, 3, 3, 2], nums2 = [2, 1, 3, 4], k = 3
// 输出: 12
// 解释:
// 四个可能的子序列分数为:
// - 选择下标 0, 1, 2, 得到分数 (1+3+3) * min(2,1,3) = 7
// - 选择下标 0, 1, 3, 得到分数 (1+3+2) * min(2,1,4) = 6
// - 选择下标 0, 2, 3, 得到分数 (1+3+2) * min(2,3,4) = 12
// - 选择下标 1, 2, 3, 得到分数 (3+3+2) * min(1,3,4) = 8
// 所以最大分数为 12
pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: usize) -> i64 {
    /*let n = nums1.len();
    let mut ids = (0..n).collect::<Vec<_>>();
    // 对下标排序
    ids.sort_unstable_by(|&i, &j| nums2[j].cmp(&nums2[i]));

    let mut h = BinaryHeap::new();
    let mut sum = 0;
    for i in 0..k {
        // 类型转换:使用 i64::from(x) 比 x as i64 更安全,但是消耗内存略高
        sum += i64::from(nums1[ids[i]]);
        h.push(-nums1[ids[i]]); // 转换成最小堆
    }

    let mut answer = sum * i64::from(nums2[ids[k - 1]]);
    for i in k..n {
        let x = nums1[ids[i]];
        if x > -h.peek().unwrap() {
            sum += i64::from(x + h.pop().unwrap());
            h.push(-x);
            answer = answer.max(sum * i64::from(nums2[ids[i]]));
        }
    }
    answer*/

    // 解法二:
    // 合并nums1、 nums2得到数组vec,并按照nums2元素的大小对vec进行降序排序。
    let mut vec: Vec<(i32, i32)> = nums1.into_iter().zip(nums2).collect();
    vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    let mut heap = BinaryHeap::<Reverse<i32>>::new();
    // 按照vec的顺序计算前k个nums1元素和sum,并且将其放入最小堆中,并计算出迭代前初始答案answer
    let mut sum: i64 = vec[..k].iter().map(|p| {
        heap.push(Reverse(p.0));
        i64::from(p.0)
    }).sum();
    let mut answer = sum * i64::from(vec[k - 1].1);

    // 迭代更新answer
    for (x, y) in &vec[k..] {
        if *x > heap.peek().unwrap().0 {
            sum += i64::from(*x - heap.pop().unwrap().0);
            heap.push(Reverse(*x));
            answer = answer.max(sum * i64::from(*y));
        }
    }

    answer
}
//-----------------------------------------------------

/// 堆/优先队列
// 题目:给你一个下标从 0 开始的整数数组 costs,其中 costs[i] 是雇佣第 i 位工人的代价。
// 同时给两个整数 k 和 candidates。想根据以下规则恰好雇佣 k 位工人：
// 总共进行 k 轮雇佣,且每一轮恰好雇佣一位工人。
// 在每一轮雇佣中,从最前面 candidates 和最后面 candidates 人中选出代价最小的一位工人,如果有多位代价相同且最小的工人,选择下标更小的一位工人。
// 例: costs = [3, 2, 7, 7, 1, 2] 且 candidates = 2,第一轮雇佣中,我们选择第 4 位工人,因为他的代价最小 [3, 2, 7, 7, 1, 2]。
// 第二轮雇佣,我们选择第 1 位工人,因为他们的代价与第 4 位工人一样都是最小代价,而且下标更小,[3, 2, 7, 7, 2]。注意每一轮雇佣后,剩余工人的下标可能会发生变化。
// 如果剩余员工数目不足 candidates 人,那么下一轮雇佣他们中代价最小的一人,如果有多位代价相同且最小的工人,选择下标更小的一位工人。
// 一位工人只能被选择一次。
// 返回雇佣恰好 k 位工人的总代价。
pub fn total_cost(mut costs: Vec<i32>, k: usize, candidates: usize) -> i64 {
    let n = costs.len();
    // let (k, candidates) = (k as usize, candidates as usize);
    // println!("costs ----> {costs:?}"); // [17, 12, 10, 2, 7, 2, 11, 20, 8]
    if 2 * candidates + k > n {
        // costs.sort_unstable();
        // costs.select_nth_unstable(k - 1);
        // println!("cost sort ----> {costs:?}"); // [2, 2, 7, 8, 10, 11, 12, 17, 20]
        // return costs.iter().take(k).map(|&x| x as i64).sum(); // [2, 2, 7] 即 11
        let (l, m, _g) = costs.select_nth_unstable(k - 1);
        return l.iter().map(|&x| i64::from(x)).sum::<i64>() + i64::from(*m);
        // println!("lesser ----> {l:?}");  // [2, 2]
        // println!("median ----> {m}");    // 7
        // println!("greater ----> {g:?}"); // [8, 10, 11, 12, 17, 20]
    }

    // println!("costs ----> {costs:?}"); // [17, 12, 10, 2, 7, 20, 11, 2, 8, 28, 11, 28]
    // 排序后                              // [2, 2, 7, 8, 10, 11, 11, 12, 17, 20, 28, 28]
    // 目的是通过不用完全排序而取出合适的值
    let (mut prev, mut suff) = (BinaryHeap::new(), BinaryHeap::new());
    // Reverse() 用于反转排序顺序的结构体
    // let mut numbers = vec![1, 3, 2];
    // numbers.sort_by_key(|&x| Reverse(x));
    // println!("numbers: {numbers:?}"); // [3, 2, 1]
    // Reverse() 用于逆序存储成本值,可以使 BinaryHeap 按照降序的方式排列,从而可以从堆的顶部取出最大的成本值
    for i in 0..candidates {
        prev.push(Reverse(costs[i])); // 前 candidates 个成本值放入 prev 堆中,并逆序放入(通过 Reverse 结构)
        suff.push(Reverse(costs[n - 1 - i])); // 最后 candidates 个成本值放入 suff 堆中,并逆序放入
    }
    // println!("prev ----> {prev:?}"); // [Reverse(2), Reverse(10), Reverse(12), Reverse(17)]
    // println!("suff ----> {suff:?}"); // [Reverse(8), Reverse(11), Reverse(28), Reverse(28)]
    // 双指针操作
    let (mut i, mut j) = (candidates, n - candidates - 1); // 4 7
    (0..k).map(|_| {
        // .peek() 取出堆中的最大值,由于使用Reverse,所以取出的反而是最小值
        let (p, s) = (prev.peek().unwrap().0, suff.peek().unwrap().0);
        // println!("p ----> {p}"); // 2 7 10
        // println!("s ----> {s}"); // 8 8 8
        if p <= s {
            prev.pop();
            // println!("i: {i}"); // 4 5
            prev.push(Reverse(costs[i]));
            i += 1;
            // println!("i: {i}"); // 5 6
            // println!("prev ----> {prev:?}");
            // [Reverse(7), Reverse(10), Reverse(12), Reverse(17)]
            // [Reverse(10), Reverse(11), Reverse(12), Reverse(17)]
            i64::from(p)
        } else {
            suff.pop();
            // println!("j: {j}"); // 7
            suff.push(Reverse(costs[j]));
            j -= 1;
            // println!("j: {j}"); // 6
            // println!("suff ----> {suff:?}");
            // [Reverse(2), Reverse(11), Reverse(28), Reverse(28)]
            i64::from(s)
        }
    }).sum()
}
//-----------------------------------------------------