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
