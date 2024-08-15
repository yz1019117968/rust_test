// use crate::List::{Cons, Nil};
// 一种叫cons list的数据结构，成员包含当前项的值以及下一项

use std::ptr::eq;
use std::ops::Deref;

enum List {
    // 指向List枚举类型的变量的指针，因此其存储空间是确定的
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

// 针对泛型<T>来实现new方法
impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

// 针对i32类型实现new1方法
impl MyBox<i32> {
    fn new1(x: i32) {

    }
}

// 为MyBox实现Deref trait, 所以经由MyBox初始化的变量可以用*解引用
impl<T> Deref for MyBox<T> {
    // 暂时不知道啥意思
    type Target = T;
    fn deref(&self) -> &T {
        // 返回元组的第一个元素的引用
        &self.0
    }
}
fn main() {
    let list = List::Cons(1, 
            Box::new(List::Cons(2, 
            Box::new(List::Cons(3, 
            Box::new(List::Nil))))));
    let x = 5;
    // 
    let y = Box::new(x);

    assert_eq!(x, *y)
    // *y等价于*(y.deref())
}

