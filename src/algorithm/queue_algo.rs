use std::collections::VecDeque;

/// 933.最近的请求次数(头尾高效操作的队列,数据流)
/// 计算特定时间范围内最近的请求
#[derive(Default)]
pub struct RecentCounter {
    // VecDeque 与 Vec 的区别
    // 1.高效的两端操作:允许在队列的前端和后端进行高效的插入和删除操作。
    //   即可以使用栈一样在队列的任一端进行 push 和 pop 操作,这种特性使其在处理需要频繁在两端进行操作的场景时非常高效。
    // 2.支持 FIFO 和 FILO 操作
    // 3.支持随机访问:虽然VecDeque支持随机访问,但其性能略低于vector。
    //   因为vector的元素是连续存储的,而VecDeque的元素则不是完全连续的,这可能会导致随机访问时性能稍差
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
            if front >= t - 3000 { break; }
            self.requests.pop_front();
        }

        i32::try_from(self.requests.len()).unwrap_or_default()
    }
}
//-----------------------------------------------------

/// 649.Dota2 参议院(贪心,队列,字符串)
pub fn predict_party_victory(senate: String) -> String {
    /*let mut radiant = VecDeque::new();
    let mut dire = VecDeque::new();
    for (i, ch) in senate.char_indices() {
        match ch {
            'R' => radiant.push_back(i),
            _ => dire.push_back(i),
        }
    }
    let n = senate.len();
    loop {
        match (radiant.pop_front(), dire.pop_front()) {
            (Some(x), Some(y)) => match x < y {
                true => radiant.push_back(x + n),
                false => dire.push_back(y + n),
            },
            (Some(_), None) => return "Radiant".to_string(),
            _ => return "Dire".to_string(),
        }
    }*/

    // 解法二:上面的优化版
    let mut rd = [VecDeque::new(), VecDeque::new()];
    for (i, c) in senate.chars().enumerate() {
        rd[usize::from(c == 'D')].push_back(i);
    }
    let n = senate.len();
    loop {
        match (rd[0].pop_front(), rd[1].pop_front()) {
            (Some(r), Some(d)) => rd[usize::from(r > d)].push_back(if r > d { r } else { d } + n),
            (Some(_r), None) => break "Radiant".to_string(),
            (None, Some(_d)) => break "Dire".to_string(),
            _ => ()
        }
    }
}
//-----------------------------------------------------