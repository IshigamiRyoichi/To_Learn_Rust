fn test_fn1(){
    let a = 30;
    let b = 20;
    if a == b{
        println!("a == b is ok.");
    } else if a < b{
        println!("a < b is ok.");
    } else {
        println!("a < b is ng.");
    }
}

fn flag(x:i32, y:i32) -> bool{
    x < y
}

fn test_fn2(){
    let x = 10;
    let y = 20;
    if flag(x,y){
        println!("x < y is OK.");
    }
}

fn test_fn3(){
    let a = 10;
    let b = 20;
    let x = if a < b {1} else {2};
    println!("x is {}",x); 
}

fn test_fn4(){
    let v = vec![10,20,30,40,50];
    print!("v is ");
    for i in &v{
        print!("{} ",i);
    }
    println!("");
}

fn test_fn5(){
    let v = vec![10,20,30,40,50];
    // print!("v is ");
    for (i,x) in v.iter().enumerate(){
        println!("{}:{}",i,x);
    }
    println!("");
}

fn test_fn6(){
    for i in 0..10{
        if i == 5{
            break;
        }
        println!("{}",i);
    }
}

fn test_fn7(){
    for i in 0..10{
        if i % 2 == 0{
            continue;
        }
        println!("{} ",i);
    }
}

fn test_fn8(){
    let mut i = 0;
    while i < 10{
        println!("{}",i);
        i += 2;
    }
}

fn test_fn9(){
    let mut i = 0;
    loop{
        if i >= 10{
            break;
        }
        println!("{}",i);
        i += 1;
    }
}

#[derive(Debug)]
enum LANG{
    JAPANESE = 81,
    ENGLISH = 44,
    CHINESE = 86,
    FRANCH = 33,
}
fn test_fn10(){
    let lang = LANG::JAPANESE;
    println!("lang is {}", lang as i32);
}
fn test_fn11(){
    let lang = LANG::JAPANESE;
    let m = match lang{
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        LANG::FRANCH => "フランス語",
    };
    println!("lang is {}",m);
}
fn test_fn12(){
    let lang = LANG::ENGLISH;
    let m = match lang{
        LANG::JAPANESE => "日本語だよ!",
        _ => "日本語じゃないよ",
    };
    println!("lang is {}",m);
}

enum Opotion<T>{
    Some(T),
    None,
}

fn test_fn13(){
    let x = None;
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("v is {}",v);
}

fn main() {
    test_fn13();
}
