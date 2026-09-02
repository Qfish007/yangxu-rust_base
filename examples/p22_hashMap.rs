use std::collections::HashMap;

fn main() {
    println!("Example p22: HashMap");

    let mut map = HashMap::new();
    map.insert(String::from("hello"), 10);
    map.insert(String::from("world"), 20);
    println!("{:?}", map);

    let vec = vec![
        (String::from("key1"), "value1"),
        (String::from("key2"), "value2"),
    ];
    let map: HashMap<_, _> = vec.into_iter().collect();
    println!("{:?}", map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // scores.get("Blue")        // Option<&i32>
    //       .copied()           // Option<i32>
    //       .unwrap_or(0)       // i32
    let value = scores.get("Blue").copied().unwrap_or(0);
    println!("scores value for Blue: {:?}", value);

    // entry 判断是否已经存在，不存在则插入，存在则返回引用值
    scores.entry(String::from("Blue")).or_insert(0);
    scores.entry(String::from("black")).or_insert(20);

    println!("{:?}", scores);

    // 计算单词出现次数
    println!("Example p22: HashMap - 计算单词出现次数");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // 写入引用值
    }
    println!("{:?}", map);
}
