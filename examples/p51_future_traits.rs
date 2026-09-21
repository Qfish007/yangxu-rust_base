fn main() {
    println!("Example p51: Future Traits 介绍概念");
}

/*
Future Traits 介绍概念

Pin     是一个结构体类型，存放指针
pin!    宏，将一个 Future 包裹在 Pin 中，使其可以安全移动
Unpin   标记trait, 可以安全移动
!Unpin  标记trait, 不能安全移动

1. trpl::join!(tx_fut, tx_fut2, rx_fut).await;
   这样写内部会隐式的固定future，不能移动future


2.let list = vec![Box::pin(tx_fut), Box::pin(tx_fut2), Box::pin(rx_fut)];
  trpl::join_all(list).await;

  join_all 要求集合中的项都实现 Future trait，而只有当Box所有包装的T是一个实现了Unpin trait时，Box<T>才实现Future
  而future 都没有实现Unpin trait，所以不能移动future，因此需要手动将Box<T>用pin包裹起来

  Pin<Box<Vec<String>>>
  Pin<Box<Box<Vec<String>>>>
 */
