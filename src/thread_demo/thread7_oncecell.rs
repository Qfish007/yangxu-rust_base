use std::{
    assert_eq,
    cell::OnceCell,
    sync::{
        atomic::{AtomicU32, Ordering},
        OnceLock,
    },
    thread,
};

pub fn main() {
    test04();
}

fn test01() {
    println!("--------------OnceCell------------");

    let cell = OnceCell::new();
    let result = cell.set(1);
    assert!(result.is_ok());

    let result = cell.set(2);
    assert!(result.is_err());

    let cell2 = OnceCell::new();
    let value = cell2.get_or_init(|| "qfish".to_string());
    assert_eq!("qfish", value);
    assert!(cell2.get().is_some());

    let mut cell3 = OnceCell::new();
    let _ = cell3.set("yangxu".to_string());

    // OnceCell 只能设置一次,
    // mut cell3 为了获得 get_mut() 方法,获取可变引用, 相当于通过指针修改值。 set不能在设置值了
    if let Some(value) = cell3.get_mut() {
        value.push_str(" is a good");
        assert_eq!("yangxu is a good", value);
    }

    let _ = cell3.set("yangxu is a bad person".to_string()); // 设置失败
    if let Some(value) = cell3.get() {
        //   value.push_str(" is a bad person");
        assert_ne!("yangxu is a bad person", value);
    }
}

static LOCK: OnceLock<usize> = OnceLock::new();
fn test02() {
    println!("--------------OnceLock------------");

    unsafe {
        LOCK.get().unwrap();
    }

    thread::spawn(|| unsafe {
        let value = LOCK.get_or_init(|| 1);
        assert_eq!(1, *value);
    })
    .join()
    .unwrap();

    unsafe {
        assert_eq!(1, *LOCK.get().unwrap());
    }
}

#[derive(Debug)]
struct OnceList<T> {
    data: OnceLock<T>,                // 数据
    next: OnceLock<Box<OnceList<T>>>, // 下一个节点
}

impl<T> OnceList<T> {
    const fn new() -> Self {
        Self {
            data: OnceLock::new(),
            next: OnceLock::new(),
        }
    }

    fn push(&self, value: T) {
        if let Err(value) = self.data.set(value) {
            // 往data设置值, 如果失败, 说明当前data已经设置过了
            let next = self.next.get_or_init(|| Box::new(Self::new())); // 如果next为空, 则创建一个新的节点
            next.push(value); //
        }
    }

    fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.data
            .get()
            .map(|item| item == value)
            .filter(|v| *v)
            .unwrap_or_else(|| {
                self.next
                    .get()
                    .map(|next| next.contains(value))
                    .unwrap_or(false)
            })
    }

    /// 查找某个值，返回匹配节点的 &mut OnceList<T>
    fn find_mut(&mut self, value: &T) -> Option<&mut OnceList<T>>
    where
        T: PartialEq,
    {
        // 用 get() 只读比较，避免持有可变借用导致 return Some(self) 时冲突
        if self.data.get() == Some(value) {
            return Some(self);
        }
        // 否则递归往 next 找
        self.next.get_mut().and_then(|next| next.find_mut(value))
    }
}

// static LIST: OnceList<u32> = OnceList::new();
static COUNTER: AtomicU32 = AtomicU32::new(0);
const LEN: u32 = 5;

fn test03() {
    println!("--------------OnceLock------------");

    let mut list = OnceList::new();

    thread::scope(|scope| {
        for i in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| {
                while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed) {
                    list.push(i);
                }
            });
        }
    });

    for i in 0..LEN {
        assert!(list.contains(&i));
    }

    // 查找值为3的节点
    // find_mut 返回 &mut OnceList，可以调用 get_mut 修改值
    if let Some(node) = list.find_mut(&3) {
        // find_mut 返回 &mut OnceList，可以调用 get_mut 修改值
        let value = node.data.get_mut().unwrap();
        *value = 100;
        println!("found node: {:?}", value);
    }
    println!("{:?}", list);
}

fn test04() {
    println!("--------------OnceLock------------");

    let m = Some(50);

    // if m.map_or(false, |v| v > 50) {
    //     println!(">50value: {:?}", m);
    // } else {
    //     println!(" <50 value: {:?}", m);
    // }

    if m.is_some_and(|v| v > 5) {
        println!(">5 value: {:?}", m);
    } else {
        println!(" <5 value: {:?}", m);
    }
}
