fn main() {
    println!("Example p24: 泛型");

    let number_list = Vec::from([34, 50, 25, 100, 65]);
    let result = largetst(&number_list);
    println!("result: {}", result);

    let char_list = Vec::from(['y', 'm', 'a', 'q']);
    let result = largetst(&char_list);
    println!("result: {}", result);
}

// 泛型函数，返回列表中的最大值
fn largetst<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<i32> {
    fn p1(&self) -> &i32 {
        &self.x
    }
}

#[allow(unused)]
fn test01() {
    let p = Point { x: 10, y: 20 };
    let p1 = p.p1();
    println!("p1: {}", p1);

    let p2 = p.y();
    println!("p2: {}", p2);

    let p3 = p.x();
    println!("p3: {}", p3);
}
