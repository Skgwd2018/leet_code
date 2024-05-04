fn main() {
    println!("------ 交替合并字符串 ------");
    let word1 = String::from("abcde");
    let word2 = String::from("xyz");
    let result = merge_alternately(word1, word2);
    println!("merge_alternately: {result}");
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
