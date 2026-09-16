use std::time::Duration;
use trpl::{self};

fn main() {
    println!("Example p48_async: Async/Await 实现并发");
    // 用异步实现并发任务
    // test01();
    // test02();
    // test03();
    // test04();
    test05();
}

/* ***********************异步里面是顺序执行，没有实现并发 *********************** */
#[allow(unused)]
fn test01() {
    println!("--------------test01------------");
    trpl::run(async {
        // 这里是新产生了一个异步任务，没有马上执行
        let fut1 = async {
            println!("hello world1");
            for i in 0..10 {
                println!("hello world1 {}  first_task", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // 下面这段在for执行完了才执行fut1
        for i in 0..5 {
            println!("hello world2 {}  second_task", i);
            trpl::sleep(Duration::from_millis(500)).await;
        }
        fut1.await;
        // fut2.await;
        println!("hello world2");
    })
}

/* ***********************异步里面是并发执行，实现并行 *********************** */
#[allow(unused)]
fn test02() {
    println!("--------------test02------------");

    trpl::run(async {
        // 这里是新产生了一个异步任务，马上执行
        // spawn_task 是把一个异步任务"提交给运行时调度执行"，让它和当前任务并发跑，而不是原地立刻执行完。
        let handle = trpl::spawn_task(async {
            println!("hello world1");
            for i in 0..10 {
                println!("hello world1 {}  first_task", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        // 下面这段代表本身就是在异步里面执行的，所以优先级比较高，当他执行完就结束了，所以后面要加 handle.await.unwrap();
        for i in 0..5 {
            println!("hello world2 {}  second_task", i);
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // handle.await.unwrap();
        println!("hello world2");
    })
}

/* ***********************异步里面是并发执行，实现并行 *********************** */
#[allow(unused)]
fn test03() {
    println!("--------------test03------------");

    trpl::run(async {
        let fut1 = async {
            println!("hello world1");
            for i in 0..10 {
                println!("hello world1 {}  first_task", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 0..5 {
                println!("hello world2 {}  second_task", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(fut1, fut2).await; // join 会产生新的future
        println!("hello world2");
    })
}

#[allow(unused)]
fn test04() {
    println!("--------------test04------------ 这是一个非我们想要的例子");

    trpl::run(async {
        // 这里是新产生了一个异步任务，
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("rust"),
            String::from("channel"),
        ];

        // 这里会阻塞，等了4*500ms，
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // 延时了4*500ms，才执行下面的代码
        while let Some(val) = rx.recv().await {
            println!("received: {}", val);
        }
    })
}

#[allow(unused)]
fn test05() {
    println!("--------------test05------------ ");

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
        trpl::join3(tx_fut, tx_fut2, rx_fut).await;
    })
}
