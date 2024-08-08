/// 前缀树
/// Trie(发音类似 "try")或者说 前缀树 是一种树形数据结构,用于高效地存储和检索字符串数据集中的键。
/// 这种数据结构有相当多的应用情景,例:自动补完和拼写检查。
#[derive(Default)]
pub struct Trie {
    is_end: bool,
    child: [Option<Box<Trie>>; 26],
}

// Trie trie = new Trie();
// trie.insert("apple");
// trie.search("apple");   // 返回 True
// trie.search("app");     // 返回 False
// trie.startsWith("app"); // 返回 True
// trie.insert("app");
// trie.search("app");     // 返回 True

impl Trie {
    /// 初始化前缀树对象
    pub fn new() -> Self { Self::default() }

    /// 向前缀树中插入字符串 word
    pub fn insert(&mut self, word: String) {
        /* let mut curr = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            curr = curr.child[i].get_or_insert_with(Default::default);
        }
        curr.is_end = true; */

        // .bytes() 和 .as_bytes()
        // 迭代方式:bytes() 返回一个迭代器,而 as_bytes() 返回一个字节切片。
        // 用途:当你需要逐个处理字符串中的UTF-8字节时,可以使用 bytes()。
        //     当你需要直接访问字符串的内部字节表示时(例如进行某种低级操作或检查),可以使用 as_bytes()。
        // 性能:as_bytes() 通常比 bytes() 快,因为它避免了迭代器的创建和逐个元素的生成。
        //     但是,这种差异在大多数情况下可能并不显著,除非你正在处理大量数据或对性能有严格要求。
        // 对于大多数实际场景种,使用引用(&b)或值(b)在这段代码中都不会产生明显的性能差异。特别是在处理像u8这样的小类型时。
        // 只有在内存占用大的数据使用引用才会有性能提升
        word.as_bytes().iter().map(|b| (b - b'a') as usize)
            .fold(self, |node, i| node.child[i].get_or_insert_with(Default::default))
            .is_end = true;
    }

    ///  如果字符串 word 在前缀树中,返回 true(即在检索之前已经插入);否则返回 false
    pub fn search(&self, word: String) -> bool {
        let mut curr = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            match &curr.child[i] {
                None => return false,
                Some(n) => curr = n,
            }
        }

        curr.is_end
    }

    /// 如果之前已经插入的字符串 word 的前缀之一为 prefix,返回 true;否则返回 false
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for i in prefix.bytes().map(|b| (b - b'a') as usize) {
            match &curr.child[i] {
                None => return false,
                Some(n) => curr = n,
            }
        }

        true
    }
}
//-----------------------------------------------------

/// 1268.搜索推荐系统(数组,字符串,字典树,二分查找,排序,堆(优先队列))
/// 推荐商品(字典树)
// 给你一个商品数组 products 和一个字符串 searchWord, products 数组中每个商品都是一个字符串。
// 请你设计一个推荐系统,在依次输入单词 searchWord 的每一个字母后,推荐 products 数组中前缀与 searchWord 相同的最多三个产品。
// 如果前缀相同的可推荐产品超过三个,请按字典序返回最小的三个。
// 请你以二维数组的形式,返回在输入 searchWord 每个字母后相应的推荐商品的列表。
pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut answer = vec![];
    products.sort_unstable();
    // 遍历搜索词的所有可能前缀
    for i in 1..=search_word.len() {
        // let vec = vec![1, 2, 3, 4, 5];
        // let filtered_vec: Vec<i32> = vec.iter().filter(|&x| x % 2 == 0).collect(); // 保留偶数
        // println!("{:?}", filtered_vec); // 输出 [2, 4], 且原始vec集合保持不变
        // let mut vec = vec![1, 2, 3, 4, 5];
        // vec.retain(|&x| x % 2 == 0); // 保留偶数
        // println!("{:?}", vec); // 输出 [2, 4], 直接修改原始vec集合
        // 修改性: .filter() 创建了一个新的迭代器,不修改原始集合；而 .retain() 直接修改原始集合,删除不满足条件的元素。
        // 返回值: .filter() 返回一个迭代器,你可以用它来创建新的集合；.retain() 没有返回值,因为它直接修改了原始集合。
        // 性能：由于 .retain() 直接在原始集合上操作,避免了创建新集合的开销,因此在某些情况下可能更高效。然而,这也意味着你不能保留原始集合的完整状态。
        // 使用场景:如果需要保留原始集合不变,并创建一个新的集合来包含满足条件的元素,那么应该使用 .filter()。
        //          如果希望直接修改原始集合,删除不满足条件的元素,那么应该使用 .retain()。

        // .retain() 方法用于过滤集合(如vec、slice等)中的元素。即只保留满足特定条件的元素
        // 遍历集合中的每个元素,并根据提供的闭包（或函数）的返回值来决定是否保留该元素。如果闭包返回 true,则保留;如果返回 false,则不要。
        // retain() 方法的一个重要特性是就地操作,即直接在原始vec上修改元素,而不是创建一个新的vec。
        // 这通常比创建一个新vec更高效,尤其是当处理大型数据集时。然而,调用 retain() 后,原始vec将被修改,可能不再包含之前所有的元素。
        // 注意:由于 retain() 方法可能会改变vec的长度,因此调用 retain() 之后,任何依赖于原始vec长度的代码都应该小心处理。
        // 此外,如果闭包内部使用了vec的引用或迭代器,并且这些引用或迭代器在 retain() 调用期间可能变得无效,可能会导致未定义的行为.
        // 此处由于闭包只使用了字符串的本地副本,因此没有这个问题
        products.retain(|s| s.starts_with(search_word.get(0..i).unwrap()));
        // 对过滤后的 products,使用 iter().take(3).cloned().collect() 获取前三个元素（如果存在的话）并将其添加到 answer 中。
        answer.push(products.iter().take(3).cloned().collect());
    }

    answer
}
//-----------------------------------------------------