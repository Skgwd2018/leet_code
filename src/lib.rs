use std::collections::VecDeque;

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
