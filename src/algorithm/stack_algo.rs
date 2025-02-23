/// 2390.从字符串中移除星号(栈,字符串)
// 给你一个包含若干星号 * 的字符串 s
// 在一步操作中,可以:选中 s 中的一个星号。
// 移除星号 左侧 最近的那个 非星号 字符,并移除该星号自身。返回移除 所有 星号之后的字符串。
pub fn remove_stars(s: &str) -> String {
    // 栈操作
    let mut stack = Vec::new();
    /*for c in s.chars() {
        match c {
            '*' => if stack.pop().is_some() {},
            _ => stack.push(c),
        }
    }
    stack.iter().collect::<String>()*/

    // 解法二:
    // 操作byte效率更高,且简单场合中if else 的运行效率比 match 高
    for &b in s.as_bytes() {
        if b == b'*' {
            stack.pop().unwrap();
        } else {
            stack.push(b);
        }
    }

    String::from_utf8(stack).unwrap()
}
//-----------------------------------------------------

/// 735.小行星碰撞(数组,栈,模拟)
// 整数数组 asteroids,表示在同一行的小行星。
// 对于数组中的每一个元素,其绝对值表示小行星的大小,正负表示小行星的移动方向(正表示向右移动,负表示向左移动)。每一颗小行星以相同的速度移动。
// 找出碰撞后剩下的所有小行星。
// 碰撞规则:两个小行星相互碰撞,较小的小行星会爆炸。如果两颗小行星大小相同,则两颗小行星都会爆炸。两颗移动方向相同的小行星,永远不会发生碰撞。
// asteroids = [10, 2, -5]
// 输出: [10]
// 解释: 2 和 -5 发生碰撞后剩下 -5 ; 10 和 -5 发生碰撞后剩下 10
pub fn asteroid_collision(mut asteroids: Vec<i32>) -> Vec<i32> {
    // 数组原地模拟,使用index表示当前最右(也就是最先接受碰撞的节点索引)行星,只需要寻找可能发生碰撞的行星,即运动方向不同的,
    // 遍历数组:
    // 1.如果当前行星向右(尝试找下一个向右的行星),或者index位于初始位置,或者当前行星和最右行星都向左,将index + 1,并将行星位置前移;
    // 2.否则,判断最右行星与当前行星的负值大小(必须向左),如果：
    //   二者相等,则双双抵消,最右行星索引移动到前一个数组位置即可;
    //   最右行星的正数值大,相当于碰撞无效,不需要替换行星,不做处理即可;
    //   当前行星的负值较大,说明碰撞后留下的是当前行星,将当前索引和最右行星同时回拨;
    // 最后返回原数组的[0, (index + 1).max(0)]范围内的元素。
    let (mut index, mut i): (i32, usize) = (-1, 0);
    while i < asteroids.len() {
        if asteroids[i] > 0 || index == -1 || asteroids[index as usize] < 0 {
            index += 1;
            asteroids[index as usize] = asteroids[i];
        } else if asteroids[index as usize] <= -asteroids[i] {
            if asteroids[index as usize] < -asteroids[i] {
                i -= 1;
            }
            index -= 1;
        }

        i += 1;
    }

    asteroids[0..(index + 1).max(0) as usize].to_vec()
}
//-----------------------------------------------------

/// 394.字符串解码(栈,字符串,递归)
// 题目要求:原始数据不包含数字,所有的数字只表示重复的次数 k,例:不会出现像 3a 或 2[4] 的输入
//         s 中所有整数的取值范围为 [1, 300]
pub fn decode_string(s: &str) -> String {
    // 栈操作
    let mut stack = Vec::new();
    let mut curr_num = 0;
    let mut curr_str = String::new();
    // let (mut curr_num, mut curr_str) = (0, String::new()); // 内存比较(完全相同)

    // "3[a12[c]]" ----> "accccccccccccaccccccccccccacccccccccccc"
    for c in s.chars() {
        match c {
            '0'..='9' => {
                // curr_num = curr_num * 10 + (c as u8 - '0' as u8) as usize; // '0' as u8 是 48
                curr_num = curr_num * 10 + (c as usize - '0' as usize);
            }
            '[' => {
                stack.push((curr_str, curr_num));
                curr_str = String::new();
                // 这个操作创建了一个新的空字符串,并将curr_str的引用更新为指向这个新字符串。
                // 这涉及到内存分配(尽管分配的是一个非常小的字符串),并且如果之前的String是堆上分配的,那么它的内存也会被回收。
                // 这种方式的优点是它确保了curr_str不再保留任何不必要的内存,但缺点是涉及到内存分配和可能的垃圾回收,这通常比简单的标记为空要慢一些。

                // curr_str.clear();
                // 这个操作仅仅将字符串的内部缓冲区标记为空,它并不会释放分配的内存。
                // 即如果字符串之前占用了大量内存,那么即使调用clear()之后,该内存仍然被String保留。
                // 这样做的优点是操作很快,因为不涉及任何内存分配或释放。
                // 如果之后String又需要存储数据,它可以在已经分配的内存上进行操作,这通常比重新分配内存要快。
                curr_num = 0;
            }
            ']' => {
                if let Some((prev_str, count)) = stack.pop() {
                    let repeated_str = curr_str.repeat(count); // 重复curr_str字符串count次
                    curr_str = prev_str + &repeated_str;
                }
            }
            _ => curr_str.push(c),
        }
    }

    curr_str
}
//-----------------------------------------------------