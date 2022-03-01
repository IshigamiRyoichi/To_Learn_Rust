fn test1(){
    // let ary = [1,2,3,4,5];
    // let ary = ["one","two","three","four","five"];
    let ary : [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
    println!("ary[0] is {}",ary[0]);
    println!("ary[4] is {}",ary[4]);
    println!("ary.len is {}",ary.len());
}

fn test2(){
    let mut ary: [u8; 16] = [0; 16];
    println!("ary[0] is {}",ary[0]);
    println!("ary[15] is {}",ary[15]);
    ary[0] = 10;
    println!("ary[0] is {}",ary[0]);
    println!("ary[15] is {}",ary[15]);
}

fn test3(){
    let a = [1u8, 2u8, 3u8, 4u8];
    unsafe {
        let b = std::mem::transmute::<[u8; 4], u32>(a);
        println!("b is {:x}",b);
    }
    let a:u32 = 0x11223344;
    unsafe{
        let b = std::mem::transmute::<u32, [u8; 4]>(a);
        println!("b[] is {:x} {:x} {:x} {:x}",b[0],b[1],b[2],b[3]);
    }
}

fn test4(){
    let v = vec![1,2,3,4,5];
    // println!("v[0] is {}",v[0]);
    // println!("v[4] is {}",v[4]);
    // println!("v.len() is {}",v.len());
    println!("v.get(0) is {:?}",v.get(0));
    println!("v.get(4) is {:?}",v.get(4));
    println!("v.get(0) is {:?}",v.get(0).unwrap());
    println!("v.get(4) is {:?}",v.get(4).unwrap());
}

fn test5(){
    let mut v: Vec<i32> = Vec::new();
    println!("v.len() is {}",v.len());
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v.len is {}",v.len());
    println!("pop is {:?}",v.pop());
    println!("pop is {:?}",v.pop());
    println!("pop is {:?}",v.pop());
    println!("v.len is {}",v.len());
}

fn test6(){
    let mut v = vec![1,2,3,4,5];
    // println!("v.first is {:?}",v.first());
    // println!("v.last is {:?}",v.last());
    // println!("v.get(1) is {:?}",v.get(1));
    // println!("v.get(10) is {:?}",v.get(10));
    // println!("v.first is {:?}",v.first().unwrap());
    // println!("v.last is {:?}",v.last().unwrap());
    println!("v.first is {:?}", v.first());
    println!("v.remove(0) is {:?}", v.remove(0));
    println!("v.first is {:?}", v.first());
}

fn test7(){
    let s = "one,two,three,four,five";
    let v = s.split(",");
    for x in v{
        print!("{} ",x);
    }
    println!("");
}

fn test8(){
    let mut v = vec!["one","two","three","four","five"];
    v.sort();
    let x = v.join(" ");
    println!("x is {}",x);
    v.reverse();
    let x = v.join(" ");
    println!("x is {}",x);
}

fn test9(){
    let v = vec![1,2,3,4,5];
    print!("for is ");
    for i in &v{
        print!("{} ",i);
    }
    println!("");
    print!("for and iter is ");
    for i in v.iter(){
        print!("{} ",i);
    }
    println!("");
}

fn test10(){
    let v = vec![1,2,3,4,5];
    let mut p = v.iter();
    println!("p is {:?}",p);
    println!("p is {:?}",p.next());
    println!("p is {:?}",p.next());
    println!("p is {:?}",p.next());
    println!("p is {:?}",p.next());
    println!("p is {:?}",p.next());
    println!("p is {:?}",p.next());
}

fn test11(){
    let v = vec![1,2,3,4,5];
    println!("by loop");
    let mut p = v.iter();
    loop{
        let x = p.next();
        if x == None{
            break;
        }
        println!("x is {}",x.unwrap());
    }
    println!("by while");
    let mut p = v.iter();
    while let Some(x) = p.next(){
        println!("x is {}",x);
    }
}

fn test12(){
    let v = vec![1,2,3,4,5];
    let lst = v.iter().map(|x| x * 10);
    for i in lst{
        println!("i is {}",i);
    }
}

fn main() {
    test12();
}
