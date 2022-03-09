#[derive(Debug)]
struct Person{
    name: &'static str,
    age: i32,
}

fn print_a(a: &Person){
    println!("print_a: a is {:?}",a);
}

fn move_a(a: Person){
    println!("move_a: a is {:?}",a);
}

fn add_age(a: &mut Person){
    a.age += 1;
}

fn test1(){
    let mut a = Person{name:"yakiniku", age:50};
    let mut x = &mut a;
    // print_a(&a);
    // move_a(a);
    println!("x is {:?}",x);
    x.age += 1;
    println!("x is {:?}",x);
    add_age(&mut x);
    println!("x is {:?}",x);
}

fn test2(){
    let a = (100,"しゃぶしゃぶ");
    println!("a is {:?}",a);
    let x = a;
    println!("x is {:?}",x);
    let y = a;
    println!("y is {:?}",y);
}

fn test3(){
    let a = vec!["one","two","three"];
    println!("a[] is {:?}", a);
    let x = &a;
    println!("x[] is {:?}", x);
    println!("a[] is {:?}", a);
}

#[derive(Debug)]
struct Person2{
    name: String,
    age: i32,
}

fn test4(){
    let x: Person2;
    {
        let a = Person2{
            name: String::from("yakiniku"),
            age: 50,
        };
        x = a;
    };
    println!("x is {:?}",x);
}

fn new_person(name: &str, age: i32) -> Person2{
    let p = Person2{
        name: String::from(name),
        age: age,
    };
    p
}

fn test5(){
    let a = new_person("しゃぶしゃぶ",50);
    println!("a is {:?}",a);
}

fn test6(){
    let mut a = Person2{
        name: String::from("焼肉"),
        age: 50,
    };
    println!("a is {:?}",a);
    let mut x = &mut a;
    x.age = 0;
    println!("x is {:?}",x);
    let mut y = &mut a;
    y.name = String::from("しゃぶしゃぶ");
    println!("y is {:?}", y);
    println!("a is {:?}", a);
}

fn main() {
    test6();
}
