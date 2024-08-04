use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

/// 470.用 rand7函数 实现 rand10函数 (数学,拒绝采样,概率与统计,随机化)
// 给定方法 rand7 可生成 [1, 7] 范围内的均匀随机整数，试写一个方法 rand10 生成 [1, 10] 范围内的均匀随机整数。
// 你只能调用 rand7() 且不能调用其他方法。请不要使用系统的 Math.random() 方法。
// 每个测试用例将有一个内部参数 n，即你实现的函数 rand10() 在测试时将被调用的次数。注意:这不是传递给 rand10() 的参数。
// 输入: 3
// 输出: [3, 8, 10]
fn rand7() -> i32 {
    rand::thread_rng().gen_range(1..=7)
}

pub fn rand10() -> i32 {
    let mut x = 40;
    while x >= 40 {
        x = 7 * (rand7() - 1) + (rand7() - 1);
    }

    x % 10 + 1
}
//-----------------------------------------------------

/// 478.在圆内随机生成点(数学,拒绝采样,几何,随机化)
pub struct Solution {
    x: f64,
    y: f64,
    r: f64,
}

impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution {
            x: x_center,
            y: y_center,
            r: radius,
        }
    }

    pub fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-self.r..=self.r);
        loop {
            let (x, y): (f64, f64) = (die.sample(&mut rng), die.sample(&mut rng));
            if x * x + y * y <= self.r * self.r {
                return vec![x + self.x, y + self.y];
            }
        }
    }
}
//-----------------------------------------------------

/// 398.随机数索引(水塘抽样,数学,随机化,哈希表)
pub struct Solution2 {
    // nums: Vec<i32>,
    map: HashMap<i32, Vec<usize>>,
}

impl Solution2 {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            map.entry(n).or_default().push(i);
        }

        Self { map }
    }

    // 水塘抽样
    pub fn pick(&self, target: i32) -> i32 {
        /*self.nums.iter().enumerate().fold((0, -1), |(mut cnt, mut ans), (i, v)|
        if *v != target { (cnt, ans) } else {
            cnt += 1;
            if rand::thread_rng().gen_range(0..cnt) < 1 { ans = i as i32; }

            (cnt, ans)
        }).1*/

        if let Some(v) = self.map.get(&target) {
            let mut rng = rand::thread_rng();
            i32::try_from(v[rng.gen_range(0..v.len())]).expect("i32 error")
        } else {
            -1
        }
    }
}
//-----------------------------------------------------