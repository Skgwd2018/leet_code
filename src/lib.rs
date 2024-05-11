use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// 计算特定时间范围内最近的请求
pub struct RecentCounter {
    // VecDeque 与 Vec 的区别
    // 1.高效的两端操作:允许在队列的前端和后端进行高效的插入和删除操作。
    // 即可以使用栈一样在队列的任一端进行push和pop操作，这种特性使其在处理需要频繁在两端进行操作的场景时非常高效。
    // 2.支持FIFO和FILO操作
    // 3.支持随机访问:虽然VecDeque支持随机访问，但其性能略低于vector。
    // 因为vector的元素是连续存储的，而VecDeque的元素则不是完全连续的，这可能会导致随机访问时性能稍差
    requests: VecDeque<i32>,
}

impl RecentCounter {
    pub fn new() -> Self {
        Self { requests: VecDeque::new() }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        self.requests.push_back(t);
        /*while !self.requests.is_empty() && (t - self.requests.front().unwrap()) > 3000 {
            self.requests.pop_front();
        }*/
        // 题目要求:保证 每次对 ping 的调用都使用比之前更大的 t 值
        while let Some(&front) = self.requests.front() {
            if front < t - 3000 {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        self.requests.len() as i32
    }
}

impl Default for RecentCounter {
    fn default() -> Self {
        Self::new()
    }
}
//-----------------------------------------------------

/// 链表节点
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn set_next(&mut self, next: Option<Box<ListNode>>) {
        self.next = next;
    }

    /// 反转链表
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None; // 前一个节点
        let mut curr = head; // 当前节点
        while let Some(mut node) = curr {
            // Option<T>::take 方法用于获取并消耗 Option<T> 中的值，
            // Option 是 Some(T)，则返回 T 并将 Option 设置为 None; Option 已经是 None，则不执行任何操作并返回 None
            // let next_node = node.next.take(); // 取走当前节点的next指针的值并设设置为None，并保存原来的值在next_node中
            // node.next = prev;  // 将当前节点的next指针指向前一个节点
            // prev = Some(node); // 然后将当前节点放入前一个节点中
            // curr = next_node;  // 再将下一个节点放入当前节点
            // 作用同上，效率更高
            curr = node.next.take(); // 取走当前节点的next指针的值并设置为None，并保存原来的值在curr中
            node.next = prev;        // 然后将前一个节点放入下一个节点中
            prev = Some(node);       // 再将取出的当前节点的值放入前一个节点中
        }

        prev
    }

    /// 遍历链表
    pub fn print_list(&self) {
        let mut curr = Some(self);
        while let Some(node) = curr {
            print!("{} ", node.val);
            curr = node.next.as_ref().map(|x| &**x);
        }
        print!("\n");
    }
}
//-----------------------------------------------------

// Rc（引用计数）是一个用于管理堆上数据的所有权的智能指针。它提供了一种在多个所有者之间共享数据的方式，而无需将数据复制到每个所有者。
// Rust的所有权模型要求每个值都有一个明确的所有者，当所有者离开作用域时，该值将被自动释放。这有助于防止内存泄漏和其他常见的内存安全问题。
// 在某些情况下，希望多个变量共享对同一数据的所有权，而不是复制数据。这就是Rc发挥作用的地方。
// Rc通过在内部维护一个引用计数来实现共享所有权。每当一个新的Rc引用被创建时，引用计数就会增加。
// 当Rc引用离开作用域或被丢弃时，引用计数就会减少。当引用计数变为零时，Rc将自动释放其管理的数据。
// let five = Rc::new(5);
// let shared_five = Rc::clone(&five);  此时 five 和 shared_five 共享同一份数据

// RefCell 用于在运行时检查借用规则，并允许在某些情况下绕过Rust的静态借用检查。
// 即是在存在不可变引用的情况下，RefCell允许你获取可变引用，从而可以在不破坏Rust的安全性和所有权规则的前提下，
// 实现在不改变数据结构的情况下进行内部修改，即实现内部可变性（interior mutability）。
// RefCell通过提供borrow()和borrow_mut()方法，允许你借用值的不可变引用和可变引用。
// 这些方法在运行时执行借用规则的检查，并在违反规则时引发运行时错误。虽然RefCell提供了更大的灵活性，但如果不正确地使用，也会导致程序在运行时崩溃。
// 此外，RefCell 与 Cell 类似，都是用于实现内部可变性的类型，但两者在使用上有存在差异。
// Cell只能用于实现Copy trait的类型，因为Cell的值在赋值时可以进行复制。而RefCell则可以用于非Copy trait类型，因为它的值在赋值时不会进行复制

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
    // 其实Rust编译器有自己的内联策略，它会自动决定哪些函数应该被内联。这意味着，即使你不使用#[inline]属性，编译器也可能会自动内联一些函数。
    // 总结: #[inline]是一个优化提示，它告诉编译器可能希望某个函数被内联。最终是否内联这个函数，还是由编译器根据代码的具体情况来决定的。
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    /// 最大深度(深度优先搜索)
    // 二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 递归操作
        match root {
            None => 0,
            Some(node) => {
                // node.borrow().left.to_owned() 相比 node.borrow().left.clone() 效率更高
                // node.borrow().left.clone() 会调用Option和Rc<RefCell<T>>的clone方法，
                // node.borrow().left.to_owned()会调用Option的cloned方法，然后返回一个新的Option，
                // 其中的Rc<RefCell<TreeNode>>也是通过克隆得到的。
                // to_owned(): 这是一个Option<T>的方法，它用于将Option<T>中的值转换为其拥有的版本。
                // 如果Option是Some(value)，那么to_owned()会返回该值的克隆；如果Option是None，则to_owned()会返回None。
                // Rc<T>的clone方法实现的是引用计数加1的操作，这是一个快速的、O(1)复杂度的操作。
                // 因此，无论是clone()还是to_owned()，都需要对Rc<RefCell<TreeNode>>进行克隆，性能上非常接近。
                1 + std::cmp::max(
                    Self::max_depth(node.borrow().left.to_owned()),
                    Self::max_depth(node.borrow().right.to_owned()),
                )
            }
        }
    }

    /// 叶值序列相似的树
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 函数内部定义的函数称为闭包（Closure）或局部函数（Local Function）。
        // pre_order() 函数实际上是在 leaf_similar() 函数内部定义的局部函数。局部函数与闭包相似，但有区别：
        // 局部函数:是一个在另一个函数内部定义的命名函数。它们的行为类似于常规函数，并且可以访问其包含作用域中的变量。
        // 闭包:是一个匿名函数，可以捕获其环境中的变量。闭包通常用于实现高阶函数，它们可以像其他值一样传递。闭包通常使用 | 符号来定义，并可以捕获其外部的变量。
        fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
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
        }

        let (mut values1, mut values2) = (Vec::new(), Vec::new());
        pre_order(root1, &mut values1);
        pre_order(root2, &mut values2);
        // 这种局部函数的实现方式性能比下面的dfs()方法的实现方式高，其实是相差在比较函数的耗时
        // Self::dfs(root1, &mut values1);
        // Self::dfs(root2, &mut values2);

        values1 == values2
    }

    // DFS是深度优先搜索（Depth first search），是用递归进行搜索，尽可能深的搜索每一个节点。
    // 可以理解为不撞墙不回头，主要用于解决一些树的遍历和图的遍历问题。由于是通过递归实现，时间复杂度较高，一般用于数据较小的情况。
    /*fn dfs(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<Rc<RefCell<TreeNode>>>) {
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
    }*/
}
//-----------------------------------------------------