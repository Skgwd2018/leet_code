// Rc(引用计数)是一个用于管理堆上数据的所有权的智能指针。它提供了一种在多个所有者之间共享数据的方式，而无需将数据复制到每个所有者。
// Rust的所有权模型要求每个值都有一个明确的所有者，当所有者离开作用域时，该值将被自动释放。这有助于防止内存泄漏和其他常见的内存安全问题。
// 在某些情况下，需要多个变量共享同一数据的所有权，而不是复制数据。这就是Rc发挥作用的地方。
// Rc通过在内部维护一个引用计数来实现共享所有权。每当一个新的Rc引用被创建时，引用计数就会增加。
// 当Rc引用离开作用域或被丢弃时，引用计数就会减少。当引用计数变为零时，Rc将自动释放其管理的数据。
// let five = Rc::new(5);
// let shared_five = Rc::clone(&five);  // 此时 five 和 shared_five 共享同一份数据

// RefCell 用于在运行时检查借用规则，并允许在某些情况下绕过Rust的静态借用检查。
// 即存在不可变引用的情况下，RefCell允许你获取可变引用，从而可以在不破坏Rust的安全性和所有权规则的前提下，
// 实现在不改变数据结构的情况下进行内部修改，即实现内部可变性(interior mutability)。
// RefCell 通过提供 borrow() 和 borrow_mut() 方法，允许你借用值的不可变引用和可变引用。
// 这些方法在运行时执行借用规则的检查，并在违反规则时引发运行时错误。虽然RefCell提供了更大的灵活性，但如果不正确地使用，也会导致程序在运行时崩溃。
// 此外，RefCell 与 Cell 类似，都是用于实现内部可变性的类型，但两者在使用上有存在差异。
// Cell只能用于实现 Copy trait 的类型，因为Cell的值在赋值时可以进行复制。而RefCell则可以用于非 Copy trait 类型，因为它的值在赋值时不会进行复制

use std::cell::RefCell;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::mem::swap;
use std::rc::Rc;

/// 二叉树节点
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    // 在Rust中，#[inline]属性是一个提示给编译器的，建议它尽可能内联指定的函数或方法。
    // 内联是将函数体直接插入到调用该函数的地方，而不是进行常规的函数调用。这有助于减少函数调用的开销，但可能会增加生成的代码的大小。
    // #[inline]属性主要用于优化那些频繁调用的小函数，或者是那些对性能有严格要求的代码。然而，编译器并不一定会遵循这个提示，它会在权衡代码大小和性能之后做出决策。
    // 注:过度使用内联可能会导致生成的代码过大，反而降低缓存效率，甚至影响性能。#[inline]应该谨慎使用，并且通常只在确定某个函数应该被内联的情况下使用。
    // 其实Rust编译器有自己的内联策略，它会自动决定哪些函数应该被内联。即使你不使用#[inline]属性，编译器也可能会自动内联一些函数。
    // 总结: #[inline]是一个优化提示，它告诉编译器可能希望某个函数被内联。最终是否内联这个函数，还是由编译器根据代码的具体情况来决定的。
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    /// 104.二叉树的最大深度(深度优先搜索问题)
    // 二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 递归操作
        match root {
            None => 0,
            Some(node) => {
                // node.borrow().left.to_owned() 相比 node.borrow().left.clone() 效率更高
                // node.borrow().left.clone() 会调用 Option和Rc<RefCell<T>> 的 clone() 方法，
                // node.borrow().left.to_owned() 会调用 Option 的 cloned() 方法,然后返回一个新的 Option，
                // 其中的 Rc<RefCell<TreeNode>> 也是通过克隆得到的。
                // to_owned():是一个Option<T>的方法,它用于将Option<T>中的值转换为其拥有的版本。
                // 如果Option是Some(value),那么to_owned()会返回该值的克隆;如果Option是None,则to_owned()会返回None。
                // Rc<T> 的 clone() 方法实现的是引用计数加1的操作,是一个快速的 O(1) 复杂度的操作。
                // 因此,无论是 clone() 还是 to_owned(),都需要对 Rc<RefCell<TreeNode>> 进行克隆,性能上非常接近。
                1 + cmp::max(
                    Self::max_depth(node.borrow().left.to_owned()),
                    Self::max_depth(node.borrow().right.to_owned()),
                )
            }
        }
    }

    /// 872.叶值序列相似的树(深度优先搜索问题)
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 函数内部定义的函数称为闭包(Closure)或局部函数(Local Function)。
        // pre_order() 函数实际上是在 leaf_similar() 函数内部定义的局部函数。局部函数与闭包相似，但有区别：
        // 局部函数:是一个在另一个函数内部定义的命名函数。它们的行为类似于常规函数,并且可以访问其包含作用域中的变量。
        // 闭包:是一个匿名函数，可以捕获其环境中的变量。闭包通常用于实现高阶函数,它们可以像其他值一样传递。
        //      闭包通常使用 | 符号来定义，并可以捕获其外部的变量。
        /*fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                // 判断是否是最底部的节点
                if left.is_none() && right.is_none() {
                    values.push(node.borrow().val);
                }

                pre_order(left, values);
                pre_order(right, values);
            }
        }*/

        let (mut values1, mut values2) = (Vec::new(), Vec::new());
        // pre_order(root1, &mut values1);
        // pre_order(root2, &mut values2);
        // 这种局部函数的实现方式性能比下面的dfs()方法的实现方式高,但是 .take() 会取走值(即修改了节点),比较函数的耗时也会长一点
        Self::dfs(root1, &mut values1);
        Self::dfs(root2, &mut values2);

        values1 == values2
    }
    // DFS是深度优先搜索(depth first search),是用递归进行搜索,尽可能深的搜索每一个节点。
    // 可以理解为不撞墙不回头,主要用于解决一些树的遍历和图的遍历问题。由于是通过递归实现,时间复杂度较高,一般用于数据较小的情况。
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            if left.is_none() && right.is_none() {
                values.push(Rc::clone(&node));
            } else {
                Self::dfs(left.clone(), values);
                Self::dfs(right.clone(), values);
            }
        }
    }

    /// 二叉搜索树(BST)中搜索一个特定的值
    // BST的特性,即对于树中的每个节点,其左子节点的值都小于该节点的值,而右子节点的值都大于该节点的值。
    // 根据这个特性,可以从根节点开始,如果当前节点的值等于目标值,则返回当前节点;
    // 如果目标值小于当前节点的值,则递归地在左子树中搜索;如果目标值大于当前节点的值,则递归地在右子树中搜索。
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // 递归搜索
        /*match root {
            None => None,
            Some(node) => {
                let node_val = node.borrow().val;
                match node_val.cmp(&val) {
                    cmp::Ordering::Equal => return Some(node),
                    cmp::Ordering::Greater => Self::search_bst(node.borrow().left.clone(), val),
                    cmp::Ordering::Less => Self::search_bst(node.borrow().right.clone(), val),
                }
            }
        }*/

        // 循环搜索,相比上面内存消耗更小,执行效率更高一点
        let mut node = root;
        while let Some(curr_node) = node {
            let cur_val = curr_node.borrow().val;
            match cur_val.cmp(&val) {
                Ordering::Equal => return Some(curr_node),
                Ordering::Greater => node = curr_node.borrow().left.clone(),
                Ordering::Less => node = curr_node.borrow().right.clone(),
            };
        }
        None
    }

    /// 450.删除二叉搜索树中的节点
    // 给定一个二叉搜索树的根节点 root 和一个值 key,删除二叉搜索树中的 key 对应的节点,并保证二叉搜索树的性质不变。
    // 返回二叉搜索树(有可能被更新)的根节点的引用。
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // 局部函数(Local Function)
        fn dfs5(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
            // if root.is_none() { return None; }
            root.as_ref()?; // 同上
            let mut node = root.as_ref().unwrap().borrow_mut();
            // 递归查找与key值相同的节点,同时设置左、右节点等于返回的子树
            match node.val.cmp(&key) {
                Ordering::Greater => node.left = dfs5(node.left.take(), key),
                Ordering::Less => node.right = dfs5(node.right.take(), key),
                Ordering::Equal => {
                    // 找到目标节点，分别处理以下4种情况:
                    // 节点只有右子树，返回右子树;
                    // 节点只有左子树，返回左子树;
                    // 节点同时有左、右子树，找到右子树的最小值的节点，将该节点的左子树设置为当前及节点的左子树，返回当前节点的右子树;
                    // 节点没有左、右节点，返回None;
                    return match (node.left.is_none(), node.right.is_none()) {
                        (true, false) => node.right.take(),
                        (false, true) => node.left.take(),
                        (false, false) => {
                            let mut min_code = node.right.clone().unwrap();
                            while min_code.borrow_mut().left.is_some() {
                                let t = min_code.borrow_mut().left.clone();
                                min_code = t.unwrap();
                            }
                            min_code.borrow_mut().left = node.left.take();

                            node.right.take()
                        }
                        _ => None,
                    };
                }
            };

            root.clone()
        }

        dfs5(root, key)
    }

    /// 1448.统计二叉树中好节点的数目(深度优先搜索问题)
    // 即从根节点开始遍历到某个节点,并且始终保持当前遍历到的节点的值是非递减的,那么该节点就是一个好节点。
    // 根节点一定是好节点,例：3 -> 4 -> 5, 3个节点都是好节点，3 -> 1 -> 3, 则3和3都是好节点，1不是;
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // i32::MIN 是 i32 类型的一个常量,表示 i32 类型可以表示的最小值,用来确保在遍历树的根节点之前,没有节点的值会大于这个初始值
        Self::dfs2(root, i32::MIN)
    }
    fn dfs2(node: Option<Rc<RefCell<TreeNode>>>, max_val: i32) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                // let count = if node.val >= max_val { 1 } else { 0 };
                let count = i32::from(node.val >= max_val);
                let max_val = node.val.max(max_val);

                count + Self::dfs2(node.left.clone(), max_val) + Self::dfs2(node.right.clone(), max_val)
            }
        }
    }

    /// 437.二叉树路径总和Ⅲ(深度优先搜索问题,回溯操作)
    // 给定一个二叉树的根节点 root,和一个整数 targetSum,求该二叉树里节点值之和等于 targetSum 的 路径 的数目。
    // 路径 不需要从根节点开始,也不需要在叶子节点结束,但是路径方向必须是向下的(只能从父节点到子节点)。
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut prefix_sum: HashMap<i64, i32> = HashMap::new();
        // 由于可以选择完整的路径,因此要把0预先插入
        prefix_sum.insert(0, 1);
        match root {
            None => 0,
            Some(root) => Self::dfs3(root, 0, i64::from(target_sum), &mut prefix_sum),
        }
    }
    fn dfs3(node: Rc<RefCell<TreeNode>>, curr_sum: i64, target_sum: i64, prefix_sum: &mut HashMap<i64, i32>) -> i32 {
        let curr_sum = curr_sum + i64::from(node.borrow().val);
        let mut count = 0;
        // 以当前节点为终点,查找是否有满足要求的前缀和
        if let Some(&val) = prefix_sum.get(&(curr_sum - target_sum)) {
            count += val;
        }

        // .or_insert() 方法返回这个 key 的 value 的一个可变引用(&mut V),就是将该key对应的value +1
        // 当前前缀和计数+1
        *prefix_sum.entry(curr_sum).or_insert(0) += 1;
        // 继续向下搜索
        if let Some(left) = node.borrow().left.clone() {
            count += Self::dfs3(left, curr_sum, target_sum, prefix_sum);
        }
        if let Some(right) = node.borrow().right.clone() {
            count += Self::dfs3(right, curr_sum, target_sum, prefix_sum);
        }
        // 回溯作用,即在处理完当前节点后,需要将 curr_sum 对应的值(路径数量)减一,以反映已经离开了这个节点,不再考虑从它开始的路径。
        // 常用于在深度优先搜索中正确地更新状态。
        *prefix_sum.entry(curr_sum).or_insert(0) -= 1;

        count
    }

    /// 1372.二叉树中的最长交错路径(DFS,动态规划)
    // 给定一棵以 root 为根的二叉树，二叉树中的交错路径定义如下：
    // 选择二叉树中 任意 节点和一个方向（左或者右）。
    // 如果前进方向为右，那么移动到当前节点的的右子节点，否则移动到它的左子节点。
    // 改变前进方向：左变右或者右变左。
    // 重复第二步和第三步，直到你在树中无法继续移动。
    // 交错路径的长度定义为：访问过的节点数目 - 1(单个节点的路径长度为 0)。
    // 返回给定树中最长 交错路径 的长度。
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs6(&root, false, 0)
    }
    // direct left:false, right:true
    fn dfs6(root: &Option<Rc<RefCell<TreeNode>>>, direct: bool, curr_sum: i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let (left, right) = if direct {
                    (Self::dfs6(&node.borrow().left, false, curr_sum + 1),
                     Self::dfs6(&node.borrow().right, true, 1))
                } else {
                    (Self::dfs6(&node.borrow().left, false, 1),
                     Self::dfs6(&node.borrow().right, true, curr_sum + 1))
                };

                curr_sum.max(left.max(right))
            }
        }
    }

    /// 236.二叉树的最近公共祖先(DFS)
    // 给定一个二叉树, 找到该树中两个指定节点的最近公共祖先。
    // 最近公共祖先的定义为:对于有根树 T 的两个节点 p、q，最近公共祖先表示为一个节点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大(一个节点也可以是它自己的祖先)。
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q { return root; }

        let node_ref = root.as_ref().unwrap();
        let left = Self::lowest_common_ancestor(node_ref.borrow_mut().left.take(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(node_ref.borrow_mut().right.take(), p, q);
        if left.is_some() && right.is_some() { return root; }

        if left.is_some() { left } else { right }
    }

    /// 199.二叉树的右视图(广度优先搜索),BFS是广度优先搜索（breadth first search）
    // 给定一个二叉树的 根节点root,想象自己站在它的右侧,按照从顶部到底部的顺序,返回从右侧能看到的节点值。
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // 深度优先搜索
        /*fn dfs4(root: &TreeNode, result: &mut Vec<i32>, deep: usize) {
            if deep > result.len() {
                result.push(root.val)
            }
            if let Some(right) = root.right.as_ref() {
                let node = right.borrow();
                dfs4(&node, result, deep + 1);
            }
            if let Some(left) = root.left.as_ref() {
                let node = left.borrow();
                dfs4(&node, result, deep + 1);
            }
        }
        if root.is_none() { return vec![]; }
        let mut result = Vec::with_capacity(8);
        dfs4(&root.unwrap().borrow(), &mut result, 1);
        result */

        // 广度优先搜索
        let mut result = vec![];
        let mut dequeue = VecDeque::new(); // 用于bfs遍历
        dequeue.push_back((root, 0));
        while let Some((node, depth)) = dequeue.pop_front() {
            if let Some(node) = node {
                let node = node.borrow();
                // 检查 result 的长度是否小于当前深度加1。如果是,即到达了新的层级,直接将当前节点的值推入 result;
                // 如果 result 向量的长度不小于当前深度加1,即已经在当前层级有了一个节点值。由于是从右侧查看,所以当前节点的值将替换掉之前存储的该层级的节点值。
                if result.len() < depth + 1 {
                    result.push(node.val);
                } else {
                    result[depth] = node.val;
                }
                // 将当前节点的左子节点和右子节点(如果存在的话)以及它们的深度(当前深度加1)压入队列的尾部，以便在下一次循环中处理。
                dequeue.push_back((node.left.clone(), depth + 1));
                dequeue.push_back((node.right.clone(), depth + 1));
            }
        }
        // 对于每一层,只保留最右侧的节点值,因为后面的节点会覆盖前面的节点值.当遍历完成时,result 向量将包含从右侧可见的所有节点值。
        result
    }

    /// 1161.最大层内元素和(广度优先搜索)
    // 给你一个二叉树的根节点 root。设根节点位于二叉树的第 1 层,而根节点的子节点位于第 2 层,依此类推。
    // 请返回层内元素之和 最大 的那几层(可能只有一层)的层号，并返回其中 最小 的那个。
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        // result:用于存储最大层级和对应的层级。
        // curr_sum:用于计算当前层级的和。
        // curr_level:用于记录当前处理的层级。
        // maximum:用于记录当前已知的最大层级和。
        let (mut result, mut curr_sum, mut curr_level, mut maximum) = (0, 0, 1, i32::MIN);
        // queue 和 next: 这两个队列用于实现广度优先搜索（BFS）。queue 用于存储当前层级的节点，而 next 用于存储下一层级的节点。
        // 使用两个队列 queue 和 next 来交替存储当前层级和下一层级的节点。queue 初始时只包含根节点。
        let (mut queue, mut next) = (VecDeque::new(), VecDeque::new());
        queue.push_back(root.unwrap());

        while let Some(curr_node) = queue.pop_front() {
            curr_sum += curr_node.as_ref().borrow().val;
            if curr_node.as_ref().borrow().left.is_some() {
                next.push_back(curr_node.as_ref().borrow().left.as_ref().unwrap().clone());
            }
            if curr_node.as_ref().borrow().right.is_some() {
                next.push_back(curr_node.as_ref().borrow().right.as_ref().unwrap().clone());
            }

            // 层级切换操作
            // 当 queue 为空时,表示当前层级的所有节点已经处理完毕,需要切换到下一层级
            if queue.is_empty() {
                // 交换 queue 和 next,使得 queue 指向下一层级的节点,而 next 清空准备存储下一批节点
                swap(&mut queue, &mut next);
                if curr_sum > maximum {
                    maximum = curr_sum;
                    result = curr_level;
                }
                curr_level += 1;
                curr_sum = 0;
            }
        }

        result
    }
}
//-----------------------------------------------------

/// LCR 055.二叉搜索树迭代器
// 实现一个二叉搜索树迭代器类BSTIterator，表示一个按中序遍历二叉搜索树（BST）的迭代器：
// BSTIterator(TreeNode root) 初始化 BSTIterator 类的一个对象。
// BST 的根节点 root 会作为构造函数的一部分给出。指针应初始化为一个不存在于 BST 中的数字，且该数字小于 BST 中的任何元素。
// boolean hasNext() 如果向指针右侧遍历存在数字，则返回 true ；否则返回 false 。
// int next()将指针向右移动，然后返回指针处的数字。
// 注意，指针初始化为一个不存在于 BST 中的数字，所以对 next() 的首次调用将返回 BST 中的最小元素。
// 可以假设 next() 调用总是有效的，也就是说，当调用 next() 时，BST 的中序遍历中至少存在一个下一个数字。
pub struct BSTIterator {
    index: usize,
    data: Vec<i32>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut data = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                data.push(n.borrow().val);
                node = n.borrow_mut().right.take();
            }
        }

        Self { index: 0, data }
    }

    pub fn next(&mut self) -> i32 {
        if self.index == self.data.len() {
            return -1;
        }

        let v = self.data[self.index];
        self.index += 1;
        v
    }

    pub fn has_next(&self) -> bool {
        self.index < self.data.len()
    }
}
//-----------------------------------------------------