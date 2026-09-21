use std::pin::pin;
use std::time::Duration;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    println!("Example p50: Streams");
    // test01();
    // test02();
    // test03();
    test04();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

    trpl::run(async {
        let values: Vec<_> = vec![1, 2, 3, 4, 5];
        let iter = values.iter().map(|v| v * 2);
        // 将迭代器转换为可迭代的异步流
        let mut stream = trpl::stream_from_iter(iter);
        while let Some(val) = stream.next().await {
            println!("received: {}", val);
        }
    });

    /*
    stream:Iter<Map<Iter<'_, i32>, impl FnMut(&i32) -> i32>>
    stream.next() 其实就是调用  stream.inner.next(),
    而 stream.inner 指向了  Map<Iter<'_, i32>, impl FnMut(&i32) -> i32>  假设它就叫map1,也是个迭代器
    会继续执行 map1.inner.next(), 继续指向  Iter<'_, i32> 假设 是 iter1,也是迭代器
    继续执行 iter1.inner.next(), 最终指向了 i32
    ---------------------------
    在换一种说法
    stream.inner = Map
    stream.inner.item = Iter
    stream.inner.inner.inner = i32
         */
}

// cbtmark
fn get_message2() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f"];

    // 发送消息到通道，这里是一次性发送所有消息，不会阻塞
    for msg in messages {
        tx.send(msg.to_string()).unwrap();
    }

    // 将rx 通过 ReceiverStream 包装 异步接受流，将其转换为 ReceiverStream<String>
    ReceiverStream::new(rx)
}
#[allow(unused)]
fn test02() {
    println!("--------------test02------------");

    trpl::run(async {
        // 这里将 get_message2() 获得一个异步接受流，但是其内部是一次性发送所有消息
        let mut message = get_message2();

        while let Some(val) = message.next().await {
            println!("received: {}", val);
        }
    });
}

// cbtmark
fn get_message3() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    // 启动一个异步任务流，模拟发送消息
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f"];

        for (index, msg) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };

            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            // 每隔100ms或者300ms发送一个消息，模拟异步操作
            if let Err(send_error) = tx.send(format!("cbt msg: {}", msg)) {
                println!("send error: {:?}", send_error);
                break;
            }
        }
    });

    // 通过 ReceiverStream 包装 rx，将其转换为 ReceiverStream<String>
    ReceiverStream::new(rx)
}

#[allow(unused)]
fn test03() {
    println!("--------------test02------------");

    trpl::run(async {
        // get_message3() 获得一个异步接受流，其内部是独立的异步任务
        // 该流被 timeout 包裹，设置超时时间为 200ms
        let mut messages = pin!(get_message3().timeout(Duration::from_millis(200)));

        /*
                你调用 messages.next().await
                │
                ▼
        ┌─────────────────────────────────────┐
        │  timeout 流被 poll                   │
        │  它同时发起两个异步操作：              │
        │                                     │
        │  ① poll 内层 get_message3 的 rx      │
        │     （问：通道里有新消息吗？）         │
        │                                     │
        │  ② 启动一个 200ms 的 Sleep 定时器     │
        │                                     │
        │  两者用 select 竞争：谁先 ready 用谁  │
        └─────────────────────────────────────┘
                │
                ├── ① 先就绪 → 返回 Ok(String)  ← 消息按时到
                │
                └── ② 先就绪 → 返回 Err(Elapsed) ← 200ms 到了，消息没来
        */
        while let Some(result) = messages.next().await {
            match result {
                Ok(val) => println!("received: {}", val),
                Err(_) => println!("timeout"),
            }
        }
    });
}

// cbtmark
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // 启动一个异步任务，模拟发送消息
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await; // 每隔1ms发送一个消息
            count += 1;
            // println!("count: {}", count);
            if let Err(send_error) = tx.send(count) {
                println!("send error: {:?}", send_error);
                break;
            }
        }
    });

    // 通过 ReceiverStream 包装 rx，将其转换为 ReceiverStream<String>
    ReceiverStream::new(rx)
}

#[allow(unused)]
fn test04() {
    println!("--------------test04--------流的合并----");

    trpl::run(async {
        // messages 是一个异步流，这个流被 timeout 操作符包装，设置超时时间为 200ms
        // 单输入，单输出，中间加了个时间闸门
        let messages = get_message3().timeout(Duration::from_millis(200));

        // intervals 是一个异步流，这个流被 map throttle  timeout 操作符包装，设置节流时间为 100ms
        // 单输入，单输出，中间加了个节流门
        // map 操作符将 u32 转换为 String 类型
        // throttle 操作符设置节流时间为 100ms
        // timeout 操作符设置超时时间为 10s
        // 单输入，单输出，中间加了个超时门
        let intervals = get_intervals()
            .map(|count| format!("interval: {}", count))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        // 将 messages 和 intervals 合并为一个流 merged 是一个合并的异步流，这个流被 Take 操作符包装，
        // 两个输入流，单输出，
        // take 操作符取前 20 个元素
        let merged = messages.merge(intervals).take(20);

        // timeout 包出来的新流不保证 Unpin（可安全移动），而调用 .next().await 需要流是固定的，
        // 所以用 pin! 把它固定在栈上。可以理解成：流被包装过之后，用之前 pin 一下。
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(val) => println!("received: {}", val),
                Err(reason) => println!("timeout: {:?}", reason),
            }
        }
    });
}
