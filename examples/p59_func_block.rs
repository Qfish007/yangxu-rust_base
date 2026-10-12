fn main() {
    println!("Example p59: Function Block");

    test01();
}

#[allow(unused)]
fn test01() {
    println!("--------------test01--------Function Block----");

    /*
     * 如果一个函数的泛型参数是 T , 是unsized类型
     *  那么该参数只接受指针类型 即 &T
     */

    //  fn test<T: Unsized>(t: &T) {
    //     println!("t: {:?}", t);
    //  }
}
