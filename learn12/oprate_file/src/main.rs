use std::fs;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::env;

fn test1(){
    let path = "unknown.txt";
    println!("read all lines");
    match fs::read_to_string(path){
        Ok(data) => {
            println!("data is {}",data);
        }
        _ => {
            println!("cannot open {}",path);
        }
    }
}

fn test2(){
    let path = "sample.txt";
    println!("read all lines by buffers");
    let mut file = File::open(path).expect("file not found.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("read error.");
    println!("data is {}",data);
}

fn test3(){
    let path = "sample.txt";
    println!("read all lines by buffers");
    if let Ok(mut file) = File::open(path){
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data){
            println!("data is {}", data);
        }
    }
}

fn test4(){
    let path = "sample.txt";
    println!("read 16 bytes by buffers");
    let mut file = File::open(path).expect("file not found.");
    let mut buf : [u8; 1] = [0; 1];
    for i in 0..16{
        file.read(&mut buf);
        println!("buf is {}:{}",i,buf[0] as char);
    }
}

fn test5(){
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found.");
    writeln!(file, "hello rust world.").expect("cannot write.");
}

fn test6(){
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found.");
    file.write(b"hello rust world.\n").expect("cannot write");
}

fn test7(){
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found.");
    let s = "hello rust world";
    file.write(s.as_bytes()).expect("cannot write");
}

fn test8(){
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found.");
    let s = "hello rust world";
    for it in s.as_bytes(){
        file.write(&[*it]).expect("cannot write.");
    }
}

fn test9(){
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found.");
    let s = "hello rust world";
    for it in s.as_bytes(){
        let ch = *it;
        let ary = [ch];
        file.write(&ary).expect("cannot write.");
    }
}

fn test10() -> std::io::Result<()>{
    let path = "sample.txt";
    let mut file = File::create(path)? ;
    file.write(b"hello rust world.\n")? ;
    Ok(())
}

// fn main(){
//     let args = env::args().collect::<Vec<String>>();
//     if args.len() <= 1{
//         let reader = BufReader::new(std::io::stdin());
//         let mut writer = BufWriter::new(std::io::stdout());
//         for it in reader.bytes(){
//             writer.write(&[it.unwrap()]);
//         }
//     } else {
//         let file = File::open(&args[1]).expect("file not found.");
//         let reader = BufReader::new(file);
//         let mut writer = BufWriter::new(std::io::stdout());
//         for it in reader.bytes(){
//             writer.write(&[it.unwrap()]);
//         }
//     }
// }

fn do_print<R>( reader: BufReader<R> )where R: std::io::Read{
    let mut writer = BufWriter::new(std::io::stdout());
    for it in reader.bytes(){
        writer.write(&[it.unwrap()]);
    }
}

fn main(){
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1{
        let reader = BufReader::new(std::io::stdin());
        do_print(reader);
    } else {
        let file = File::open( &args[1] ).expect("file not found.");
        let reader = BufReader::new(file);
        do_print(reader);
    }
}