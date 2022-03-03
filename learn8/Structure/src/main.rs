// 構造体
struct Person{
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn test_struct(){
    let mut pe = Person{
        id: 100,
        name: String::from("nasubi"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    println!("{} {} ({}) in {}",pe.id, pe.name, pe.age, pe.addr);
    pe.age += 1;
    println!("{} {} ({}) in {}",pe.id, pe.name, pe.age, pe.addr);
    pe.addr = String::from("Oosaka");
    println!("{} {} ({}) in {}",pe.id, pe.name, pe.age, pe.addr);
}

fn print_Person(pa: &Person){
    println!("{} {} ({}) in {}",pa.id, pa.name, pa.age, pa.addr);
}

fn add_age(pa: &mut Person){
    pa.age += 1;
}

fn new_person(id: i32, name: &str) -> Person{
    let pa = Person{
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknow"),
    };
    pa
}

fn test1(){
    let mut pa = Person{
        id: 100,
        name: String::from("nasubi"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    print_Person(&pa);
    add_age(&mut pa);
    print_Person(&pa);
}

fn test2(){
    let pa2 = new_person(200, "kato");
    print_Person(&pa2);
}

fn test3(){
    let pa1 = new_person(100, "masuda");
    let pa2 = new_person(200, "kato");
    let mut people = vec![pa1, pa2];
    people.push(new_person(200, "yamada"));
    people.push(new_person(200, "sato"));

    for p in people{
        print_Person(&p);
    }
}

fn test4(){
    struct Color(f32,f32,f32);
    let yellow = Color(1.0,1.0,1.0);
    println!("R:{:.2} G:{:.2} B:{:.2}",yellow.0, yellow.1, yellow.2);
}

struct A{
    id: i32,
}

struct B{
    id1: i32,
    id2: i32,
    id3: i32,
}

struct C{
    id: i32,
    name: String,
}

struct D{
    id: i32,
    name: &'static str,
}

struct E{
    id: i32,
    v: Vec<u8>,
}

struct F{
    id: i32,
    v: [u8; 100],
}

fn test5(){
    println!("A size_of is {}",std::mem::size_of::<A>());
    println!("B size_of is {}",std::mem::size_of::<B>());
    println!("C size_of is {}",std::mem::size_of::<C>());
    println!("D size_of is {}",std::mem::size_of::<D>());
    println!("E size_of is {}",std::mem::size_of::<E>());
    println!("F size_of is {}",std::mem::size_of::<F>());
}

static mut PERSON_ID: i32 = 0;

impl Person{
    fn print(&self){
        println!("{} {} ({}) in {}",self.id, self.name, self.age, self.addr);
    }

    fn print_t(&self, private: bool){
        if private == true{
            println!("{}:{}",self.id, self.name);
        } else{
            println!("{} {} ({}) in {}",self.id, self.name, self.age, self.addr);
        }
    }

    fn to_str(&self) -> String{
        let s = format!("{} {} ({}) in {}",self.id, self.name, self.age, self.addr);
        s
    }

    fn add_age(&mut self, n: i32){
        self.age += n;
    }

    fn new(name: &str, age: i32, addr: &str) -> Person{
        let id = unsafe{
            PERSON_ID += 1;
            PERSON_ID
        };
        Person{
            id: id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }
}

fn test6(){
    let mut pa = Person{
        id: 100,
        name: String::from("nasubi"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    // pa.print_t(true);
    // let s = pa.to_str();
    // println!("s is {}",s);    
    pa.print();
    pa.add_age(1);
    pa.print();
}

fn test7(){
    let mut people = Vec::<Person>::new();
    people.push(Person::new("masuda", 50, "Tokyo"));
    people.push(Person::new("kato", 30, "Osaka"));
    people.push(Person::new("yamada", -1, "unknow"));
    people.push(Person::new("sato", -1, "unknow"));
    for p in &people{
        p.print();
    }
}

// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }

fn test8(){
    let r = "xxx".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}",n),
        Err(e) => println!("error: {:?}",e),
    }
}

fn test9(){
    let n = "100".parse::<i32>().unwrap();
    // let n = "xxx".parse::<i32>().unwrap();
    println!("n is {}",n);
}

// fn half_number(a: &str) -> i32{
//     a.parse::<i32>().unwrap() / 2
// }

// fn test10(){
//     // let n: i32 = half_number("100");
//     let n: i32 = half_number("xxx");
//     println!("n is {}",n);
// }

// use std::num::ParseIntError;
// fn half_number(s: &str) -> Result<i32, ParseIntError> {
//     match s.parse::<i32>(){
//         Ok(n) => Ok(n/2),
//         Err(err) => Err(err),
//     }
// }

// fn test11(){
//     // match half_number("100"){
//     match half_number("xxx"){
//         Ok(n) => println!("Ok: {}",n),
//         Err(err) => println!("Err: {:?}", err),
//     }
// }

use std::num::ParseIntError;
// fn half_number(s: &str) -> Result<i32, ParseIntError> {
    // s.parse::<i32>().map(|n| n/2)
    // let n = s.parse::<i32>()?;
    // Ok(n/2)
// }

fn half_number(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>(){
        Ok(n) => Ok(n/2),
        Err(err) => Err("実行エラー:これは数値ではありません"),
    }
}

fn test12(){
    match half_number("100"){
        Ok(n) => println!("Ok: {}",n),
        Err(err) => println!("Err: {:?}", err),
    }
    match half_number("xxx"){
        Ok(n) => println!("Ok: {}",n),
        Err(err) => println!("Err: {:?}", err),
    }
}

fn test13(){
    let n = "100".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}",n);
    let n = "xxx".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}",n);
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let path = "./sample.txt";
    let data = std::fs::read_to_string(path)? ;
    println!("data is {}",data);
    Ok(())
}
