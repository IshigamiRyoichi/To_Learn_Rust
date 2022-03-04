fn test1(){
    // マクロでベクターを定義
    let mut v = vec![1,2,3,4,5];
    // Vet::new()でベクターを初期化
    let mut v: Vec<i32> = Vec::new();
    // 型を指定してVet::new()で指定
    let mut v = Vec::<i32>::new();
    // 文字列(&str)のベクター
    let mut v: Vec::<&str> = Vec::new();
    // 文字列(&String)のベクター
    let mut v: Vec::<&String> = Vec::new();
    // 実数(f64)のベクター
    let mut v: Vec::<f64> = Vec::new();
}

fn print_i32(a: &[i32]){
    print!("a is ");
    for i in a{
        print!("{}",i);
    }
    println!("");
}

fn print_char(a: &[char]){
    print!("a is ");
    for i in a{
        print!("{}",i);
    }
    println!("");
}

fn print_str(a: &[&str]){
    print!("a is ");
    for i in a{
        print!("{}",i);
    }
    println!("");
}

fn print_total<T>(a: &[T]) where T: std::fmt::Debug{
    print!("a is ");
    for i in a{
        print!("{:?}",i);
    }
    println!("");
}

fn test2(){
    let v =[10,20,30,40,50];
    // print_i32(&v);
    print_total(&v);
    let v = ['A','B','C','D','E'];
    // print_char(&v);
    print_total(&v);
    let v = ["one","two","three","four","five"];
    // print_str(&v);
    print_total(&v);
}

// pub trait Debug{
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
// }

struct Rectange{
    wight: f32,
    height: f32,
}

struct Triangle{
    base: f32,
    height: f32,
}

struct Circle{
    radius: f32
}

trait CalcArea{
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectange{
    fn calc_area(&self) -> f32{
        self.wight * self.height
    }
}

impl CalcArea for Triangle{
    fn calc_area(&self) -> f32{
        self.base * self.height * 0.5
    }
}

impl CalcArea for Circle{
    fn calc_area(&self) -> f32{
        self.radius * self.radius * 3.14
    }
}

trait ExprString{
    fn expr_str(&self) -> String{
        "幅 × 高さ =".to_string()
    }
}

impl ExprString for Rectange{}

impl ExprString for Triangle{
    fn expr_str(&self) -> String{
        "幅 × 高さ ÷ 2 =".to_string()
    }
}

impl ExprString for Circle{
    fn expr_str(&self) -> String{
        "π × 半径 × 半径 =".to_string()
    }
}

trait ToNumber {
    fn to_i(&self) -> i32;
}

impl ToNumber for String{
    fn to_i(&self) -> i32{
        match self.parse::<i32>(){
            Ok(n) => n,
            Err(_) => 0,
        }
    }
}

fn test3(){
    let rect = Rectange{
        wight: 10.0,
        height: 20.0,
    };
    let tri = Triangle{
        base: 10.0,
        height: 20.0,
    };
    let cir = Circle{
        radius: 10.0
    };
    // println!("rect area is {}",rect.calc_area());
    // println!("rect area is {}",tri.calc_area());
    // println!("rect area is {}",cir.calc_area());
    println!("rect {} {}",rect.expr_str(), rect.calc_area());
    println!("tri {} {}",tri.expr_str(), tri.calc_area());
    println!("cir {} {}",cir.expr_str(), cir.calc_area());
}

fn test4(){
    let s = String::from("100");
    let n = s.to_i();
    println!("n is {}",n);
}

fn main() {
    test3();
}
