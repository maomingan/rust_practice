use std::collections::HashMap;

fn main() {
    // 创建一个hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // 获取hashmap的值
    let team_name = String::from("Blue");
    // 获取直接的数值而非some值，如果key不存在，则返回0
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {:?}", score);

    // 遍历hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 给定一个空格分割的字符串，统计每个单词出现的次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    
}