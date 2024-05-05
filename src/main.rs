use std::cmp;

fn main() {
    println!("------ 交替合并字符串 ------");
    let word1 = String::from("abcde");
    let word2 = String::from("xyz");
    let result = merge_alternately(word1, word2);
    println!("merge_alternately: {result}");

    println!("------ 字符串的最大公因子 ------");
    let str1 = String::from("ABABAB");
    let str2 = String::from("AB");
    let result = gcd_of_strings(str1, str2);
    println!("gcd_of_strings: {result}");

    println!("------ 拥有最多糖果的孩子 ------");
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let result = kids_with_candies(candies, extra_candies);
    println!("kids_with_candies: {:?}", result);

    println!("------ 种花问题 ------");
    // let flowerbed = vec![1, 0, 0, 0, 0, 1];
    let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
    // let flowerbed = vec![0, 1, 0];
    let n = 2;
    let result = can_place_flowers(flowerbed, n);
    println!("can_place_flowers {n}: {}", result);

    println!("------ 反转字符串中的元音字母 ------");
    let s = "leetcode".to_string();
    // let s = "hello".to_string();
    let result = reverse_vowels(s);
    println!("reverse_vowels: {}", result);
}

/// 交替合并字符串
fn merge_alternately(word1: String, word2: String) -> String {
    // let len1 = word.len();
    // let len2 = word.chars().count();
    // word.len(): 这个方法直接返回字符串中字节的数量。在Rust中，String是一个UTF-8编码的字符串，所以len()方法返回的是字节的数量。
    // 如果字符串只包含ASCII字符，那么字节和字符的数量是相同的。
    // 但是，如果字符串包含非ASCII字符（如中文字符或其他Unicode字符），一个字符可能由多个字节表示。因此，len()返回的可能不是字符的实际数量。
    // word.len()的执行效率非常高，因为它只需要读取字符串的内部长度字段，不需要遍历整个字符串。
    //
    // word.chars().count(): 这个方法首先将字符串转换为一个字符迭代器，然后计算迭代器的长度。
    // 即是它需要遍历整个字符串来计算字符的数量。因此，它的执行效率通常比len()低，特别是当字符串很长时。
    // 如果需要知道字符串中字符的实际数量，无论它们是否由多少个字节表示，那么word.chars().count()才是正确的方法，尽管它的执行效率相对较低。

    let len1 = word1.chars().count();
    let len2 = word2.chars().count();
    let mut result = String::with_capacity(len1 + len2);

    // 使用zip()将两个等长的Vec组合成一个Vec，其中元素是一个元组，包含原来两个Vec(向量)中对应位置的元素。
    for (ch1, ch2) in word1.chars().zip(word2.chars()) {
        result.push(ch1);
        result.push(ch2);
    }

    // .iter().skip(n):从迭代器中跳过前 n 个元素
    for ch in word1.chars().skip(len2) {
        result.push(ch);
    }

    for ch in word2.chars().skip(len1) {
        result.push(ch);
    }

    result
}
//------------------------------------------------

// 字符串的最大公因子
fn can_divide(s1: &str, s2: &str) -> bool {
    // .chunks_exact(s2.len()) 将 s1 的字节切片分割成长度为 s2.len() 的块。
    // 如果 s1 的长度不是 s2.len() 的整数倍，这个函数会抛出一个 panic。但由于s1.len() % s2.len() == 0，所以这里不会有问题。
    // .all(|chunk| chunk == s2.as_bytes()) 对所有分割出的块执行检查每个块是否都与 s2 的字节切片相等。
    // 如果所有块都相等，那么 s1 是由 s2 重复构成的，函数返回 true,否则返回 false。
    s1.len() % s2.len() == 0 && s1.as_bytes().chunks_exact(s2.len()).all(|chunk| chunk == s2.as_bytes())
}

/// 字符串的最大公因子
fn gcd_of_strings(str1: String, str2: String) -> String {
    let len1 = str1.len();
    let len2 = str2.len();

    // 求两个字符串长度的最大公约数
    let gec_len = (1..cmp::min(len1, len2) + 1).rev()
        .find(|&i| len1 % i == 0 && len2 % i == 0).unwrap_or_else(|| cmp::min(len1, len2));

    // let cd1 = can_divide(&str1, &str1[0..gec_len]);
    // let cd2 = can_divide(&str2, &str1[0..gec_len]);
    // println!("cd1: {cd1}, cd2: {cd2}");

    if can_divide(&str1, &str1[0..gec_len]) && can_divide(&str2, &str1[0..gec_len]) {
        return str1[0..gec_len].to_string();
    }

    "".to_string()
}
//---------------------------------------------------

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    // 通过遍历candies并比较每个孩子的糖果数量加上extra_candies之后是否大于或等于数组中的最大值。
    let max_candies = *candies.iter().max().unwrap_or(&0);

    // .iter(): 遍历向量中的每一个元素。迭代器是一个可以记住遍历的位置并在之后再次访问这些位置的对象。
    // .enumerate(): 这个方法附加在迭代器之后，它会改变迭代器产生的内容。
    // 原本迭代器只产生向量中的元素，但调用enumerate()后，迭代器现在产生的是元组(Tuple),
    // 每个元组包含两个元素：第一个是元素的索引(从0开始)，第二个是元素的值。
    /*for (i, &candy) in candies.iter().enumerate() {
        if candy + extra_candies >= *max_candies {
            result[i] = true;
        }
    }*/

    // .map(|&candy| candy + extra_candies >= max_candies)
    // 对迭代器中的每个元素(使用模式匹配|&candy|来借用每个candy的值，避免不必要的复制)应用一个函数。
    // 这个函数计算后会返回一个bool，true表示当前孩子的糖果加上额外的糖果后至少和最大的糖果数量一样多，false则表示不够。
    // .collect()方法调用，将map步骤返回的迭代器中的所有布尔值收集到一个新的(Vec<bool>)中
    candies.iter().map(|&candy| candy + extra_candies >= max_candies).collect()
}
//-------------------------------------------------------

// n: 是否可以种的花数量
fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let len = flowerbed.len();
    let mut n = n;
    let mut i = 0;

    while i < len {
        // 检查头尾&相邻项的问题
        if flowerbed[i] == 0 && (i == 0 || flowerbed[i - 1] == 0) && (i == len - 1 || flowerbed[i + 1] == 0) {
            n -= 1;
            /*if n == 0 {
                return true;
            }*/

            i += 2; // 下个位置肯定不能种花，可以跳过
        } else {
            i += 1;
        }
    }

    n <= 0
}
//-------------------------------------------------------

fn is_vowel(c: char) -> bool {
    matches!(c, 'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U')
}

fn reverse_vowels(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    // 双指针操作
    let mut i = 0;
    let mut j = chars.len() - 1;

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
//----------------------------------------------------