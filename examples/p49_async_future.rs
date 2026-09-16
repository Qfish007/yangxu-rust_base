use std::future::Future;
use std::pin::{pin, Pin};
use std::thread;
use std::time::{Duration, Instant};
use trpl::{self, Either};

/*
1. Pin<Box<dyn Future<Output = ()>>>, 在堆上分配内存，使用动态调度
2. Pin<&mut dyn Future<Output = ()>>, 在栈上分配内存，需要pinned引用

如果要使用 Pin<&mut dyn Future<Output = ()>>, 必须要满足以下条件之一
1. 他们是 Unpin，运行他们被安全的移动
2. 他们是被pin!宏显式固定的，以确保他们的内存位置不会发生变化

这里的Unpin 理解起来有点绕，但是只要记住它表示可以安全的移动
【Unpin 字面意思是没有固定,即不需要固定，因为它可以安全的被移动】
*/
fn main() {
    println!("Example p49_async_future: Async Future");
    test05();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01---------介绍join_all的用法---");

    trpl::run(async {
        // 这里是新产生了一个异步任务，
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();
        // cbtmark   这里的move是为了将tx传递给tx_fut，否则tx不会被释放,程序不会结束
        let tx_fut = async move {
            let vals = vec![
                String::from("hello"),
                String::from("world"),
                String::from("rust"),
                String::from("channel"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut2 = async move {
            let vals = vec![
                String::from("我"),
                String::from("是"),
                String::from("陈"),
                String::from("宝"),
                String::from("涛"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("received: {}", val);
            }
        };
        // trpl::join!(tx_fut, tx_fut2, rx_fut);
        let list: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx_fut), Box::pin(tx_fut2), Box::pin(rx_fut)];
        trpl::join_all(list).await;
    })
}

#[allow(unused)]
fn test02() {
    println!("--------------test02------------");

    trpl::run(async {
        // 这里是新产生了一个异步任务，
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();
        // cbtmark   这里的move是为了将tx传递给tx_fut，否则tx不会被释放,程序不会结束
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("hello"),
                String::from("world"),
                String::from("rust"),
                String::from("channel"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let tx_fut2 = pin!(async move {
            let vals = vec![
                String::from("我"),
                String::from("是"),
                String::from("陈"),
                String::from("宝"),
                String::from("涛"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(val) = rx.recv().await {
                println!("received: {}", val);
            }
        });
        // trpl::join!(tx_fut, tx_fut2, rx_fut);
        let list: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, tx_fut2, rx_fut];
        trpl::join_all(list).await;
    })
}

#[allow(unused)]
fn test03() {
    println!("--------------test03-----race 竞争-------");

    trpl::run(async {
        let slow = async {
            println!("slow start");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("slow end");
        };

        let fast = async {
            println!("fast start");
            trpl::sleep(Duration::from_millis(10)).await;
            println!("fast end");
        };

        trpl::race(slow, fast);
        println!("hello world");
    })
}

#[allow(unused)]
fn test04() {
    println!("--------------test04------------");

    trpl::run(async {
        let a = async {
            println!("a start");
            slow("a", 30);
            slow("a", 50); // step 1
            trpl::sleep(Duration::from_millis(100)).await; // step 3

            // trpl::yield_now().await; // 马上交出控制权，让其他任务执行

            println!("a end"); // step 4 结束了
        };

        let b = async {
            println!("b start");
            slow("b", 100);
            slow("b", 500); // step 2
            trpl::sleep(Duration::from_millis(10)).await; // step 5 不会执行
            println!("b end"); // step 6 不会执行
        };
        /*
           理论上a要先完成，
           但是在进入异步前，b执行了很多阻塞操作，

        */
        trpl::race(a, b);
        println!("hello world");
    });

    fn slow(name: &str, ms: u64) {
        // trpl::sleep(Duration::from_millis(10)).await; // 这个模拟异步操作不会阻塞主线程
        thread::sleep(Duration::from_millis(ms)); // 这个模拟阻塞操作
        println!("{} end, cost {} ms", name, ms);
    }
}

#[allow(unused)]
fn test05() {
    println!("--------------test05------yield_now------");

    trpl::run(async {
        let one_ns = Duration::from_nanos(1);

        // ======================测试 sleep 时间
        // let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        // println!("time: {:?}", time);
        println!("sleeptime cost: {:?}", time.as_secs_f64());

        // ======================测试 yield_now 时间
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        // println!("time: {:?}", time);
        println!("yield_now time cost: {:?}", time.as_secs_f64());
    })
}

#[allow(unused)]
fn test06() {
    println!("--------------test06---------模拟一个延时操作---");

    trpl::run(async {
        let fut = async {
            trpl::sleep(Duration::from_secs(2)).await;
            println!("fut finished");
            "fut finished".to_string()
        };

        // 比较fut是否在2秒内完成，否则超时
        match timeout(fut, Duration::from_secs(2)).await {
            Ok(msg) => println!("future success finished: {:?}", msg),
            Err(duration) => println!("future fail, timeout cost {:?}", duration.as_secs()),
        }
    });

    async fn timeout<F: Future>(fut: F, max_time: Duration) -> Result<F::Output, Duration> {
        match trpl::race(fut, trpl::sleep(max_time)).await {
            Either::Left(left) => Ok(left),
            Either::Right(_) => Err(max_time),
        }
    }
}
