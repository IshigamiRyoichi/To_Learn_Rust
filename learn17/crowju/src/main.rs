// map
fn test1() {
    println!("use map");
    let a = [1,2,3,4,5];
    let b = a.iter().map(|x| x*x);
    for it in b{
        println!("it is {}",it);
    }
}

// filter
fn test2() {
    println!("use filter");
    let a = [1,2,3,4,5];
    let b = a.iter().filter(|&x| x % 2 == 1);
    for it in b {
        println!("it is {}", it);
    }
}

// find
fn test3() {
    println!("use find");
    let a = [1,2,3,4,5];
    let b = a.iter().find(|&&x| x == 3);
    let c = a.iter().find(|&&x| x > 10);
    println!("b is {:?}",b);
    println!("c is {:?}",c);
}

// max & min
fn test4() {
    println!("use max and min");
    // let a = [1,2,3,4,5];
    let a: [i32;0] = [];
    let max = a.iter().max();
    let min = a.iter().min();
    println!("max is {:?}",max);
    println!("min is {:?}",min);
}

// zip
fn test5() {
    println!("use zip");
    let a = [1,2,3,4,5];
    let b = ["one", "two", "three", "four", "five"];
    let c: Vec<_> = a.iter().zip(b.iter()).collect();
    for (n,w) in c {
        println!("c is {} and {}",n ,w);
    }
}

// クロージュ
fn test6() {
    // クロージュの定義
    let print_name = |name, age| {
        println!("name: {}, age: {}",name ,age);
    };
    // クロージュ呼び出し
    println!("call closure");
    print_name("Ishigami", 21);
}

fn test7() {
    println!("use return of closure");
    let format_name = |name, age| {
        format!("name: {}, age: {}",name, age)
    };
    let x = format_name("Ishigami", 21);
    println!("x is {}", x);
}

fn test8() {
    // mapにクロージュ
    println!("use map");
    let a = [("masuda", 50), ("kato", 40), ("yasuda", 30)];
    let b = a.iter().map(|(name, age)|{
        format!("name: {}, age: {}", name ,age)
    });
    for it in b {
        println!("{}", it);
    }
}

fn call_with_one<F>(x: usize, func: F) -> usize
    where F: Fn(usize) -> usize{
    func(x)
}

fn test9() {
    let double = |x| x * 2;
    let triple = |x| x * 3;
    let a = call_with_one(100, double);
    let b = call_with_one(100, triple);
    println!("a is {}",a);
    println!("b is {}",b);
}

fn call_with_vec<F>(v: &Vec::<usize>, func: F) -> usize
where F: Fn(usize) -> usize {
    let mut sum = 0;
    for it in v {
        sum += func(*it);
    }
    sum
}

fn test10() {
    let double = |x| x * 2;
    let v = vec![1,2,3,4,5];
    let a = call_with_vec(&v, double);
    println!("a is {}",a);
    let sum: usize = v.iter().map(|x| x * 2).sum();
    println!("sum is {}", sum);
}

fn main() {
    test10();
}
