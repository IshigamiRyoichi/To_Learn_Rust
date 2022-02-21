fn test_fn1(){
    let a = 10;
    let b = 20;
    println!("a is {}, b is {}",a,b);
}

fn test_fn2(){
    let a = {10 + 20};
    println!("a is {}",a);
}

fn add(x:i32, y:i32) -> i32{
    x + y
}

fn plus(x:i32) -> bool{
    x > 0
}

fn test_fn3(){
    let a = 10;
    if plus(a){
        println!("a is {}",a);
    }
}

fn test_fn4(){
    let a = 10;
    let b = 3;
    println!("a/b is {}",a/b);
    let a = 10.0;
    let b = 3.0;
    println!("a/b is {}",a/b);
}

fn test_fn5(){
    let mut a = 10;
    a += 20;
    println!("a is {}",a);
    let mut sum = 0;
    for i in 0..10{
        sum += i;
    }
    println!("sum is {}",sum);
}

fn test_fn6(){
    let a : u8 = 0b1111;
    let b : u8 = 0b0011;
    println!("a & b is {:04b}",a & b);
    println!("a | b is {:04b}",a | b);
}

fn test_fn7(){
    let a : u8 = 0x02;
    println!("a is {}",a);
    println!("a << 1 is {}", a << 1);
    println!("a >> 1 is {}", a >> 1);
}

fn test_fn8(){
    let a = true;
    let b = !a;
    println!("a is {}. b is {}.",a,b);
    println!("a && b is {}", a && b);
    println!("a || b is {}", a || b);
}

fn str_param_and_return(s : &str) -> String{
    println!("called str_param_and_return, s is {}",s);
    let ret = format!("hello {} world.",s);
    ret
}

fn test_fn9(){
    let ret = str_param_and_return("rust");
    println!("ret is {}",ret);
}

fn main() {
    test_fn9();
}
