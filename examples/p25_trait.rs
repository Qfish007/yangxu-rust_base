use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::Display;
// Rust 中 Clone 是一个 trait，需要显式导入
fn main() {
    println!("Example p25: trait");

    test01();
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("read more from {}", self.summarize_author()).to_string()
    }

    fn summarize_author(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location).to_string()
    }
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content).to_string()
    }

    fn summarize_author(&self) -> String {
        self.username.clone()
    }
}

// &impl Summary 表示 item 是一个实现 Summary trait 的引用
pub fn notify(item: &impl Summary) {
    println!("notify: {}", item.summarize());
}
#[allow(unused)]
fn test01() {
    let news = NewsArticle {
        headline: String::from("Rust is a great language"),
        location: String::from("Rust city"),
        author: String::from("Rust author"),
        content: String::from("Rust is a great language"),
    };
    println!("{}", news.summarize());

    let tweet = Tweet {
        username: String::from("Rust author"),
        content: String::from("Rust is a great language"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());

    notify(&news);
    notify(&tweet);
}

#[allow(unused)]
fn test02() {
    // trait bound 表示 item 是一个实现 Summary trait 的引用
    pub fn notify2<T: Summary>(item: &T) {
        println!("notify: {}", item.summarize());
    }

    #[allow(unused)]
    fn some_function1<T: Display + Clone + Summary, U: Clone + Debug>(item: &T, item2: &U) {
        println!("some_function: {}", item.summarize());
    }

    #[allow(unused)]
    fn some_function2<T, U>(item: &T, item2: &U)
    where
        T: Display + Clone + Summary,
        U: Clone + Debug,
    {
        println!("some_function: {}", item.summarize());
    }
}

#[allow(unused)]
fn test03() {
    // 返回多类型
    #[allow(unused)]
    fn get_summary1() -> impl Summary {
        NewsArticle {
            headline: String::from("Rust is a great language"),
            location: String::from("Rust city"),
            author: String::from("Rust author"),
            content: String::from("Rust is a great language"),
        }
    }
    #[allow(unused)]
    fn get_summary2(a: i32) -> Box<dyn Summary> {
        if a > 10 {
            Box::new(NewsArticle {
                headline: String::from("Rust is a great language"),
                location: String::from("Rust city"),
                author: String::from("Rust author"),
                content: String::from("Rust is a great language"),
            })
        } else {
            Box::new(Tweet {
                username: String::from("Rust author"),
                content: String::from("Rust is a great language"),
                reply: false,
                retweet: false,
            })
        }
    }
}

/*
Example p25: trait bound

blanket实现

/// 下面的意思是：如果 T 实现了 Display trait，那么 T 就可以调用 to_string 方法了
impl<T:Display> ToString for T {
    fn to_string(&self) -> String {

    }
}
 */
