fn main() {
    println!("------ 交替合并字符串 ------");
    let word1 = String::from("abcde");
    let word2 = String::from("xyz");
    let result = merge_alternately(word1, word2);
    println!("merge_alternately: {result}");
}

/// 交替合并字符串
fn merge_alternately(word1: String, word2: String) -> String {
    let len1 = word1.chars().count();
    let len2 = word2.chars().count();
    let mut result = String::with_capacity(len1 + len2);

    for (ch1, ch2) in word1.chars().zip(word2.chars()) {
        result.push(ch1);
        result.push(ch2);
    }

    for ch in word1.chars().skip(len2) {
        result.push(ch);
    }

    for ch in word2.chars().skip(len1) {
        result.push(ch);
    }

    result
}
