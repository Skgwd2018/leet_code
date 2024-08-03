/// 链表节点
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn set_next(&mut self, next: Option<Box<ListNode>>) {
        self.next = next;
    }

    /// 206.反转链表
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None; // 前一个节点
        let mut curr = head; // 当前节点
        while let Some(mut node) = curr {
            // Option<T>::take 方法用于获取并消耗 Option<T> 中的值，
            // Option 是 Some(T),则返回 T 并将 Option 设置为 None; Option 已经是 None，则不执行任何操作并返回 None
            // let next_node = node.next.take(); // 取走当前节点的next指针的值并设设置为None，并保存原来的值在next_node中
            // node.next = prev;  // 将当前节点的next指针指向前一个节点
            // prev = Some(node); // 然后将当前节点放入前一个节点中
            // curr = next_node;  // 再将下一个节点放入当前节点
            // 作用同上，效率更高
            curr = node.next.take(); // 取走当前节点的next指针的值并设置为None,并保存原来的值在curr中
            node.next = prev;        // 然后将前一个节点放入下一个节点中
            prev = Some(node);       // 再将取出的当前节点的值放入前一个节点中
        }

        prev
    }

    /// 2095.删除链表的中间节点
    // 题目要求:链表中节点的数目范围[1, 105]
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        /*let mut prev = std::ptr::null_mut();
        let mut slow = &mut head as *mut Option<Box<ListNode>>;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            prev = slow;
            slow = unsafe {
                &mut slow.as_mut().unwrap().as_mut().unwrap().next as *mut Option<Box<ListNode>>
            };
        }
        unsafe {
            match prev.as_mut() {
                Some(prev) => prev.as_mut().unwrap().next = slow.as_mut().unwrap().as_mut().unwrap().next.take(),
                None => return None,
            }
        }
        head*/

        // 解法二:
        let (mut count, mut ptr) = (0, &head);
        // 遍历链表
        while ptr.is_some() {
            count += 1;
            ptr = &ptr.as_ref().unwrap().next;
        }

        let mut temp_head = Box::new(ListNode { val: 0, next: head });
        let mut i = 0;
        let mut curr = temp_head.as_mut();
        while let Some(node_next) = curr.next.take() {
            if i == count / 2 {
                curr.next = node_next.next;
                break;
            } else {
                curr.next = Some(node_next);
                curr = curr.next.as_mut().unwrap();
            }

            i += 1;
        }

        temp_head.next
    }

    /// 328.奇偶链表
    // 给定单链表的头节点 head,将所有索引为奇数的节点和索引为偶数的节点分别组合在一起,然后返回重新排序的列表。
    // 第一个节点的索引被认为是 奇数,第二个节点的索引为 偶数,以此类推。
    // 请注意，偶数组和奇数组内部的相对顺序应该与输入时保持一致。
    // 输入: head = [2, 1, 3, 5, 6, 4, 7]
    // 输出:        [2, 3, 6, 7, 1, 5, 4]
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        // let mut odd = Some(Box::new(ListNode::new(0)));
        let mut odd = None; // 内存消耗更低
        let mut curr_odd = &mut odd;
        let mut even = None;
        let mut curr_even = &mut even;
        let mut is_even = true;
        // 区分奇偶位,然后分别加入两个链表
        while let Some(mut curr) = head {
            head = curr.next.take();
            if is_even {
                // 通过as_mut()获取curr_even的可变引用,然后unwrap()这个Option,假设它一定是Some。
                // 接着设置当前curr_even指向的Option<Box<ListNode>>的next字段为Some(curr)。
                // 即修改even链表,将当前的节点curr添加到even链表的末尾。
                // curr_even.as_mut().unwrap().next = Some(curr);

                // 通过解引用curr_even来修改它指向的Option<Box<ListNode>>。将curr_even设置为Some(curr)，
                // 即让curr_even指向新的节点curr，而不是修改curr_even当前指向的节点的next字段。
                *curr_even = Some(curr);
                curr_even = &mut curr_even.as_mut().unwrap().next;
            } else {
                *curr_odd = Some(curr);
                curr_odd = &mut curr_odd.as_mut().unwrap().next;
            }

            is_even = !is_even;
        }

        // 偶数位链表的末尾指向奇数位链表的头
        // curr_even.as_mut().unwrap().next = odd.unwrap().next;
        // even.unwrap().next
        *curr_even = odd;
        even
    }

    /// 2130.链表最大孪生和
    // 在一个大小为 n 且 n 为 偶数 的链表中，对于 0 <= i <= (n / 2) - 1 的 i，第 i 个节点（下标从 0 开始）的孪生节点为第 (n-1-i) 个节点。
    // 比方说，n = 4 那么节点 0 是节点 3 的孪生节点，节点 1 是节点 2 的孪生节点。这是长度为 n = 4 的链表中所有的孪生节点。
    // 孪生和 定义为一个节点和它孪生节点两者值之和。
    // 给定一个长度为偶数的链表的头节点 head，返回链表的 最大孪生和。
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut list = vec![];
        let mut curr = head;
        let mut ans = 0;
        while let Some(node) = curr {
            list.push(node.val);
            curr = node.next;
        }
        let n = list.len();
        for i in 0..n / 2 {
            ans = ans.max(list[i] + list[n - 1 - i]);
        }

        ans
    }

    /// 遍历链表
    pub fn print_list(&self) {
        let mut curr = Some(self);
        while let Some(node) = curr {
            print!("{} ", node.val);
            // curr = node.next.as_ref().map(|x| &**x);
            // .as_ref() 用于获取值的引用, 返回 Option<&T> 类型，(得到的是对 Rc 内部数据的引用，而不是对 Rc 自身的引用),即是 &T
            // .as_ref():返回对 Box 内部数据的引用，对Option<Box<T>> 返回 Option<&T> 类型。(得到的是对 Box 内部数据的引用，而不是对 Box 自身的引用)
            // 如果 Option 是 Some(box)，则返回 Some(&*box)（即 box 的解引用引用）;如果 Option 是 None，则返回 None。返回 &T 类型
            curr = node.next.as_deref(); //作用同上
            // .as_deref() 用于获取原始类型的指针,返回 Option<Option<*const T>> 类型，(得到的是对 Rc 自身的引用，而不是对 Rc 内部数据的引用),即是 &Rc<T>
            // .as_deref():返回对 Box 自身的引用,是 Option<Box<T>> 的方法,它返回 Option<Option<*const T>>。(得到的是对 Box 自身的引用，而不是对 Box 内部数据的引用)
            // 如果 Option<T> 是 Some(T)，则返回 Some(ptr)，其中 ptr 是 T 的原始指针;如果 Option<T> 是 None,则返回 None。返回 &Box<T> 类型
        }
        println!();
    }
}
//-----------------------------------------------------