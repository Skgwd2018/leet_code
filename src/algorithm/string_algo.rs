use std::cmp;

/// 28.找出字符串中第一个匹配项的下标(字符串匹配)
// 给定两个字符串 haystack 和 needle,请在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标(下标从 0 开始)。
// 如果 needle 不是 haystack 的一部分,则返回 -1
pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map_or(-1, |n| i32::try_from(n).expect("i32 error"))
}
//-----------------------------------------------------

/// 1768.交替合并字符串(字符串,双指针)
pub fn merge_alternately(word1: String, word2: String) -> String {
    // let len1 = word.len();
    // let len2 = word.chars().count();
    // word.len():这个方法直接返回字符串中字节的数量。在Rust中,String是一个UTF-8编码的字符串,所以len()方法返回的是字节的数量。
    // 如果字符串只包含ASCII字符,那么字节和字符的数量是相同的。
    // 但是,如果字符串包含非ASCII字符(如中文字符或其他Unicode字符),一个字符可能由多个字节表示。因此,len()返回的可能不是字符的实际数量。
    // word.len()的执行效率非常高,因为它只需要读取字符串的内部长度字段,不需要遍历整个字符串。
    //
    // word.chars().count():这个方法首先将字符串转换为一个字符迭代器,然后计算迭代器的长度。
    // 即是它需要遍历整个字符串来计算字符的数量。因此,它的执行效率通常比 len() 低,特别是当字符串很长时。
    // 当需要知道字符串中字符的实际数量,无论它们是否由多少个字节表示,则 word.chars().count() 才是正确的做法。

    let len1 = word1.chars().count();
    let len2 = word2.chars().count();
    let mut answer = String::with_capacity(len1 + len2);
    // 使用zip()将两个等长的Vec组合成一个Vec,其中元素是一个元组,包含原来两个Vec中对应位置的元素。
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        answer.push(c1);
        answer.push(c2);
    }

    // .iter().skip(n):从迭代器中跳过前 n 个元素
    for c in word1.chars().skip(len2) {
        answer.push(c);
    }
    for c in word2.chars().skip(len1) {
        answer.push(c);
    }

    answer
}
//-----------------------------------------------------

/// 1071.字符串的最大公因子(字符串,数学)
// 题目要求:字符串中的字符全是字母
pub fn _gcd_of_strings(str1: String, str2: String) -> String {
    fn can_divide(s1: &str, s2: &str) -> bool {
        // .chunks_exact(s2.len()) 将 s1 的字节切片分割成长度为 s2.len() 的块。
        // 如果 s1 的长度不是 s2.len() 的整数倍,这个函数会抛出一个 panic。但由于s1.len() % s2.len() == 0,所以这里不会有问题。
        // .all(|chunk| chunk == s2.as_bytes()) 对所有分割出的块执行检查每个块是否都与 s2 的字节切片相等。
        // 如果所有块都相等,那么 s1 是由 s2 重复构成的,函数返回 true,否则返回 false。
        s1.len() % s2.len() == 0 && s1.as_bytes().chunks_exact(s2.len()).all(|chunk| chunk == s2.as_bytes())
    }
    // let cd1 = can_divide(&str1, &str1[0..gec_len]);
    // let cd2 = can_divide(&str2, &str1[0..gec_len]);
    // println!("cd1: {cd1}, cd2: {cd2}");

    let (len1, len2) = (str1.len(), str2.len());
    // 求两个字符串长度的最大公约数
    // .find() 用于查找单个元素,返回满足条件的第一个元素(如果存在)。返回类型为 Option<T>。
    // .filter() 返回一个新迭代器,包含所有满足条件的元素。返回类型为实现了 Iterator<Item=T> 的类型。
    let gec_len = (1..=cmp::min(len1, len2)).rev()
        .find(|&i| len1 % i == 0 && len2 % i == 0).unwrap_or_else(|| cmp::min(len1, len2));

    if can_divide(&str1, &str1[0..gec_len]) && can_divide(&str2, &str1[0..gec_len]) {
        return str1[0..gec_len].to_string();
    }

    // 创建空字符串推荐使用 String::new() 的方式
    // "".to_string()
    String::new()
}

/// 字符串的最大公因子
/// 解法二:使用欧几里得算法
// 欧几里得算法即辗转相除法,指用于计算两个非负整数 a, b 的最大公约数。计算公式gcd(a, b) = gcd(b, a mod b)
// 两个整数的最大公约数等于其中较小的数和两数相除余数的最大公约数
// 如果两个字符串交替相加后,值仍然相等,即str1 + str2 == str2 + str1时,就可以认为存在公因子字符串。
// 当一定存在公因子时,最大公因子字符的长度一定就是两个字符串长度的最大公因数。
// 公因子字符串也就是str1或str2的前缀下标。范围为:[0, 最大公因数]
pub fn gcd_of_strings2(str1: String, str2: String) -> String {
    fn get_gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            get_gcd(b, a % b)
        }
    }

    // let s1 = str1.clone() + &str2;  // 消耗内存稍高,运行稍快
    // let s2 = str2.clone() + &str1;
    let s1 = format!("{str1}{str2}"); // 消耗内存稍低,但是运行稍慢
    let s2 = format!("{str2}{str1}");
    if s1 != s2 {
        return String::new();
    }

    str1[0..get_gcd(str1.len(), str2.len())].to_string()

    // 解法二:
    /*if str1.len() < str2.len() {
        return gcd_of_strings2(str2, str1);
    }
    if str2.is_empty() {
        return str1;
    }

    return if str1.starts_with(&str2) {
        gcd_of_strings2(str1[str2.len()..].to_string(), str2)
    } else {
        "".to_string()
    };*/
}
//-----------------------------------------------------

/// 是否是元音字母
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

/// 345.反转字符串中的元音字母(字符串,双指针)
pub fn reverse_vowels(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    // 双指针操作
    let (mut i, mut j) = (0, chars.len() - 1);
    while i < j {
        if !is_vowel(chars[i]) {
            i += 1;
            continue;
        }
        if !is_vowel(chars[j]) {
            j -= 1;
            continue;
        }

        // 交换位置
        chars.swap(i, j);
        i += 1;
        j -= 1;
    }

    chars.iter().collect()
}

// 解法二:使用bytes操作
/*fn reverse_vowels2(mut s: String) -> String {
    let vowels = b"aAeEiIoOuU".into_iter().collect::<std::collections::BTreeSet<_>>();
    let bytes = unsafe { s.as_bytes_mut() };
    let (mut i, mut j) = (0, bytes.len() - 1);
    while i < j {
        if !vowels.contains(&bytes[i]) {
            i += 1;
        } else if !vowels.contains(&bytes[j]) {
            j -= 1;
        } else {
            bytes.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    s
}*/
//-----------------------------------------------------

/// 151.反转字符串中的单词(字符串,双指针)
// 给你一个字符串 s,请你反转字符串中 单词 的顺序。
// 单词 是由非空格字符组成的字符串。s 中使用至少一个空格将字符串中的 单词 分隔开。
// 返回 单词 顺序颠倒且 单词 之间用单个空格连接的结果字符串。
// 题目要求:输入的字符串s中可能会存在前导空格、尾随空格或者单词间的多个空格。返回的结果字符串中,单词间应当仅用单个空格分隔,且不包含任何额外的空格。
pub fn reverse_words(s: String) -> String {
    /*let mut words: VecDeque<String> = VecDeque::new();
    let mut curr_word = String::new();
    for c in s.trim().chars() {
        if c.is_whitespace() {
            if !curr_word.is_empty() {
                words.push_front(curr_word);
                curr_word = String::new();
            }
        } else {
            curr_word.push(c);
        }
    }
    if !curr_word.is_empty() { words.push_front(curr_word); }
    words.into_iter().collect::<Vec<_>>().join(" ")*/

    // 解法二:
    // .split_ascii_whitespace() 将字符串s按照ASCII空白字符(如空格、制表符、换行符等)进行分割,返回一个迭代器,其中每个元素都是原始字符串中的一个单词
    // .rev() 将迭代器中所有元素的顺序反转
    // ::<T> 是Rust中用于指定泛型参数或返回类型的语法,也被称为类型提示。此处用于告诉 collect() 方法要收集元素到一个 Vec<&str> 类型的vec中
    // .collect::<Vec<&str>>() 将反转后的迭代器元素收集到一个新的vec中并指定了它的返回类型。且每个元素都是一个指向原始字符串中单词的切片(&str)
    // .join(" ") 将vec中的所有切片用空格连接起来,形成一个新的字符串
    s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
//-----------------------------------------------------

/// 443.压缩字符串(字符串,双指针)
// 题目要求:chars不为空
pub fn compress(chars: &mut [char]) -> i32 {
    let n = chars.len();
    if n <= 1 { return i32::try_from(n).expect("i32 error"); }

    let (mut idx, mut count) = (0, 1);
    for i in 1..n {
        if chars[i - 1] == chars[i] {
            count += 1;
        } else {
            chars[idx] = chars[i - 1];
            idx += 1;
            if count > 1 {
                for c in count.to_string().chars() {
                    chars[idx] = c;
                    idx += 1;
                }
            }
            count = 1;
        }
    }

    chars[idx] = chars[n - 1];
    idx += 1;
    if count > 1 {
        for c in count.to_string().chars() {
            chars[idx] = c;
            idx += 1;
        }
    }

    // idx as i32
    i32::try_from(idx).unwrap_or_default()
}
//-----------------------------------------------------

/// 3042.统计前后缀下标对 I(字典树,字符串匹配,哈希函数,滚动哈希)
// 给定一个下标从 0 开始的字符串数组 words。
// 定义一个 布尔 函数 isPrefixAndSuffix,它接受两个字符串参数 str1 和 str2:
// 当 str1 同时是 str2 的前缀(prefix)和后缀(suffix)时,isPrefixAndSuffix(str1, str2) 返回 true,否则返回 false。
// 例如,isPrefixAndSuffix("aba", "ababa") 返回 true,因为 "aba" 既是 "ababa" 的前缀,也是 "ababa" 的后缀,但是 isPrefixAndSuffix("abc", "abcd") 返回 false。
// 以整数形式,返回满足 i < j 且 isPrefixAndSuffix(words[i], words[j]) 为 true 的下标对 (i, j) 的 数量
// 输入: words = ["a","aba","ababa","aa"]
// 输出: 4
// 解释: 在本示例中,计数的下标对包括:
// i = 0 且 j = 1 ,因为 isPrefixAndSuffix("a", "aba") 为 true
// i = 0 且 j = 2 ,因为 isPrefixAndSuffix("a", "ababa") 为 true
// i = 0 且 j = 3 ,因为 isPrefixAndSuffix("a", "aa") 为 true
// i = 1 且 j = 2 ,因为 isPrefixAndSuffix("aba", "ababa") 为 true
// 因此,答案是 4
pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    /*fn is_prefix_and_suffix(str1: &str, str2: &str) -> bool {
        if str1.len() > str2.len() { return false; }

        let prefix = &str2[..str1.len()];
        let suffix = &str2[(str2.len() - str1.len())..];

        prefix == str1 && suffix == str1
    }*/

    let n = words.len();
    let mut cnt = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                cnt += 1;
            }
        }
    }

    cnt
}
//-----------------------------------------------------