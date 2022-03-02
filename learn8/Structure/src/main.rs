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

fn main() {
    test7();
}
