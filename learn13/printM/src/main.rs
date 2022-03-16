fn test1(){
    println!("Hello rust world!");
    println!("Hello {} world","rust");
}

fn test2(){
    println!("number is {}",10);
    println!("float is {}",10.234);
}

fn test3(){
    println!("tuple is {:?}",("masuda", 20));
}

fn test4(){
    let n = Option::<i32>::Some(10);
    println!("option is {:?}",n.unwrap());
    let n = Option::<i32>::None;
    println!("option is {:?}",n);
}

fn test5(){
    println!("a is {}, b is {}.",100, "tweet");
    println!("a is {a}, b is {b}.",a=100, b="tweet");
}

fn test6(){
    let n = 10;
    println!("10進数 {}", n);
    println!("16進数 {:x}", n);
    println!("16進数 {:X}", n);
    println!("8進数 {:o}", n);
    println!("2進数 {:b}", n);
}

fn test7(){
    let n = 10;
    println!("通常 {}", n);
    println!("桁揃え {:4}", n);
    println!("10進数 {:04}", n);
    println!("16進数 {:02X}", n);
    println!("2進数 {:08b}", n);
}

fn test8(){
    let f = 10.234;
    println!("{} ",f);
    println!("{:e} ",f);
    println!("{:E} ",f);
    println!("{:.2} ",f);
    let f = 0.0234;
    println!("{} ",f);
    println!("{:e} ",f);
    println!("{:E} ",f);
    println!("{:.2} ",f);
}

fn test9(){
    println!("hello, `{:8}` world", "rust");
    println!("hello, `{:<8}` world", "rust");
    println!("hello, `{:>8}` world", "rust");
    println!("hello, `{:^8}` world", "rust");
    println!("hello, `{:8}` world", 123);
    println!("hello, `{:<8}` world", 123);
    println!("hello, `{:>8}` world", 123);
    println!("hello, `{:^8}` world", 123);
}

fn test10(){
    let a = [1,2,3,4,5];
    // println!("a is {:?}",a);
    dbg!(a);
}

#[derive(Debug)]
struct Person{
    id :i32,
    name: String,
    age: i32,
    addr: String,
}

fn test11(){
    let p = Person{
        id: 100,
        name: String::from("yakiniku"),
        age: 50,
        addr: String::from("Kyoto"),
    };
    dbg!(p);
    // println!("p is {:?}",p);
}

fn test12(){
    panic!("このプログラムは動きません");
}

fn test13(){
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1{
        panic!("パラメータは必須です。");
    } else {
        for (i, s) in args.iter().enumerate(){
            println!("{}: {}",i,s);
        }
    }
}

use std::path::Path;
fn test14(){
    let path = "unknown.txt";
    if Path::new(path).exists(){
        println!("ファイルが存在する");
    } else {
        panic!("ファイルが存在しない");
    }
}

fn main() {
    test12();
}
