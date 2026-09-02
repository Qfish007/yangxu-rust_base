// use std::print;

fn main() {
    println!("Example p31: Iterator");

    // test01();
    test02();
}

#[allow(unused)]
fn test01() {
    let mut opt = Vec::from([1, 2, 3]);
    let iter = opt.iter();
    for i in iter {
        println!("{}", i);
    }

    let mut iter_mut = opt.iter_mut();
    for i in iter_mut {
        *i *= 100;
    }
    println!("{:?}", opt);

    let mut iter = opt.iter();
    assert_eq!(iter.next(), Some(&100));
}

#[allow(unused)]
fn test02() {
    let mut opt = Vec::from([1, 2, 3]);

    let iter = opt.iter_mut();
    for i in iter {
        *i *= 100;
    }

    println!("{:?}", opt);

    /*
    let iter = opt.iter_mut();
    opt.iter_mut() 返回一个可变迭代器, 意思是内部每个元素都是 &mut T 类型的引用, 所以可以通过解引用来修改元素的值。

    let iter = opt.iter();
    opt.iter() 返回一个不可变迭代器, 意思是内部每个元素都是 &T 类型的引用, 所以不能通过解引用来修改元素的值

    let mut iter = opt.iter_mut();
    mut iter 代表这个iter 可以被修改;
    类似 let mut x = 5;  x 可以被修改, 但是 iter 本身是一个迭代器, 迭代器本身是一个对象, 这个对象可以被修改, 但是这个对象内部的元素是 &mut T 类型的引用, 所以可以通过解引用来修改元素的值

    */
}
