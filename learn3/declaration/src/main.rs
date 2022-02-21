struct Sample {
    x : i32,
}
impl Sample{
    fn new(x:i32) -> Sample{
        Sample{ x:x }
    }
    fn inc(&self) -> i32{
        self.x + 1
    }
    fn add(&self, x : i32) -> i32{
        self.x + x
    }
}

fn add(x : f64, y : f64) -> f64{
    x + y
}

fn string_length(s : &String) -> usize{
    let length = s.len();
    length
}

fn test(x : i32) -> i32{
    let mut ans = x;
    if x < 0{
        0
    }
    else if x > 100{
        100
    }
    else{
        x
    }
}

fn main() {
    /*
    整数方 浮動小数点型
    */
    // let name : &str = "Ishigami Ryoichi";
    // let age : i32 = 21;
    // let x = 100.234;
    // let mut y : f64 = 100.234;
    // let x = String :: from("Hello");
    // let x = 112;
    // let ans = test(x);
    // println!("ans is {}",ans);
    // {
    //     let x = 76;
    //     println!("x is {}",x); 
    // }
    // let len = string_length(&x);
    // y = 200.123;
    // println!("x is {}",x);
    // println!("y is {}",y);
    // println!("len i  {}",len);
    // println!("y is {}",y);
    // let xy : f64 = add(x,y);
    // println!("x + y is {}",xy);
    // let flag = true;
    // println!("flag is {}",flag);

    /*
    文字列型
    */
    // let cat = "🐈";
    // let dog = "🐕";
    // let s1 = String :: from("Hello");
    // let s2 = String :: from("Rust");
    // let s3 = String :: from("World.");
    // // let s = s1 + " " + &s2 + " " + &s3;
    // let s = format!("{} {} {}",s1,s2,s3);
    // println!("{} and {}",dog,cat);
    // println!("{}",s);

    /*
    複合型
    */
    // let name = "Ishigami Ryoichi";
    // let age = 21;
    // let t = (name,age);
    // println!("name is {}. age is {}",t.0,t.1);

    /*
    配列型
    */
    // let a = ["春","夏","秋","冬"];
    // let i = 0;
    // let j = 3;
    // println!("最初の季節 {}",a[i]);
    // println!("最後の季節 {}",a[j]);


    /*
    構造体
    */
    // let a = Sample::new(10);
    // let ans = a.inc();
    // println!("ans is {}",ans);
    // let ans = a.add(20);
    // println!("ans is {}",ans);

    /*
    クロージャ
    */
    let num = 10;
    let add_one = |x| {num + x};
    let add_two = |x,y| {x + y};

    let ans = add_one(1);
    println!("ans is {}",ans);
    let ans = add_two(10,20);
    println!("ans is {}",ans);
}