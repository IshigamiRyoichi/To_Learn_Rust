fn test_string1(){
    let ch = 'A';
    println!("ch is {}",ch);
    let u = ch as u8;
    println!("u is {}",u);
    let ch = u as char;
    println!("ch is {}",ch);
}

fn test_string2(){
    let s = "hello rust world.";
    let len = s.len();
    println!("s.len is {}.",len);
    let hello = &s[0..5];
    let world = &s[11..];
    println!("hello is {}.",hello);
    println!("world is {}.",world);
}

fn test_string3(){
    //空の文字列
    let mut s = String::new();
    s.push_str("hello ");
    s.push_str("rust ");
    s.push_str("world.");
    println!("s is {}",s);
}

fn test_string4(){
    let hello = "HELLO";
    let rust = "RUST";
    let world = "WORLD.";
    let s = format!("{} {} {}",hello,rust,world);
    println!("s is {}",s);
}

fn test_string5(){
    let s = "こんにちは rust コードの世界";
    let hello = &s[0..15];
    let world = &s[21..];
    println!("こんにちは is {}",hello);
    println!("コードの世界 is {}",world);
    println!("s.lem is {}",s.len());
}

fn test_string6(){
    let mut s = String::new();
    s.push_str("こんにちは ");
    s.push_str("rust ");
    s.push_str("コードの世界");
    println!("s is {}",s);
}

fn test_string7(){
    let s = "This is ねこ🐈neko 文字列";
    let mut v : Vec<char> = Vec::new();
    for c in s.chars(){
        v.push(c);
    }
    let v = &v[8..15];
    let mut s = String::new();
    for c in v{
        s.push(*c);
    }
    println!("s is {}",s);
}

fn test_string8(){
    let s = "hello rust world.";
    let a = &s[0..1];
    println!("a is {}",a);
    let a = &s[0..5];
    println!("a is {}",a);
    let a = &s[..5];
    println!("a is {}",a);
}

fn test_string9(){
    let s = "hello rust world.";
    let a = &s[6..10];
    println!("a is {}",a);
    let a = &s[6..(6+4)];
    println!("a is {}",a);
}

fn test_string10(){
    let s = "hello rust world.";
    let a = &s[..];
    println!("a is {}",a);
    let n = s.len();
    let a = &s[0..n];
    println!("a is {}",a);
}

fn main() {
    test_string10();
}
