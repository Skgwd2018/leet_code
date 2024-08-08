use std::collections::{HashMap, HashSet};

/// 2215.找出两数组的不同(数组,哈希表)
/// 使用哈希集合解决去重复问题
pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    // let set1: HashSet<&i32> = nums1.iter().collect();
    let set1: HashSet<i32> = nums1.into_iter().collect(); // 效率更高
    let set2: HashSet<i32> = nums2.into_iter().collect();

    // let unique_in_nums1: Vec<i32> = nums1.into_iter().filter(|&x| !set2.contains(&x)).collect();
    // let unique_in_nums1: Vec<i32> = nums1.iter().filter(|x| !set2.contains(x)).cloned().collect();
    // let unique_in_nums1: Vec<i32> = unique_in_nums1.into_iter().collect::<HashSet<_>>().into_iter().collect();

    // 调用 HashSet 的 difference() 方法,返回一个迭代器,其中包含所有在 set1 中但不在 set2 中的元素。
    // set1.difference(&set2).map(|&x| x).collect(); // 这种方式会更高效
    // set1.difference(&set2).cloned().collect();    // 作用同上
    // .cloned() 是一个适配器,它会对迭代器中的每个元素调用 clone() 方法。
    // 使用 .map() 方法来克隆每个元素。.map() 方法接受一个闭包,该闭包对迭代器中的每个元素进行转换。
    // 在这里,闭包 |&x| x 实际上并没有改变元素,因为它只是借用并返回了元素本身。
    // 因此,这里的 map 操作实际上并没有做额外的工作,只是简单地返回了元素的引用
    // vec![set1.difference(&set2).cloned().collect(), set2.difference(&set1).map(|&x| x).collect()]

    // .copied() 是一个用于复制迭代器中原始元素值的适配器。.copied()的作用和上面的.map(|&x| x)是一样的
    // 它通常用于原始元素是 Copy trait 的实现者的情况,这意味着这些元素可以通过简单的位复制来复制,而不是通过调用 clone() 方法。
    // 这通常比 clone() 克隆更高效,因为位复制通常比调用 clone() 方法更快。
    // .cloned() 用于克隆实现了 Clone trait 的元素。
    // .copied() 用于复制实现了 Copy trait 的元素,这通常比克隆更高效。
    // 在选择使用哪个适配器时:
    // 如果元素是 Copy 的,使用 .copied() 通常是更好的选择。
    // 如果元素不是 Copy 的,但实现了 Clone trait,那应该使用 .cloned()。
    vec![set1.difference(&set2).copied().collect(), set2.difference(&set1).copied().collect()]
}
//-----------------------------------------------------

/// 1207.独一无二的出现次数(数组,哈希表)
/// 使用哈希集合解决统计出现次数问题
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    // 存储每个数的出现次数的集合
    let mut count_map = HashMap::new();
    /*for num in arr {
        // 使用 entry() 方法检查键 num 是否已经存在于 count_map 中,
        // 如果键 num 不存在,or_insert(0) 会将键 num 插入到哈希映射中,并设置其对应的值为 0。
        // 如果遇到相同的键时,就可以在其现有值的基础上增加计数。
        *count_map.entry(num).or_insert(0) += 1;
    }*/
    // for_each() 方法,用于遍历迭代器中的每个元素,并对每个元素执行一个给定的操作。
    arr.into_iter().for_each(|num| *count_map.entry(num).or_insert(0) += 1);

    // .iter() 方法返回一个对原始数组 arr 的引用迭代器,即它不会消耗或移动 arr 中的数据。
    // .into_iter() 方法将 arr 转换为一个拥有权的迭代器,即它消耗了 arr,并且 arr 在迭代之后不再可用。
    // 如果你在迭代之后仍然需要访问或使用 arr,那么显然应该选择 .iter(),因为 into_iter() 会消耗 arr。
    // 如果迭代之后不再需要 arr,那么理论上 .into_iter() 可能会稍微快一些,因为它避免了引用计数的操作(如果 arr 是一个引用类型的话)。在大多数实际场景中差异都是可以忽略不计。
    // 但是当迭代的数据元素体积比较大时,.iter()的性能反而会更高

    // 存储出现次数的集合
    let mut occurrences = HashSet::new();
    /*for count in count_map.values() {
        // 将值添加到HashSet集合中。
        // 返回值:是否是新插入的。即:如果集合以前不包含此值,则返回true;如果集合已经包含此值,则返回false;并且不修改集合:不替换原始值,并删除作为参数传递的值
        if !occurrences.insert(*count) { return false; }
    }
    true */
    // all() 方法:用于检查迭代器中的所有元素是否都满足给定的条件.检查 count_map 中的每个出现次数(即值)是否都是唯一的。
    // 如果所有出现次数都成功插入到 occurrences 中,all() 方法将返回 true。
    // 如果有任何出现次数已经存在于 occurrences 中,all() 方法将立刻返回 false。
    count_map.values().all(|count| occurrences.insert(count))
}
//-----------------------------------------------------

/// 1657.确定两个字符串是否接近(字符串,哈希表,计数)
// 题目要求:字符串中的内容全是小写字母
// 如果可以使用以下操作从一个字符串得到另一个字符串,则认为两个字符串'接近':
// 操作1:交换任意两个 现有 字符。
// 例: abcde -> aecdb
// 操作2:将一个 现有 字符的每次出现转换为另一个 现有 字符,并对另一个字符执行相同的操作。
// 例如: aacabb -> bbcbaa (所有 a 转化为 b,而所有的 b 转换为 a)
// 你可以根据需要对任意一个字符串多次使用这两种操作。
// 给你两个字符串,word1 和 word2。如果 word1 和 word2 接近,就返回 true;否则,返回 false
// 输入: word1 = "cabbba", word2 = "abbccc"
// 输出: true
// 解释: 3 次操作从 word1 获得 word2 。
// 执行操作 1: "cabbba" -> "caabbb"
// 执行操作 2: "caabbb" -> "baaccc"
// 执行操作 2: "baaccc" -> "abbccc"
// 这个函数的目标是判断两个字符串是否“接近”。
// 具体来说即是:两个字符串长度相同,并且对于每个字母,两个字符串中该字母的出现次数相同(无论顺序如何)
pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() { return false; }

    let mut word1_cnt = [0; 26]; // 字符计数:用于存储字符串中每个字母的出现次数
    // 显式循环:编译器可能更容易对其进行优化,因为它直接反映了循环的意图,没有额外的抽象层。
    /*for c in word1.as_bytes() {
        word1_cnt[(c - b'a') as usize] += 1;
    }*/
    // 迭代器链:利用了 Rust 的迭代器抽象,使得代码更加函数式。
    // 虽然迭代器链提供了更多的灵活性(即可以很容易地添加额外的操作到链中),但也可能引入一些微小的运行时开销,因为每次调用迭代器方法时都可能涉及到一些额外的函数调用。
    // 在某些情况下,编译器可能不如处理显式循环那样优化迭代器链。
    // 总结:迭代器链方式的内存消耗会相对较少,显式循环的运行会较快,但差异非常微小
    word1.as_bytes().iter().for_each(|c| word1_cnt[(c - b'a') as usize] += 1);
    let mut word2_cnt = [0; 26]; // 字符计数
    word2.as_bytes().iter().for_each(|c| word2_cnt[(c - b'a') as usize] += 1);
    // 检查零计数,当含有不同字母时就退出
    for i in 0..26 {
        if (word1_cnt[i] == 0) != (word2_cnt[i] == 0) { return false; }
    }

    // sort:对切片进行稳定的排序,即如果两个元素相等,它们在排序后的相对顺序会保持不变。
    // 由于它保证了稳定性,所以通常比 sort_unstable 慢一些,因为它需要额外的内存来保持元素的相对顺序。
    // sort_unstable:对切片进行排序,如果两个元素相等,它们在排序后的相对顺序可能会改变。
    // 由于它不需要保证稳定性,所以通常比 sort 快一些,因为它可以采用更高效的排序算法。
    word1_cnt.sort_unstable();
    word2_cnt.sort_unstable();

    word1_cnt == word2_cnt
}
//-----------------------------------------------------

/// 2352.相等行列对(数组,哈希,矩阵,模拟)
// 给定一个下标从 0 开始、大小为 n x n 的整数矩阵 grid ,返回满足 Ri行 和 Cj列 相等的行列对 (Ri, Cj) 的数目。
// 如果行和列以相同的顺序包含相同的元素(即相等的数组),则认为二者是相等的。
// 存在三对相等行列对:
// - (第 0 行, 第 0 列): [3, 1, 2, 2]
// - (第 2 行, 第 2 列): [2, 4, 2, 2]
// - (第 3 行, 第 2 列): [2, 4, 2, 2]
pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let (mut cnt, mut cache_map) = (0, HashMap::new());
    // 遍历行,将其作为 key,行出现的次数为 value 存入 HashMap
    grid.iter().for_each(|g| *cache_map.entry(g).or_insert(0) += 1);
    // 遍历列,找到与之匹配的行,累加对应的计数
    for i in 0..grid.len() {
        let curr: Vec<i32> = (0..grid.len()).map(|j| grid[j][i]).collect();
        cnt += cache_map.get(&curr).unwrap_or(&0);
    }

    cnt

    // 解法二:
    /*let n = grid.len();
    let mut column_vec = vec![0; n];
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            column_vec[j] = grid[j][i];
        }
        cnt += grid.iter().filter(|&x| x == &column_vec).count();
    }
    cnt as i32*/
}
//-----------------------------------------------------

/// 149.直线上最多的点数(几何,数学,数组,哈希表)
// 给定一个数组 points,其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点。求最多有多少个点在同一条直线上。
// points 中的所有点 互不相同
// 输入: points = [[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]
// 输出: 4
pub fn max_points(mut points: Vec<[i32; 2]>) -> i32 {
    let mut counter = 0;
    points.sort_unstable();
    let mut s = HashMap::<i32, i32>::with_capacity(points.len() / 2 + 50);
    while let Some(x) = points.pop() {
        for i in &points {
            // Inf 的符号不会发生改变,不必处理+-Inf的问题
            // *s.entry(unsafe {
            //     std::mem::transmute::<f32, i32>((x[1] - i[1]) as f32 / (x[0] - i[0]) as f32)
            // }).or_insert(0) += 1;
            *s.entry(((x[1] - i[1]) as f32 / (x[0] - i[0]) as f32).to_bits() as i32).or_insert(0) += 1;
        }
        counter = s.drain().map(|x| x.1).max().unwrap_or(counter).max(counter);

        if counter > i32::try_from(points.len()).expect("i32 error") / 2 { break; }
    }

    counter + 1
}
//-----------------------------------------------------