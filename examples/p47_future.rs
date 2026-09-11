// use trpl::trpl;
use trpl::Either;
use trpl::Html;
fn main() {
    println!("Example p47_future");
    // test01();
    test02();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01------------");

    trpl::run(async {
        let (url, title) = page_title("https://www.rust-lang.org/").await;
        println!("url: {}, title: {:?}", url, title);
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|element| element.inner_html());

    (url, title)
}

#[allow(unused)]
fn test02() {
    println!("--------------test02------------");

    trpl::run(async {
        let f1 = page_title("https://www.rust-lang.org/");
        let f2 = page_title("https://www.baidu.com");

        let (url, title) = match trpl::race(f1, f2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("url: {}, title: {:?}", url, title.unwrap());
    })
}
